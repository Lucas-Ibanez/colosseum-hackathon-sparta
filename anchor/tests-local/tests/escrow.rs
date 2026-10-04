//! In-process tests of the SBF build of `vericode_escrow`.
//!
//! The program is loaded from `SBF_OUT_DIR` by `solana-program-test`; no
//! validator, cluster, RPC or keypair file is used. Every rejected
//! instruction is checked for unchanged Job, vault and token balances, and
//! expected error codes are literals, not values derived from the program.

use std::{fs, path::PathBuf};

use anchor_lang::solana_program::clock::Clock;
use anchor_lang::solana_program::instruction::{Instruction, InstructionError};
use anchor_lang::solana_program::program_pack::Pack;
use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_lang::{AccountDeserialize, InstructionData, ToAccountMetas};
use anchor_spl::token::spl_token;
use anchor_spl::token_2022::spl_token_2022;
use solana_keypair::Keypair;
use solana_program_test::{tokio, BanksClientError, ProgramTest, ProgramTestContext};
use solana_signer::Signer;
use solana_system_interface::instruction as system_instruction;
use solana_system_interface::program as system_program;
use solana_transaction::Transaction;
use solana_transaction_error::TransactionError;
use vericode_core::{hash_restricted_artifact, JournalV1, RestrictedArtifactV1};
use vericode_escrow::{
    accounts, instruction, EscrowStatus, JobAccount, JOB_ACCOUNT_VERSION, JOB_SEED, VAULT_SEED,
};

const PROGRAM_NAME: &str = "vericode_escrow";
const DECIMALS: u8 = 6;
const AMOUNT: u64 = 1_000_000;
const BUYER_START_BALANCE: u64 = 5_000_000;
// Inside the creation window of a Job created at the first slots of the test.
const DEADLINE: u64 = 5_000;
const JOB_ID: [u8; 32] = [0x11; 32];
// Final D1c2b guest ImageID, the only image a v1 Job admits.
const IMAGE_ID_HEX: &str = "4da06f90da75ec8980c943ce017d69c48370fddbf3aa27689d375d78fac0fb1a";
// Creation window decided for D2b.1, in slots.
const MIN_WINDOW: u64 = 1_500;
const MAX_WINDOW: u64 = 1_512_000;
const OTHER: [u8; 32] = [0x99; 32];

// `VericodeEscrowError` codes; `tests/layout.rs` binds each to its variant.
const E_AMOUNT_ZERO: u32 = 6000;
const E_ZERO_EXECUTOR: u32 = 6002;
const E_BUYER_IS_EXECUTOR: u32 = 6004;
const E_NOT_FUNDED: u32 = 6005;
const E_ALREADY_FUNDED: u32 = 6006;
const E_ALREADY_REFUNDED: u32 = 6008;
const E_DEPOSITOR_MISMATCH: u32 = 6009;
const E_RECIPIENT_MISMATCH: u32 = 6010;
const E_MINT_MISMATCH: u32 = 6011;
const E_AMOUNT_MISMATCH: u32 = 6012;
const E_DEADLINE_NOT_REACHED: u32 = 6021;
const E_DEADLINE_PASSED: u32 = 6022;
const E_UNSUPPORTED_ACCOUNT_VERSION: u32 = 6023;
const E_MINT_HAS_FREEZE_AUTHORITY: u32 = 6024;
const E_ALREADY_DELIVERED: u32 = 6026;
const E_DELIVERER_MISMATCH: u32 = 6027;
const E_SPEC_NOT_ADMITTED: u32 = 6028;
const E_HARNESS_NOT_ADMITTED: u32 = 6029;
const E_IMAGE_ID_NOT_ADMITTED: u32 = 6030;
const E_DEADLINE_OUT_OF_WINDOW: u32 = 6031;
const E_EXECUTOR_IS_PROGRAM_ACCOUNT: u32 = 6032;
// Anchor 0.31.1 framework errors.
const ANCHOR_CONSTRAINT_SEEDS: u32 = 2006;
const ANCHOR_ACCOUNT_OWNED_BY_WRONG_PROGRAM: u32 = 3007;
const ANCHOR_INVALID_PROGRAM_ID: u32 = 3008;
const ANCHOR_ACCOUNT_NOT_SIGNER: u32 = 3010;
// System Program `AccountAlreadyInUse`.
const SYSTEM_ACCOUNT_ALREADY_IN_USE: u32 = 0;

struct Env {
    ctx: ProgramTestContext,
    buyer: Keypair,
    executor: Keypair,
    mint: Pubkey,
    mint_authority: Keypair,
    buyer_token: Pubkey,
}

/// Commitments sent to `create_job`.
#[derive(Clone, Copy)]
struct Commitments {
    spec_hash: [u8; 32],
    harness_hash: [u8; 32],
    image_id: [u8; 32],
}

fn hex32(hex: &str) -> [u8; 32] {
    decode_hex(hex).try_into().expect("32 bytes")
}

