//! In-process tests of the SBF build of `vericode_escrow`.
//!
//! The program is loaded from `SBF_OUT_DIR` by `solana-program-test`; no
//! validator, cluster, RPC or keypair file is used. Every rejected
//! instruction is checked for an unchanged Job, vault and token balances.

use anchor_lang::solana_program::instruction::{Instruction, InstructionError};
use anchor_lang::solana_program::program_pack::Pack;
use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_lang::{AccountDeserialize, InstructionData, ToAccountMetas};
use anchor_spl::token::spl_token;
use solana_keypair::Keypair;
use solana_program_test::{tokio, BanksClientError, ProgramTest, ProgramTestContext};
use solana_signer::Signer;
use solana_system_interface::instruction as system_instruction;
use solana_system_interface::program as system_program;
use solana_transaction::Transaction;
use solana_transaction_error::TransactionError;
use vericode_escrow::{
    accounts, instruction, EscrowStatus, JobAccount, VericodeEscrowError, JOB_SEED,
    JOB_ACCOUNT_VERSION, VAULT_SEED,
};

const PROGRAM_NAME: &str = "vericode_escrow";
const DECIMALS: u8 = 6;
const AMOUNT: u64 = 1_000_000;
const BUYER_START_BALANCE: u64 = 5_000_000;
const DEADLINE: u64 = 1_000;
const JOB_ID: [u8; 32] = [0x11; 32];
// Final D1c2b guest ImageID, stored here only as a Job commitment.
const IMAGE_ID_HEX: &str = "4da06f90da75ec8980c943ce017d69c48370fddbf3aa27689d375d78fac0fb1a";

struct Env {
    ctx: ProgramTestContext,
    buyer: Keypair,
    executor: Keypair,
    mint: Pubkey,
    mint_authority: Keypair,
    buyer_token: Pubkey,
}

fn hex32(hex: &str) -> [u8; 32] {
    assert_eq!(hex.len(), 64);
    let mut out = [0_u8; 32];
    for (index, byte) in out.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&hex[index * 2..index * 2 + 2], 16).unwrap();
    }
    out
}

fn spec_hash() -> [u8; 32] {
    vericode_core::hash_restricted_spec(&vericode_core::RESTRICTED_SPEC_V1)
        .unwrap()
        .into_bytes()
}

fn harness_hash() -> [u8; 32] {
    vericode_core::hash_harness_version(vericode_core::DETERMINISTIC_HARNESS_VERSION)
        .unwrap()
        .into_bytes()
}

fn job_pda(job_id: &[u8; 32]) -> Pubkey {
    Pubkey::find_program_address(&[JOB_SEED, job_id], &vericode_escrow::ID).0
}

fn vault_pda(job: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[VAULT_SEED, job.as_ref()], &vericode_escrow::ID).0
}

fn custom(error: VericodeEscrowError) -> u32 {
    u32::from(error)
}

async fn send(
    ctx: &mut ProgramTestContext,
    instructions: &[Instruction],
    signers: &[&Keypair],
) -> Result<(), BanksClientError> {
    let blockhash = ctx.get_new_latest_blockhash().await.unwrap();
    let mut all_signers: Vec<&Keypair> = vec![&ctx.payer];
    all_signers.extend_from_slice(signers);
    let transaction = Transaction::new_signed_with_payer(
        instructions,
        Some(&ctx.payer.pubkey()),
        &all_signers,
        blockhash,
    );
    ctx.banks_client.process_transaction(transaction).await
}

fn assert_custom(result: Result<(), BanksClientError>, expected: u32) {
    match result {
        Err(BanksClientError::TransactionError(TransactionError::InstructionError(
            _,
            InstructionError::Custom(code),
        ))) => assert_eq!(code, expected, "unexpected custom error code"),
        other => panic!("expected custom error {expected}, got {other:?}"),
    }
}

async fn transfer_lamports(ctx: &mut ProgramTestContext, to: &Pubkey, lamports: u64) {
    let ix = system_instruction::transfer(&ctx.payer.pubkey(), to, lamports);
    send(ctx, &[ix], &[]).await.unwrap();
}

async fn create_mint(ctx: &mut ProgramTestContext, authority: &Pubkey) -> Pubkey {
    let mint = Keypair::new();
    let rent = ctx.banks_client.get_rent().await.unwrap();
    let create = system_instruction::create_account(
        &ctx.payer.pubkey(),
        &mint.pubkey(),
        rent.minimum_balance(spl_token::state::Mint::LEN),
        spl_token::state::Mint::LEN as u64,
        &spl_token::ID,
    );
    let init =
        spl_token::instruction::initialize_mint2(&spl_token::ID, &mint.pubkey(), authority, None, DECIMALS)
            .unwrap();
    send(ctx, &[create, init], &[&mint]).await.unwrap();
    mint.pubkey()
}