fn decode_hex(hex: &str) -> Vec<u8> {
    assert!(hex.len() % 2 == 0, "odd hex length");
    (0..hex.len())
        .step_by(2)
        .map(|index| u8::from_str_radix(&hex[index..index + 2], 16).unwrap())
        .collect()
}

fn admitted() -> Commitments {
    Commitments {
        spec_hash: vericode_core::hash_restricted_spec(&vericode_core::RESTRICTED_SPEC_V1)
            .unwrap()
            .into_bytes(),
        harness_hash: vericode_core::hash_harness_version(vericode_core::DETERMINISTIC_HARNESS_VERSION)
            .unwrap()
            .into_bytes(),
        image_id: hex32(IMAGE_ID_HEX),
    }
}

/// Commitment of the artifact `(7, 14)` that the executor delivers.
fn delivered_artifact_hash() -> [u8; 32] {
    hash_restricted_artifact(&RestrictedArtifactV1::new(7, 14))
        .unwrap()
        .into_bytes()
}

/// Artifact commitment of a versioned Groth16 fixture journal (Job `0x11`).
fn fixture_artifact_hash(name: &str) -> [u8; 32] {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/groth16")
        .join(format!("{name}.txt"));
    let text = fs::read_to_string(&path).unwrap();
    let journal = text
        .lines()
        .find_map(|line| line.strip_prefix("journal="))
        .expect("journal line");
    JournalV1::decode_candidate(&decode_hex(journal.trim()))
        .unwrap()
        .artifact_hash()
        .into_bytes()
}

fn job_pda(job_id: &[u8; 32]) -> Pubkey {
    Pubkey::find_program_address(&[JOB_SEED, job_id], &vericode_escrow::ID).0
}

fn vault_pda(job: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[VAULT_SEED, job.as_ref()], &vericode_escrow::ID).0
}

/// Replaces one account of an instruction, keeping its signer and writable flags.
fn substitute(mut ix: Instruction, original: &Pubkey, replacement: &Pubkey) -> Instruction {
    let meta = ix
        .accounts
        .iter_mut()
        .find(|meta| meta.pubkey == *original)
        .expect("account in instruction");
    meta.pubkey = *replacement;
    ix
}

/// Clears the signer flag of one account, as a caller without its key would.
fn unsigned(mut ix: Instruction, key: &Pubkey) -> Instruction {
    let meta = ix
        .accounts
        .iter_mut()
        .find(|meta| meta.pubkey == *key)
        .expect("account in instruction");
    meta.is_signer = false;
    ix
}

fn signed_transaction(
    ctx: &ProgramTestContext,
    instructions: &[Instruction],
    signers: &[&Keypair],
    blockhash: anchor_lang::solana_program::hash::Hash,
) -> Transaction {
    let mut all_signers: Vec<&Keypair> = vec![&ctx.payer];
    all_signers.extend_from_slice(signers);
    Transaction::new_signed_with_payer(instructions, Some(&ctx.payer.pubkey()), &all_signers, blockhash)
}

async fn send(
    ctx: &mut ProgramTestContext,
    instructions: &[Instruction],
    signers: &[&Keypair],
) -> Result<(), BanksClientError> {
    let blockhash = ctx.get_new_latest_blockhash().await.unwrap();
    let transaction = signed_transaction(ctx, instructions, signers, blockhash);
    ctx.banks_client.process_transaction(transaction).await
}

/// Simulates a transaction, then sends the same transaction, and returns the
/// compute units the simulation consumed.
///
/// Simulation takes no account locks, so it cannot race with the transaction
/// queue of the test bank as a direct execution with metadata can.
async fn compute_units(ctx: &mut ProgramTestContext, instructions: &[Instruction], signers: &[&Keypair]) -> u64 {
    let blockhash = ctx.get_new_latest_blockhash().await.unwrap();
    let transaction = signed_transaction(ctx, instructions, signers, blockhash);
    let simulation = ctx
        .banks_client
        .simulate_transaction(transaction.clone())
        .await
        .unwrap();
    simulation.result.unwrap().unwrap();
    ctx.banks_client.process_transaction(transaction).await.unwrap();
    simulation.simulation_details.unwrap().units_consumed
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

async fn current_slot(ctx: &mut ProgramTestContext) -> u64 {
    ctx.banks_client.get_sysvar::<Clock>().await.unwrap().slot
}

async fn warp(ctx: &mut ProgramTestContext, slot: u64) {
    ctx.warp_to_slot(slot).unwrap();
    assert_eq!(current_slot(ctx).await, slot, "Clock.slot after warp");
}

async fn transfer_lamports(ctx: &mut ProgramTestContext, to: &Pubkey, lamports: u64) {
    let ix = system_instruction::transfer(&ctx.payer.pubkey(), to, lamports);
    send(ctx, &[ix], &[]).await.unwrap();
}

async fn create_mint(ctx: &mut ProgramTestContext, authority: &Pubkey) -> Pubkey {
    create_mint_with_freeze(ctx, authority, None).await
}

async fn create_mint_with_freeze(
    ctx: &mut ProgramTestContext,
    authority: &Pubkey,
    freeze_authority: Option<&Pubkey>,
) -> Pubkey {
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
        spl_token::instruction::initialize_mint2(
            &spl_token::ID,
            &mint.pubkey(),
            authority,
            freeze_authority,
            DECIMALS,
        )
        .unwrap();
    send(ctx, &[create, init], &[&mint]).await.unwrap();
    mint.pubkey()
}

/// Token-2022 mint without extensions or freeze authority.
async fn create_token_2022_mint(ctx: &mut ProgramTestContext, authority: &Pubkey) -> Pubkey {
    let mint = Keypair::new();
    let rent = ctx.banks_client.get_rent().await.unwrap();
    // The base Token-2022 mint has the classic 82-byte layout.
    let create = system_instruction::create_account(
        &ctx.payer.pubkey(),
        &mint.pubkey(),
        rent.minimum_balance(spl_token::state::Mint::LEN),
        spl_token::state::Mint::LEN as u64,
        &spl_token_2022::ID,
    );
    let init = spl_token_2022::instruction::initialize_mint2(
        &spl_token_2022::ID,
        &mint.pubkey(),
        authority,
        None,
        DECIMALS,
    )
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
fn create_job_ix_with(
    buyer: &Pubkey,
    mint: &Pubkey,
    job_id: [u8; 32],
    executor: Pubkey,
    amount: u64,
    deadline_slot: u64,
    commitments: Commitments,
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
            spec_hash: commitments.spec_hash,
            harness_hash: commitments.harness_hash,
            image_id: commitments.image_id,
        }
        .data(),
    }
}