async fn create_token_account(ctx: &mut ProgramTestContext, mint: &Pubkey, owner: &Pubkey) -> Pubkey {
    let account = Keypair::new();
    let rent = ctx.banks_client.get_rent().await.unwrap();
    let create = system_instruction::create_account(
        &ctx.payer.pubkey(),
        &account.pubkey(),
        rent.minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN as u64,
        &spl_token::ID,
    );
    let init =
        spl_token::instruction::initialize_account3(&spl_token::ID, &account.pubkey(), mint, owner)
            .unwrap();
    send(ctx, &[create, init], &[&account]).await.unwrap();
    account.pubkey()
}

async fn mint_to(ctx: &mut ProgramTestContext, mint: &Pubkey, to: &Pubkey, authority: &Keypair, amount: u64) {
    let ix =
        spl_token::instruction::mint_to(&spl_token::ID, mint, to, &authority.pubkey(), &[], amount).unwrap();
    send(ctx, &[ix], &[authority]).await.unwrap();
}

async fn token_balance(ctx: &mut ProgramTestContext, account: &Pubkey) -> u64 {
    let data = ctx.banks_client.get_account(*account).await.unwrap().unwrap().data;
    spl_token::state::Account::unpack(&data).unwrap().amount
}

async fn read_job(ctx: &mut ProgramTestContext, job: &Pubkey) -> JobAccount {
    let data = ctx.banks_client.get_account(*job).await.unwrap().unwrap().data;
    JobAccount::try_deserialize(&mut data.as_slice()).unwrap()
}

/// Raw bytes and lamports of the given accounts, for unchanged-state checks.
async fn snapshot(ctx: &mut ProgramTestContext, keys: &[Pubkey]) -> Vec<Option<(u64, Vec<u8>)>> {
    let mut out = Vec::new();
    for key in keys {
        let account = ctx.banks_client.get_account(*key).await.unwrap();
        out.push(account.map(|account| (account.lamports, account.data)));
    }
    out
}

#[allow(clippy::too_many_arguments)]
fn create_job_ix(
    buyer: &Pubkey,
    mint: &Pubkey,
    job_id: [u8; 32],
    executor: Pubkey,
    amount: u64,
    deadline_slot: u64,
) -> Instruction {
    let job = job_pda(&job_id);
    Instruction {
        program_id: vericode_escrow::ID,
        accounts: accounts::CreateJob {
            buyer: *buyer,
            mint: *mint,
            job,
            vault: vault_pda(&job),
            token_program: spl_token::ID,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
        data: instruction::CreateJob {
            job_id,
            executor,
            amount,
            deadline_slot,
            spec_hash: spec_hash(),
            harness_hash: harness_hash(),
            image_id: hex32(IMAGE_ID_HEX),
        }
        .data(),
    }
}

fn fund_ix(buyer: &Pubkey, job_id: [u8; 32], mint: &Pubkey, buyer_token: &Pubkey, amount: u64) -> Instruction {
    let job = job_pda(&job_id);
    Instruction {
        program_id: vericode_escrow::ID,
        accounts: accounts::Fund {
            buyer: *buyer,
            job,
            mint: *mint,
            buyer_token: *buyer_token,
            vault: vault_pda(&job),
            token_program: spl_token::ID,
        }
        .to_account_metas(None),
        data: instruction::Fund { amount }.data(),
    }
}

fn refund_ix(job_id: [u8; 32], mint: &Pubkey, buyer_token: &Pubkey) -> Instruction {
    let job = job_pda(&job_id);
    Instruction {
        program_id: vericode_escrow::ID,
        accounts: accounts::RefundOnTimeout {
            job,
            mint: *mint,
            vault: vault_pda(&job),
            buyer_token: *buyer_token,
            token_program: spl_token::ID,
        }
        .to_account_metas(None),
        data: instruction::RefundOnTimeout {}.data(),
    }
}

async fn setup() -> Env {
    let mut program_test = ProgramTest::new(PROGRAM_NAME, vericode_escrow::ID, None);
    program_test.prefer_bpf(true);
    let mut ctx = program_test.start_with_context().await;

    let buyer = Keypair::new();
    let executor = Keypair::new();
    let mint_authority = Keypair::new();
    transfer_lamports(&mut ctx, &buyer.pubkey(), 1_000_000_000).await;
    let mint = create_mint(&mut ctx, &mint_authority.pubkey()).await;
    let buyer_token = create_token_account(&mut ctx, &mint, &buyer.pubkey()).await;
    mint_to(&mut ctx, &mint, &buyer_token, &mint_authority, BUYER_START_BALANCE).await;

    Env {
        ctx,
        buyer,
        executor,
        mint,
        mint_authority,
        buyer_token,
    }
}

async fn create_default_job(env: &mut Env) {
    let ix = create_job_ix(
        &env.buyer.pubkey(),
        &env.mint,
        JOB_ID,
        env.executor.pubkey(),
        AMOUNT,
        DEADLINE,
    );
    let buyer = env.buyer.insecure_clone();
    send(&mut env.ctx, &[ix], &[&buyer]).await.unwrap();
}

async fn fund_default_job(env: &mut Env) {
    let ix = fund_ix(&env.buyer.pubkey(), JOB_ID, &env.mint, &env.buyer_token, AMOUNT);
    let buyer = env.buyer.insecure_clone();
    send(&mut env.ctx, &[ix], &[&buyer]).await.unwrap();
}

#[tokio::test]
async fn create_job_persists_terms_and_an_empty_pda_vault() {
    let mut env = setup().await;
    create_default_job(&mut env).await;

    let job = job_pda(&JOB_ID);
    let account = read_job(&mut env.ctx, &job).await;
    assert_eq!(account.version, JOB_ACCOUNT_VERSION);
    assert_eq!(account.job_id, JOB_ID);
    assert_eq!(account.buyer, env.buyer.pubkey());
    assert_eq!(account.executor, env.executor.pubkey());
    assert_eq!(account.mint, env.mint);
    assert_eq!(account.amount, AMOUNT);
    assert_eq!(account.deadline_slot, DEADLINE);
    assert_eq!(account.spec_hash, spec_hash());
    assert_eq!(account.harness_hash, harness_hash());
    assert_eq!(account.image_id, hex32(IMAGE_ID_HEX));
    assert_eq!(account.status, EscrowStatus::Created);

    let vault = vault_pda(&job);
    let vault_account = env.ctx.banks_client.get_account(vault).await.unwrap().unwrap();
    assert_eq!(vault_account.owner, spl_token::ID);
    let vault_state = spl_token::state::Account::unpack(&vault_account.data).unwrap();
    assert_eq!(vault_state.mint, env.mint);
    assert_eq!(vault_state.owner, job);
    assert_eq!(vault_state.amount, 0);
}

#[tokio::test]
async fn create_job_rejects_invalid_terms_without_creating_accounts() {
    let mut env = setup().await;
    let buyer = env.buyer.insecure_clone();
    let job = job_pda(&JOB_ID);

    let cases = [
        (Pubkey::new_from_array([0; 32]), AMOUNT, VericodeEscrowError::ZeroExecutor),
        (buyer.pubkey(), AMOUNT, VericodeEscrowError::BuyerIsExecutor),
        (env.executor.pubkey(), 0, VericodeEscrowError::AmountZero),
    ];
    for (executor, amount, error) in cases {
        let ix = create_job_ix(&buyer.pubkey(), &env.mint, JOB_ID, executor, amount, DEADLINE);
        assert_custom(send(&mut env.ctx, &[ix], &[&buyer]).await, custom(error));
        assert!(env.ctx.banks_client.get_account(job).await.unwrap().is_none());
        assert!(env.ctx.banks_client.get_account(vault_pda(&job)).await.unwrap().is_none());
    }
}

#[tokio::test]
async fn create_job_rejects_a_duplicate_job_id() {
    let mut env = setup().await;
    create_default_job(&mut env).await;
    let job = job_pda(&JOB_ID);
    let before = snapshot(&mut env.ctx, &[job, vault_pda(&job)]).await;

    let squatter = Keypair::new();
    transfer_lamports(&mut env.ctx, &squatter.pubkey(), 1_000_000_000).await;
    let ix = create_job_ix(&squatter.pubkey(), &env.mint, JOB_ID, env.executor.pubkey(), AMOUNT, DEADLINE);
    assert!(send(&mut env.ctx, &[ix], &[&squatter]).await.is_err());
    assert_eq!(snapshot(&mut env.ctx, &[job, vault_pda(&job)]).await, before);
}

#[tokio::test]
async fn fund_moves_exactly_the_job_amount_into_the_vault() {
    let mut env = setup().await;
    create_default_job(&mut env).await;
    fund_default_job(&mut env).await;

    let job = job_pda(&JOB_ID);
    assert_eq!(read_job(&mut env.ctx, &job).await.status, EscrowStatus::Funded);
    assert_eq!(token_balance(&mut env.ctx, &vault_pda(&job)).await, AMOUNT);
    assert_eq!(
        token_balance(&mut env.ctx, &env.buyer_token.clone()).await,
        BUYER_START_BALANCE - AMOUNT
    );
}

#[tokio::test]
async fn fund_rejections_move_no_tokens_and_keep_the_state() {
    let mut env = setup().await;
    create_default_job(&mut env).await;
    let job = job_pda(&JOB_ID);
    let vault = vault_pda(&job);
    let buyer = env.buyer.insecure_clone();

    // Other mint, with a buyer token account of that mint.
    let other_authority = Keypair::new();
    let other_mint = create_mint(&mut env.ctx, &other_authority.pubkey()).await;
    let buyer_other_token = create_token_account(&mut env.ctx, &other_mint, &buyer.pubkey()).await;
    mint_to(&mut env.ctx, &other_mint, &buyer_other_token, &other_authority, AMOUNT).await;

    // Another signer holding the Job mint.
    let intruder = Keypair::new();
    let intruder_token = create_token_account(&mut env.ctx, &env.mint, &intruder.pubkey()).await;
    let authority = env.mint_authority.insecure_clone();
    mint_to(&mut env.ctx, &env.mint, &intruder_token, &authority, AMOUNT).await;

    let watched = [job, vault, env.buyer_token, buyer_other_token, intruder_token];
    let before = snapshot(&mut env.ctx, &watched).await;

    let wrong_mint = fund_ix(&buyer.pubkey(), JOB_ID, &other_mint, &buyer_other_token, AMOUNT);
    assert_custom(
        send(&mut env.ctx, &[wrong_mint], &[&buyer]).await,
        custom(VericodeEscrowError::MintMismatch),
    );
    let wrong_signer = fund_ix(&intruder.pubkey(), JOB_ID, &env.mint, &intruder_token, AMOUNT);
    assert_custom(
        send(&mut env.ctx, &[wrong_signer], &[&intruder]).await,
        custom(VericodeEscrowError::DepositorMismatch),
    );
    let wrong_amount = fund_ix(&buyer.pubkey(), JOB_ID, &env.mint, &env.buyer_token, AMOUNT - 1);
    assert_custom(
        send(&mut env.ctx, &[wrong_amount], &[&buyer]).await,
        custom(VericodeEscrowError::AmountMismatch),
    );
    assert_eq!(snapshot(&mut env.ctx, &watched).await, before);

    fund_default_job(&mut env).await;
    let funded = snapshot(&mut env.ctx, &watched).await;
    let again = fund_ix(&buyer.pubkey(), JOB_ID, &env.mint, &env.buyer_token, AMOUNT);
    assert_custom(
        send(&mut env.ctx, &[again], &[&buyer]).await,
        custom(VericodeEscrowError::AlreadyFunded),
    );
    assert_eq!(snapshot(&mut env.ctx, &watched).await, funded);
}

#[tokio::test]
async fn vault_is_controlled_by_the_job_pda_without_private_key() {
    let mut env = setup().await;
    create_default_job(&mut env).await;
    let job = job_pda(&JOB_ID);
    let vault = vault_pda(&job);

    assert!(!job.is_on_curve(), "Job PDA must have no private key");
    assert!(!vault.is_on_curve(), "vault address must have no private key");
    let data = env.ctx.banks_client.get_account(vault).await.unwrap().unwrap().data;
    let state = spl_token::state::Account::unpack(&data).unwrap();
    assert_eq!(state.owner, job);
    assert!(state.delegate.is_none());
    assert!(state.close_authority.is_none());
}

#[tokio::test]
async fn timeout_refund_fails_before_and_at_the_deadline() {
    let mut env = setup().await;
    create_default_job(&mut env).await;
    fund_default_job(&mut env).await;
    let job = job_pda(&JOB_ID);
    let watched = [job, vault_pda(&job), env.buyer_token];
    let before = snapshot(&mut env.ctx, &watched).await;

    let early = refund_ix(JOB_ID, &env.mint, &env.buyer_token);
    assert_custom(
        send(&mut env.ctx, &[early], &[]).await,
        custom(VericodeEscrowError::DeadlineNotReached),
    );
    env.ctx.warp_to_slot(DEADLINE).unwrap();
    let at_deadline = refund_ix(JOB_ID, &env.mint, &env.buyer_token);
    assert_custom(
        send(&mut env.ctx, &[at_deadline], &[]).await,
        custom(VericodeEscrowError::DeadlineNotReached),
    );
    assert_eq!(snapshot(&mut env.ctx, &watched).await, before);
}

#[tokio::test]
async fn timeout_refund_after_the_deadline_returns_exactly_the_amount_to_the_buyer() {
    let mut env = setup().await;
    create_default_job(&mut env).await;
    fund_default_job(&mut env).await;
    let job = job_pda(&JOB_ID);
    env.ctx.warp_to_slot(DEADLINE + 1).unwrap();

    // Permissionless: only the fee payer signs; the buyer does not.
    let refund = refund_ix(JOB_ID, &env.mint, &env.buyer_token);
    send(&mut env.ctx, &[refund], &[]).await.unwrap();

    assert_eq!(
        read_job(&mut env.ctx, &job).await.status,
        EscrowStatus::RefundedOnTimeout
    );
    assert_eq!(token_balance(&mut env.ctx, &vault_pda(&job)).await, 0);
    assert_eq!(
        token_balance(&mut env.ctx, &env.buyer_token.clone()).await,
        BUYER_START_BALANCE
    );
}

#[tokio::test]
async fn timeout_refund_rejects_other_recipients_and_mints() {
    let mut env = setup().await;
    create_default_job(&mut env).await;
    fund_default_job(&mut env).await;
    let job = job_pda(&JOB_ID);
    let executor = env.executor.pubkey();
    let executor_token = create_token_account(&mut env.ctx, &env.mint, &executor).await;
    let other_authority = Keypair::new();
    let other_mint = create_mint(&mut env.ctx, &other_authority.pubkey()).await;
    let buyer = env.buyer.pubkey();
    let buyer_other_token = create_token_account(&mut env.ctx, &other_mint, &buyer).await;
    env.ctx.warp_to_slot(DEADLINE + 1).unwrap();

    let watched = [job, vault_pda(&job), env.buyer_token, executor_token, buyer_other_token];
    let before = snapshot(&mut env.ctx, &watched).await;

    let to_executor = refund_ix(JOB_ID, &env.mint, &executor_token);
    assert_custom(
        send(&mut env.ctx, &[to_executor], &[]).await,
        custom(VericodeEscrowError::RecipientMismatch),
    );
    let other_mint_account = refund_ix(JOB_ID, &env.mint, &buyer_other_token);
    assert_custom(
        send(&mut env.ctx, &[other_mint_account], &[]).await,
        custom(VericodeEscrowError::MintMismatch),
    );
    assert_eq!(snapshot(&mut env.ctx, &watched).await, before);
}

#[tokio::test]
async fn timeout_refund_requires_funding_and_settles_once() {
    let mut env = setup().await;
    create_default_job(&mut env).await;
    let job = job_pda(&JOB_ID);
    env.ctx.warp_to_slot(DEADLINE + 1).unwrap();
    let watched = [job, vault_pda(&job), env.buyer_token];

    let before = snapshot(&mut env.ctx, &watched).await;
    let unfunded = refund_ix(JOB_ID, &env.mint, &env.buyer_token);
    assert_custom(
        send(&mut env.ctx, &[unfunded], &[]).await,
        custom(VericodeEscrowError::NotFunded),
    );
    assert_eq!(snapshot(&mut env.ctx, &watched).await, before);

    fund_default_job(&mut env).await;
    let refund = refund_ix(JOB_ID, &env.mint, &env.buyer_token);
    send(&mut env.ctx, &[refund], &[]).await.unwrap();
    let settled = snapshot(&mut env.ctx, &watched).await;

    let again = refund_ix(JOB_ID, &env.mint, &env.buyer_token);
    assert_custom(
        send(&mut env.ctx, &[again], &[]).await,
        custom(VericodeEscrowError::AlreadyRefunded),
    );
    let buyer = env.buyer.insecure_clone();
    let refund_then_fund = fund_ix(&buyer.pubkey(), JOB_ID, &env.mint, &env.buyer_token, AMOUNT);
    assert_custom(
        send(&mut env.ctx, &[refund_then_fund], &[&buyer]).await,
        custom(VericodeEscrowError::AlreadyRefunded),
    );
    assert_eq!(snapshot(&mut env.ctx, &watched).await, settled);
}