fn create_job_ix(
    buyer: &Pubkey,
    mint: &Pubkey,
    job_id: [u8; 32],
    executor: Pubkey,
    amount: u64,
    deadline_slot: u64,
) -> Instruction {
    create_job_ix_with(buyer, mint, job_id, executor, amount, deadline_slot, admitted())
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

fn deliver_ix(executor: &Pubkey, job_id: [u8; 32], artifact_hash: [u8; 32]) -> Instruction {
    Instruction {
        program_id: vericode_escrow::ID,
        accounts: accounts::Deliver {
            executor: *executor,
            job: job_pda(&job_id),
        }
        .to_account_metas(None),
        data: instruction::Deliver { artifact_hash }.data(),
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

async fn create_job_with_deadline(env: &mut Env, job_id: [u8; 32], deadline_slot: u64) {
    let slot = current_slot(&mut env.ctx).await;
    assert!(
        slot + MIN_WINDOW <= deadline_slot && deadline_slot <= slot + MAX_WINDOW,
        "deadline {deadline_slot} outside the creation window at slot {slot}"
    );
    let ix = create_job_ix(
        &env.buyer.pubkey(),
        &env.mint,
        job_id,
        env.executor.pubkey(),
        AMOUNT,
        deadline_slot,
    );
    let buyer = env.buyer.insecure_clone();
    send(&mut env.ctx, &[ix], &[&buyer]).await.unwrap();
}

async fn create_default_job(env: &mut Env) {
    create_job_with_deadline(env, JOB_ID, DEADLINE).await;
}

async fn fund_job(env: &mut Env, job_id: [u8; 32]) {
    let ix = fund_ix(&env.buyer.pubkey(), job_id, &env.mint, &env.buyer_token, AMOUNT);
    let buyer = env.buyer.insecure_clone();
    send(&mut env.ctx, &[ix], &[&buyer]).await.unwrap();
}

async fn fund_default_job(env: &mut Env) {
    fund_job(env, JOB_ID).await;
}

async fn deliver_default_job(env: &mut Env) {
    let executor = env.executor.insecure_clone();
    let ix = deliver_ix(&executor.pubkey(), JOB_ID, delivered_artifact_hash());
    send(&mut env.ctx, &[ix], &[&executor]).await.unwrap();
}

#[tokio::test]
async fn create_job_rejects_invalid_terms_without_creating_accounts() {
    let mut env = setup().await;
    let buyer = env.buyer.insecure_clone();
    let job = job_pda(&JOB_ID);
    let watched = [buyer.pubkey(), job, vault_pda(&job)];
    let before = snapshot(&mut env.ctx, &watched).await;

    let cases = [
        (Pubkey::new_from_array([0; 32]), AMOUNT, E_ZERO_EXECUTOR),
        (buyer.pubkey(), AMOUNT, E_BUYER_IS_EXECUTOR),
        (env.executor.pubkey(), 0, E_AMOUNT_ZERO),
    ];
    for (executor, amount, error) in cases {
        let ix = create_job_ix(&buyer.pubkey(), &env.mint, JOB_ID, executor, amount, DEADLINE);
        assert_custom(send(&mut env.ctx, &[ix], &[&buyer]).await, error);
        assert_eq!(snapshot(&mut env.ctx, &watched).await, before);
    }
}

#[tokio::test]
async fn create_job_rejects_an_executor_that_is_a_program_account() {
    let mut env = setup().await;
    let buyer = env.buyer.insecure_clone();
    let job = job_pda(&JOB_ID);
    let vault = vault_pda(&job);
    let watched = [buyer.pubkey(), job, vault];
    let before = snapshot(&mut env.ctx, &watched).await;

    for executor in [job, vault] {
        let ix = create_job_ix(&buyer.pubkey(), &env.mint, JOB_ID, executor, AMOUNT, DEADLINE);
        assert_custom(send(&mut env.ctx, &[ix], &[&buyer]).await, E_EXECUTOR_IS_PROGRAM_ACCOUNT);
        assert_eq!(snapshot(&mut env.ctx, &watched).await, before);
    }
}

#[tokio::test]
async fn create_job_rejects_terms_not_admitted_for_v1() {
    let mut env = setup().await;
    let buyer = env.buyer.insecure_clone();
    let job = job_pda(&JOB_ID);
    let watched = [buyer.pubkey(), job, vault_pda(&job)];
    let before = snapshot(&mut env.ctx, &watched).await;
    let admitted = admitted();

    let cases = [
        (Commitments { spec_hash: OTHER, ..admitted }, E_SPEC_NOT_ADMITTED),
        (
            Commitments {
                spec_hash: admitted.harness_hash,
                harness_hash: admitted.spec_hash,
                ..admitted
            },
            E_SPEC_NOT_ADMITTED,
        ),
        (Commitments { harness_hash: OTHER, ..admitted }, E_HARNESS_NOT_ADMITTED),
        // A guest chosen by the buyer, for example one that always fails.
        (Commitments { image_id: OTHER, ..admitted }, E_IMAGE_ID_NOT_ADMITTED),
    ];
    for (commitments, error) in cases {
        let ix = create_job_ix_with(
            &buyer.pubkey(),
            &env.mint,
            JOB_ID,
            env.executor.pubkey(),
            AMOUNT,
            DEADLINE,
            commitments,
        );
        assert_custom(send(&mut env.ctx, &[ix], &[&buyer]).await, error);
        assert_eq!(snapshot(&mut env.ctx, &watched).await, before);
    }
}

#[tokio::test]
async fn create_job_bounds_the_deadline_window() {
    let mut env = setup().await;
    let buyer = env.buyer.insecure_clone();
    let slot = current_slot(&mut env.ctx).await;
    let job = job_pda(&JOB_ID);
    let watched = [buyer.pubkey(), job, vault_pda(&job)];
    let before = snapshot(&mut env.ctx, &watched).await;

    // R-D2 PoC-5 (deadline 0) and PoC-9 (deadline u64::MAX) included.
    for deadline in [slot + MIN_WINDOW - 1, slot + MAX_WINDOW + 1, 0, slot, u64::MAX] {
        let ix = create_job_ix(&buyer.pubkey(), &env.mint, JOB_ID, env.executor.pubkey(), AMOUNT, deadline);
        assert_custom(send(&mut env.ctx, &[ix], &[&buyer]).await, E_DEADLINE_OUT_OF_WINDOW);
        assert_eq!(snapshot(&mut env.ctx, &watched).await, before);
    }

    // Both edges are admitted, against the same Clock slot.
    for (job_id, deadline) in [([0x21; 32], slot + MIN_WINDOW), ([0x22; 32], slot + MAX_WINDOW)] {
        let ix = create_job_ix(&buyer.pubkey(), &env.mint, job_id, env.executor.pubkey(), AMOUNT, deadline);
        send(&mut env.ctx, &[ix], &[&buyer]).await.unwrap();
        assert_eq!(read_job(&mut env.ctx, &job_pda(&job_id)).await.deadline_slot, deadline);
    }
    assert_eq!(current_slot(&mut env.ctx).await, slot, "Clock.slot moved during the test");
}

#[tokio::test]
async fn create_job_rejects_token_2022() {
    let mut env = setup().await;
    let buyer = env.buyer.insecure_clone();
    let authority = env.mint_authority.pubkey();
    let token_2022_mint = create_token_2022_mint(&mut env.ctx, &authority).await;
    let job = job_pda(&JOB_ID);
    let watched = [buyer.pubkey(), job, vault_pda(&job)];
    let before = snapshot(&mut env.ctx, &watched).await;

    let executor = env.executor.pubkey();
    let foreign_mint = create_job_ix(&buyer.pubkey(), &token_2022_mint, JOB_ID, executor, AMOUNT, DEADLINE);
    assert_custom(
        send(&mut env.ctx, &[foreign_mint], &[&buyer]).await,
        ANCHOR_ACCOUNT_OWNED_BY_WRONG_PROGRAM,
    );
    let foreign_program = substitute(
        create_job_ix(&buyer.pubkey(), &env.mint, JOB_ID, executor, AMOUNT, DEADLINE),
        &spl_token::ID,
        &spl_token_2022::ID,
    );
    assert_custom(
        send(&mut env.ctx, &[foreign_program], &[&buyer]).await,
        ANCHOR_INVALID_PROGRAM_ID,
    );
    assert_eq!(snapshot(&mut env.ctx, &watched).await, before);
}

#[tokio::test]
async fn create_job_rejects_a_mint_with_freeze_authority() {
    let mut env = setup().await;
    let buyer = env.buyer.insecure_clone();
    let freezer = Keypair::new();
    let freezable = create_mint_with_freeze(
        &mut env.ctx,
        &freezer.pubkey(),
        Some(&freezer.pubkey()),
    )
    .await;
    let job = job_pda(&JOB_ID);

    let ix = create_job_ix(&buyer.pubkey(), &freezable, JOB_ID, env.executor.pubkey(), AMOUNT, DEADLINE);
    assert_custom(send(&mut env.ctx, &[ix], &[&buyer]).await, E_MINT_HAS_FREEZE_AUTHORITY);
    assert!(env.ctx.banks_client.get_account(job).await.unwrap().is_none());
    assert!(env.ctx.banks_client.get_account(vault_pda(&job)).await.unwrap().is_none());
}

#[tokio::test]
async fn freeze_authority_cannot_be_added_to_the_job_mint() {
    let mut env = setup().await;
    create_default_job(&mut env).await;
    let authority = env.mint_authority.insecure_clone();

    // The accepted mint was created without freeze authority; the SPL Token
    // program executed here (`spl_token-3.5.0.so`) refuses to add one later,
    // so the vault can never be frozen.
    let ix = spl_token::instruction::set_authority(
        &spl_token::ID,
        &env.mint,
        Some(&authority.pubkey()),
        spl_token::instruction::AuthorityType::FreezeAccount,
        &authority.pubkey(),
        &[],
    )
    .unwrap();
    assert_custom(
        send(&mut env.ctx, &[ix], &[&authority]).await,
        spl_token::error::TokenError::MintCannotFreeze as u32,
    );
}

#[tokio::test]
async fn create_job_rejects_a_duplicate_job_id() {
    let mut env = setup().await;
    let job = job_pda(&JOB_ID);

    // R-D2 PoC-9: a squatter takes the Job ID first, with its own mint and
    // amount, inside the creation window.
    let squatter = Keypair::new();
    transfer_lamports(&mut env.ctx, &squatter.pubkey(), 1_000_000_000).await;
    let squatter_mint = create_mint(&mut env.ctx, &squatter.pubkey()).await;
    let squat = create_job_ix(&squatter.pubkey(), &squatter_mint, JOB_ID, env.executor.pubkey(), 1, DEADLINE);
    send(&mut env.ctx, &[squat], &[&squatter]).await.unwrap();

    let buyer = env.buyer.insecure_clone();
    let watched = [buyer.pubkey(), job, vault_pda(&job)];
    let before = snapshot(&mut env.ctx, &watched).await;
    let ix = create_job_ix(&buyer.pubkey(), &env.mint, JOB_ID, env.executor.pubkey(), AMOUNT, DEADLINE);
    assert_custom(send(&mut env.ctx, &[ix], &[&buyer]).await, SYSTEM_ACCOUNT_ALREADY_IN_USE);
    assert_eq!(snapshot(&mut env.ctx, &watched).await, before);
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
    assert_eq!(account.spec_hash, admitted().spec_hash);
    assert_eq!(account.harness_hash, admitted().harness_hash);
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
    assert_custom(send(&mut env.ctx, &[wrong_mint], &[&buyer]).await, E_MINT_MISMATCH);
    // R-D2 F-08: a foreign mint account is a VeriCode error, not an SPL one.
    let foreign_mint_account = fund_ix(&buyer.pubkey(), JOB_ID, &other_mint, &env.buyer_token, AMOUNT);
    assert_custom(
        send(&mut env.ctx, &[foreign_mint_account], &[&buyer]).await,
        E_MINT_MISMATCH,
    );
    let wrong_signer = fund_ix(&intruder.pubkey(), JOB_ID, &env.mint, &intruder_token, AMOUNT);
    assert_custom(
        send(&mut env.ctx, &[wrong_signer], &[&intruder]).await,
        E_DEPOSITOR_MISMATCH,
    );
    let wrong_amount = fund_ix(&buyer.pubkey(), JOB_ID, &env.mint, &env.buyer_token, AMOUNT - 1);
    assert_custom(send(&mut env.ctx, &[wrong_amount], &[&buyer]).await, E_AMOUNT_MISMATCH);
    assert_eq!(snapshot(&mut env.ctx, &watched).await, before);

    fund_default_job(&mut env).await;
    let funded = snapshot(&mut env.ctx, &watched).await;
    let again = fund_ix(&buyer.pubkey(), JOB_ID, &env.mint, &env.buyer_token, AMOUNT);
    assert_custom(send(&mut env.ctx, &[again], &[&buyer]).await, E_ALREADY_FUNDED);
    assert_eq!(snapshot(&mut env.ctx, &watched).await, funded);
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
async fn deliver_rejects_other_signers_and_an_unfunded_job() {
    let mut env = setup().await;
    create_default_job(&mut env).await;
    let job = job_pda(&JOB_ID);
    let watched = [job, vault_pda(&job), env.buyer_token];
    let executor = env.executor.insecure_clone();
    let buyer = env.buyer.insecure_clone();
    let intruder = Keypair::new();
    let hash = delivered_artifact_hash();

    let created = snapshot(&mut env.ctx, &watched).await;
    let unfunded = deliver_ix(&executor.pubkey(), JOB_ID, hash);
    assert_custom(send(&mut env.ctx, &[unfunded], &[&executor]).await, E_NOT_FUNDED);
    assert_eq!(snapshot(&mut env.ctx, &watched).await, created);

    fund_default_job(&mut env).await;
    let funded = snapshot(&mut env.ctx, &watched).await;
    for signer in [&buyer, &intruder] {
        let ix = deliver_ix(&signer.pubkey(), JOB_ID, hash);
        assert_custom(send(&mut env.ctx, &[ix], &[signer]).await, E_DELIVERER_MISMATCH);
    }
    // Naming the executor without its signature is not a delivery.
    let forged = unsigned(deliver_ix(&executor.pubkey(), JOB_ID, hash), &executor.pubkey());
    assert_custom(send(&mut env.ctx, &[forged], &[]).await, ANCHOR_ACCOUNT_NOT_SIGNER);
    assert_eq!(snapshot(&mut env.ctx, &watched).await, funded);
    assert_eq!(read_job(&mut env.ctx, &job).await.status, EscrowStatus::Funded);
}

#[tokio::test]
async fn deliver_is_rejected_after_the_deadline_and_accepted_at_it() {
    let mut env = setup().await;
    let late_job_id = [0x12; 32];
    create_default_job(&mut env).await;
    create_job_with_deadline(&mut env, late_job_id, DEADLINE - 1).await;
    fund_default_job(&mut env).await;
    fund_job(&mut env, late_job_id).await;
    let executor = env.executor.insecure_clone();
    let hash = delivered_artifact_hash();

    warp(&mut env.ctx, DEADLINE).await;
    let late_job = job_pda(&late_job_id);
    let watched = [late_job, vault_pda(&late_job), env.buyer_token];
    let before = snapshot(&mut env.ctx, &watched).await;
    // One slot after the deadline of the late Job.
    let late = deliver_ix(&executor.pubkey(), late_job_id, hash);
    assert_custom(send(&mut env.ctx, &[late], &[&executor]).await, E_DEADLINE_PASSED);
    assert_eq!(snapshot(&mut env.ctx, &watched).await, before);

    // At the deadline slot of the default Job.
    let on_time = deliver_ix(&executor.pubkey(), JOB_ID, hash);
    send(&mut env.ctx, &[on_time], &[&executor]).await.unwrap();
    assert_eq!(
        read_job(&mut env.ctx, &job_pda(&JOB_ID)).await.status,
        EscrowStatus::Delivered { artifact_hash: hash }
    );
}

#[tokio::test]
async fn deliver_records_the_executor_commitment_once() {
    let mut env = setup().await;
    create_default_job(&mut env).await;
    fund_default_job(&mut env).await;
    let job = job_pda(&JOB_ID);
    let vault = vault_pda(&job);
    let hash = delivered_artifact_hash();
    // The same artifact as the versioned Groth16 PASS fixture of Job 0x11.
    assert_eq!(hash, fixture_artifact_hash("pass"));

    let balances = snapshot(&mut env.ctx, &[vault, env.buyer_token]).await;
    deliver_default_job(&mut env).await;
    assert_eq!(
        read_job(&mut env.ctx, &job).await.status,
        EscrowStatus::Delivered { artifact_hash: hash }
    );
    assert_eq!(snapshot(&mut env.ctx, &[vault, env.buyer_token]).await, balances);

    let watched = [job, vault, env.buyer_token];
    let delivered = snapshot(&mut env.ctx, &watched).await;
    let executor = env.executor.insecure_clone();
    for artifact in [hash, fixture_artifact_hash("fail")] {
        let again = deliver_ix(&executor.pubkey(), JOB_ID, artifact);
        assert_custom(send(&mut env.ctx, &[again], &[&executor]).await, E_ALREADY_DELIVERED);
    }
    let buyer = env.buyer.insecure_clone();
    let fund_after_deliver = fund_ix(&buyer.pubkey(), JOB_ID, &env.mint, &env.buyer_token, AMOUNT);
    assert_custom(
        send(&mut env.ctx, &[fund_after_deliver], &[&buyer]).await,
        E_ALREADY_FUNDED,
    );
    assert_eq!(snapshot(&mut env.ctx, &watched).await, delivered);
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
    assert_custom(send(&mut env.ctx, &[early], &[]).await, E_DEADLINE_NOT_REACHED);
    warp(&mut env.ctx, DEADLINE).await;
    let at_deadline = refund_ix(JOB_ID, &env.mint, &env.buyer_token);
    assert_custom(send(&mut env.ctx, &[at_deadline], &[]).await, E_DEADLINE_NOT_REACHED);
    assert_eq!(current_slot(&mut env.ctx).await, DEADLINE);
    assert_eq!(snapshot(&mut env.ctx, &watched).await, before);
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
    warp(&mut env.ctx, DEADLINE + 1).await;

    let watched = [job, vault_pda(&job), env.buyer_token, executor_token, buyer_other_token];
    let before = snapshot(&mut env.ctx, &watched).await;

    let to_executor = refund_ix(JOB_ID, &env.mint, &executor_token);
    assert_custom(send(&mut env.ctx, &[to_executor], &[]).await, E_RECIPIENT_MISMATCH);
    let other_mint_account = refund_ix(JOB_ID, &env.mint, &buyer_other_token);
    assert_custom(send(&mut env.ctx, &[other_mint_account], &[]).await, E_MINT_MISMATCH);
    // R-D2 PoC-3 / F-08: a foreign mint account used to fail inside SPL Token
    // (`0x3`); it is now bound to the Job mint by the program.
    let foreign_mint_account = refund_ix(JOB_ID, &other_mint, &env.buyer_token);
    assert_custom(send(&mut env.ctx, &[foreign_mint_account], &[]).await, E_MINT_MISMATCH);
    assert_eq!(snapshot(&mut env.ctx, &watched).await, before);
}

#[tokio::test]
async fn timeout_refund_requires_funding_and_settles_once() {
    let mut env = setup().await;
    create_default_job(&mut env).await;
    let job = job_pda(&JOB_ID);
    warp(&mut env.ctx, DEADLINE + 1).await;
    let watched = [job, vault_pda(&job), env.buyer_token];

    let before = snapshot(&mut env.ctx, &watched).await;
    let unfunded = refund_ix(JOB_ID, &env.mint, &env.buyer_token);
    assert_custom(send(&mut env.ctx, &[unfunded], &[]).await, E_NOT_FUNDED);
    assert_eq!(snapshot(&mut env.ctx, &watched).await, before);

    fund_default_job(&mut env).await;
    let refund = refund_ix(JOB_ID, &env.mint, &env.buyer_token);
    send(&mut env.ctx, &[refund], &[]).await.unwrap();
    let settled = snapshot(&mut env.ctx, &watched).await;

    let again = refund_ix(JOB_ID, &env.mint, &env.buyer_token);
    assert_custom(send(&mut env.ctx, &[again], &[]).await, E_ALREADY_REFUNDED);
    let buyer = env.buyer.insecure_clone();
    let refund_then_fund = fund_ix(&buyer.pubkey(), JOB_ID, &env.mint, &env.buyer_token, AMOUNT);
    assert_custom(
        send(&mut env.ctx, &[refund_then_fund], &[&buyer]).await,
        E_ALREADY_REFUNDED,
    );
    assert_eq!(snapshot(&mut env.ctx, &watched).await, settled);
}

#[tokio::test]
async fn timeout_refund_after_the_deadline_returns_exactly_the_amount_to_the_buyer() {
    let mut env = setup().await;
    create_default_job(&mut env).await;
    fund_default_job(&mut env).await;
    let job = job_pda(&JOB_ID);
    warp(&mut env.ctx, DEADLINE + 1).await;

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
async fn timeout_refund_returns_a_delivered_job_to_the_buyer() {
    let mut env = setup().await;
    create_default_job(&mut env).await;
    fund_default_job(&mut env).await;
    deliver_default_job(&mut env).await;
    let job = job_pda(&JOB_ID);
    let watched = [job, vault_pda(&job), env.buyer_token];
    let before = snapshot(&mut env.ctx, &watched).await;

    let early = refund_ix(JOB_ID, &env.mint, &env.buyer_token);
    assert_custom(send(&mut env.ctx, &[early], &[]).await, E_DEADLINE_NOT_REACHED);
    warp(&mut env.ctx, DEADLINE).await;
    let at_deadline = refund_ix(JOB_ID, &env.mint, &env.buyer_token);
    assert_custom(send(&mut env.ctx, &[at_deadline], &[]).await, E_DEADLINE_NOT_REACHED);
    assert_eq!(snapshot(&mut env.ctx, &watched).await, before);

    warp(&mut env.ctx, DEADLINE + 1).await;
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

    let settled = snapshot(&mut env.ctx, &watched).await;
    let executor = env.executor.insecure_clone();
    let deliver_after_refund = deliver_ix(&executor.pubkey(), JOB_ID, delivered_artifact_hash());
    assert_custom(
        send(&mut env.ctx, &[deliver_after_refund], &[&executor]).await,
        E_ALREADY_REFUNDED,
    );
    assert_eq!(snapshot(&mut env.ctx, &watched).await, settled);
}

#[tokio::test]
async fn accounts_bound_to_the_job_reject_substitutes() {
    let mut env = setup().await;
    create_default_job(&mut env).await;
    fund_default_job(&mut env).await;
    let job = job_pda(&JOB_ID);
    let vault = vault_pda(&job);
    // A token account of the Job mint owned by the Job PDA, at another address.
    let fake_vault = create_token_account(&mut env.ctx, &env.mint, &job).await;
    // Past the deadline, so that only the substituted account is wrong. The
    // warp comes first: it verifies the bank capitalization, which an
    // injected account would change.
    warp(&mut env.ctx, DEADLINE + 1).await;
    // A byte-for-byte copy of the Job account owned by another program.
    let forged_job = Keypair::new().pubkey();
    let mut forged = env.ctx.banks_client.get_account(job).await.unwrap().unwrap();
    forged.owner = system_program::ID;
    env.ctx.set_account(&forged_job, &forged.into());

    let buyer = env.buyer.insecure_clone();
    let executor = env.executor.insecure_clone();
    let watched = [job, vault, fake_vault, forged_job, env.buyer_token];
    let before = snapshot(&mut env.ctx, &watched).await;

    let fund_fake_vault = substitute(
        fund_ix(&buyer.pubkey(), JOB_ID, &env.mint, &env.buyer_token, AMOUNT),
        &vault,
        &fake_vault,
    );
    assert_custom(send(&mut env.ctx, &[fund_fake_vault], &[&buyer]).await, ANCHOR_CONSTRAINT_SEEDS);
    let refund_fake_vault = substitute(refund_ix(JOB_ID, &env.mint, &env.buyer_token), &vault, &fake_vault);
    assert_custom(send(&mut env.ctx, &[refund_fake_vault], &[]).await, ANCHOR_CONSTRAINT_SEEDS);

    let forged_instructions = [
        (substitute(refund_ix(JOB_ID, &env.mint, &env.buyer_token), &job, &forged_job), None),
        (
            substitute(
                fund_ix(&buyer.pubkey(), JOB_ID, &env.mint, &env.buyer_token, AMOUNT),
                &job,
                &forged_job,
            ),
            Some(&buyer),
        ),
        (
            substitute(deliver_ix(&executor.pubkey(), JOB_ID, delivered_artifact_hash()), &job, &forged_job),
            Some(&executor),
        ),
    ];
    for (ix, signer) in forged_instructions {
        let signers: Vec<&Keypair> = signer.into_iter().collect();
        assert_custom(
            send(&mut env.ctx, &[ix], &signers).await,
            ANCHOR_ACCOUNT_OWNED_BY_WRONG_PROGRAM,
        );
    }
    assert_eq!(snapshot(&mut env.ctx, &watched).await, before);
}

#[tokio::test]
async fn unsupported_account_version_is_rejected() {
    let mut env = setup().await;
    create_default_job(&mut env).await;
    let job = job_pda(&JOB_ID);
    let mut account = env.ctx.banks_client.get_account(job).await.unwrap().unwrap();
    // `JobAccount::version` follows the 8-byte Anchor discriminator.
    assert_eq!(account.data[8], JOB_ACCOUNT_VERSION);
    account.data[8] = JOB_ACCOUNT_VERSION + 1;
    env.ctx.set_account(&job, &account.into());

    let watched = [job, vault_pda(&job), env.buyer_token];
    let before = snapshot(&mut env.ctx, &watched).await;
    let buyer = env.buyer.insecure_clone();
    let fund = fund_ix(&buyer.pubkey(), JOB_ID, &env.mint, &env.buyer_token, AMOUNT);
    assert_custom(send(&mut env.ctx, &[fund], &[&buyer]).await, E_UNSUPPORTED_ACCOUNT_VERSION);
    let executor = env.executor.insecure_clone();
    let deliver = deliver_ix(&executor.pubkey(), JOB_ID, delivered_artifact_hash());
    assert_custom(send(&mut env.ctx, &[deliver], &[&executor]).await, E_UNSUPPORTED_ACCOUNT_VERSION);
    assert_eq!(snapshot(&mut env.ctx, &watched).await, before);
}

#[tokio::test]
async fn create_job_compute_units() {
    let mut env = setup().await;
    // Relative to the Clock, so the same instruction also runs on a program
    // without the creation window (the D2c.1 build, for comparison).
    let deadline = current_slot(&mut env.ctx).await + 10_000;
    let buyer = env.buyer.insecure_clone();
    let ix = create_job_ix(&buyer.pubkey(), &env.mint, JOB_ID, env.executor.pubkey(), AMOUNT, deadline);
    let units = compute_units(&mut env.ctx, &[ix], &[&buyer]).await;
    println!("create_job compute units: {units}");
    assert!(units < 200_000, "create_job uses {units} compute units");
}

#[tokio::test]
async fn deliver_compute_units() {
    let mut env = setup().await;
    create_default_job(&mut env).await;
    fund_default_job(&mut env).await;
    let executor = env.executor.insecure_clone();
    let ix = deliver_ix(&executor.pubkey(), JOB_ID, delivered_artifact_hash());
    let units = compute_units(&mut env.ctx, &[ix], &[&executor]).await;
    println!("deliver compute units: {units}");
    assert!(units < 200_000, "deliver uses {units} compute units");
}
