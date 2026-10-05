//! Regression tests of the R-D2e adversarial PoCs 1 to 7 (gate D4a, R-05).
//!
//! Each test asserts the result the review observed with the D2e program,
//! adapted to the D4a account set (Groth16 verifier called directly, no
//! Verifier Router; admitted Test USDC mint). PoC-8 (replay across two
//! deployments) needs a second program ID and stays in the R-D2e report.

mod common;

use anchor_spl::token::spl_token::instruction::AuthorityType;
use common::*;

const CU: u32 = 400_000;
const ANCHOR_ACCOUNT_NOT_INITIALIZED: u32 = 3012;
const ANCHOR_CONSTRAINT_MUT: u32 = 2000;
const PACKET_DATA_SIZE: usize = 1232;

/// Job 0x11 created and funded; `delivered` fixture delivered if given; the
/// executor ATA is created only when `executor_ata` is true.
async fn scenario(delivered: Option<&str>, executor_ata: bool) -> (Env, Pubkey) {
    let mut env = start(verifier_program_test()).await;
    create_default_job(&mut env).await;
    fund_default_job(&mut env).await;
    if let Some(name) = delivered {
        deliver_artifact(&mut env, fixture(name).artifact_hash()).await;
    }
    let executor = env.executor.pubkey();
    let executor_token = if executor_ata {
        create_ata(&mut env.ctx, &executor, &env.mint).await
    } else {
        ata_address(&executor, &env.mint)
    };
    (env, executor_token)
}

fn watched(env: &Env, executor_token: &Pubkey) -> Vec<Pubkey> {
    let job = job_pda(&JOB_ID);
    vec![job, vault_pda(&job), env.buyer_token, *executor_token]
}

/// Sends `ixs`, asserts the custom error `code` and an unchanged snapshot of
/// `keys`.
async fn expect_code(env: &mut Env, ixs: &[Instruction], signers: &[&Keypair], code: u32, keys: &[Pubkey]) {
    let before = snapshot(&mut env.ctx, keys).await;
    assert_custom(send(&mut env.ctx, ixs, signers).await, code);
    assert_eq!(snapshot(&mut env.ctx, keys).await, before, "a rejection moved state");
}

fn release(env: &Env, to: &Pubkey, name: &str) -> Instruction {
    let f = fixture(name);
    release_ix(JOB_ID, &env.mint, to, f.journal.clone(), groth16_seal(&f))
}

fn refund_fail(env: &Env, name: &str) -> Instruction {
    let f = fixture(name);
    refund_on_fail_ix(JOB_ID, &env.mint, &env.buyer_token, f.journal.clone(), groth16_seal(&f))
}

fn wire_size(tx: &Transaction) -> usize {
    // shortvec(signature count) + signatures + serialized message.
    1 + 64 * tx.signatures.len() + tx.message_data().len()
}

// PoC-1: owner reassignment of the canonical ATA (classic SPL Token).
#[tokio::test]
async fn poc1_ata_owner_reassignment_never_redirects_funds() {
    let (mut env, executor_token) = scenario(Some("pass"), true).await;
    let executor = env.executor.insecure_clone();
    let other = Keypair::new();
    let set = spl_token::instruction::set_authority(
        &spl_token::ID,
        &executor_token,
        Some(&other.pubkey()),
        AuthorityType::AccountOwner,
        &executor.pubkey(),
        &[],
    )
    .unwrap();
    // The classic SPL Token program accepts the owner change of an ATA.
    send(&mut env.ctx, &[set], &[&executor]).await.unwrap();
    let keys = watched(&env, &executor_token);
    let ix = release(&env, &executor_token, "pass");
    expect_code(&mut env, &[compute_unit_limit_ix(CU), ix], &[], E_RECIPIENT_MISMATCH, &keys).await;

    // Restored by the new owner, the release pays the executor.
    let back = spl_token::instruction::set_authority(
        &spl_token::ID,
        &executor_token,
        Some(&executor.pubkey()),
        AuthorityType::AccountOwner,
        &other.pubkey(),
        &[],
    )
    .unwrap();
    send(&mut env.ctx, &[back], &[&other]).await.unwrap();
    let ix = release(&env, &executor_token, "pass");
    send(&mut env.ctx, &[compute_unit_limit_ix(CU), ix], &[]).await.unwrap();
    assert_eq!(token_balance(&mut env.ctx, &executor_token).await, AMOUNT);

    // Buyer side: a buyer that reassigns its own ATA cannot be refunded there.
    let (mut env, executor_token) = scenario(Some("pass"), true).await;
    let buyer = env.buyer.insecure_clone();
    let set = spl_token::instruction::set_authority(
        &spl_token::ID,
        &env.buyer_token,
        Some(&other.pubkey()),
        AuthorityType::AccountOwner,
        &buyer.pubkey(),
        &[],
    )
    .unwrap();
    send(&mut env.ctx, &[set], &[&buyer]).await.unwrap();
    warp(&mut env.ctx, DEADLINE + 1).await;
    let keys = watched(&env, &executor_token);
    let ix = refund_ix(JOB_ID, &env.mint, &env.buyer_token);
    expect_code(&mut env, &[ix], &[], E_RECIPIENT_MISMATCH, &keys).await;
}

// PoC-2: missing canonical ATA, created afterwards by an unrelated payer.
#[tokio::test]
async fn poc2_missing_ata_blocks_until_anyone_creates_it() {
    let (mut env, executor_token) = scenario(Some("pass"), false).await;
    let keys = watched(&env, &executor_token);
    let ix = release(&env, &executor_token, "pass");
    expect_code(&mut env, &[compute_unit_limit_ix(CU), ix], &[], ANCHOR_ACCOUNT_NOT_INITIALIZED, &keys).await;
    let executor = env.executor.pubkey();
    let mint = env.mint;
    // Paid by the test payer, not by the executor.
    create_ata(&mut env.ctx, &executor, &mint).await;
    let ix = release(&env, &executor_token, "pass");
    send(&mut env.ctx, &[compute_unit_limit_ix(CU), ix], &[]).await.unwrap();
    assert_eq!(token_balance(&mut env.ctx, &executor_token).await, AMOUNT);

    // The buyer closes its ATA; the timeout is blocked until anyone recreates it.
    let (mut env, executor_token) = scenario(None, true).await;
    let buyer = env.buyer.insecure_clone();
    let sink = create_token_account(&mut env.ctx, &env.mint, &buyer.pubkey()).await;
    let rest = token_balance(&mut env.ctx, &env.buyer_token).await;
    let drain =
        spl_token::instruction::transfer(&spl_token::ID, &env.buyer_token, &sink, &buyer.pubkey(), &[], rest).unwrap();
    let close = spl_token::instruction::close_account(
        &spl_token::ID,
        &env.buyer_token,
        &buyer.pubkey(),
        &buyer.pubkey(),
        &[],
    )
    .unwrap();
    send(&mut env.ctx, &[drain, close], &[&buyer]).await.unwrap();
    warp(&mut env.ctx, DEADLINE + 1).await;
    let job = job_pda(&JOB_ID);
    let keys = vec![job, vault_pda(&job), executor_token];
    let ix = refund_ix(JOB_ID, &env.mint, &env.buyer_token);
    expect_code(&mut env, &[ix], &[], ANCHOR_ACCOUNT_NOT_INITIALIZED, &keys).await;
    let buyer_key = buyer.pubkey();
    let mint = env.mint;
    create_ata(&mut env.ctx, &buyer_key, &mint).await;
    let ix = refund_ix(JOB_ID, &env.mint, &env.buyer_token);
    send(&mut env.ctx, &[ix], &[]).await.unwrap();
    assert_eq!(token_balance(&mut env.ctx, &env.buyer_token.clone()).await, AMOUNT);
}

// PoC-3: settlement at exactly Clock.slot == deadline.
#[tokio::test]
async fn poc3_settlement_at_the_deadline_slot() {
    let (mut env, executor_token) = scenario(Some("pass"), true).await;
    warp(&mut env.ctx, DEADLINE).await;
    let keys = watched(&env, &executor_token);
    let timeout = refund_ix(JOB_ID, &env.mint, &env.buyer_token);
    expect_code(&mut env, &[timeout], &[], E_DEADLINE_NOT_REACHED, &keys).await;
    let ix = release(&env, &executor_token, "pass");
    send(&mut env.ctx, &[compute_unit_limit_ix(CU), ix], &[]).await.unwrap();
    assert_eq!(current_slot(&mut env.ctx).await, DEADLINE);
    assert_eq!(token_balance(&mut env.ctx, &executor_token).await, AMOUNT);

    // Deliver and release atomically in one transaction at the deadline.
    let (mut env, executor_token) = scenario(None, true).await;
    warp(&mut env.ctx, DEADLINE).await;
    let executor = env.executor.insecure_clone();
    let pass = fixture("pass");
    let deliver = deliver_ix(&executor.pubkey(), JOB_ID, pass.artifact_hash());
    let ix = release(&env, &executor_token, "pass");
    send(&mut env.ctx, &[compute_unit_limit_ix(CU), deliver, ix], &[&executor]).await.unwrap();
    assert_eq!(current_slot(&mut env.ctx).await, DEADLINE);
    assert_eq!(
        read_job(&mut env.ctx, &job_pda(&JOB_ID)).await.status,
        EscrowStatus::Released {
            artifact_hash: pass.artifact_hash()
        }
    );
}

// PoC-4: funding is not bounded by the deadline window. This documents the
// known limitation R-02 (mitigated off-chain by create_job+fund in one
// transaction), not a desired property.
#[tokio::test]
async fn poc4_funding_after_or_just_before_the_deadline() {
    let mut env = start(verifier_program_test()).await;
    create_default_job(&mut env).await;
    warp(&mut env.ctx, DEADLINE + 1).await;
    fund_default_job(&mut env).await;
    let job = job_pda(&JOB_ID);
    assert_eq!(read_job(&mut env.ctx, &job).await.status, EscrowStatus::Funded);
    let executor = env.executor.insecure_clone();
    let keys = vec![job, vault_pda(&job), env.buyer_token];
    let deliver = deliver_ix(&executor.pubkey(), JOB_ID, delivered_artifact_hash());
    expect_code(&mut env, &[deliver], &[&executor], E_DEADLINE_PASSED, &keys).await;
    let ix = refund_ix(JOB_ID, &env.mint, &env.buyer_token);
    send(&mut env.ctx, &[ix], &[]).await.unwrap();
    assert_eq!(token_balance(&mut env.ctx, &env.buyer_token.clone()).await, BUYER_START_BALANCE);

    let mut env = start(verifier_program_test()).await;
    create_default_job(&mut env).await;
    warp(&mut env.ctx, DEADLINE - 1).await;
    fund_default_job(&mut env).await;
    // Slots DEADLINE - 1 and DEADLINE remain to deliver and release.
    assert_eq!(DEADLINE + 1 - current_slot(&mut env.ctx).await, 2);
}

// PoC-5: transaction sizes against the 1232-byte packet limit. Three fewer
// accounts than in D2e (no Router, router PDA nor verifier entry), so each
// case is 99 bytes smaller than in R-D2e (937, 977, 1078, 1118).
#[tokio::test]
async fn poc5_transaction_sizes() {
    let (mut env, executor_token) = scenario(None, true).await;
    let blockhash = env.ctx.get_new_latest_blockhash().await.unwrap();
    let executor = env.executor.insecure_clone();
    let deliver = deliver_ix(&executor.pubkey(), JOB_ID, fixture("pass").artifact_hash());
    let rel = release(&env, &executor_token, "pass");
    assert_eq!(rel.data.len(), 437);
    assert_eq!(rel.accounts.len(), 7);
    let cases: Vec<(Vec<Instruction>, Vec<&Keypair>, usize)> = vec![
        (vec![rel.clone()], vec![], 838),
        (vec![compute_unit_limit_ix(CU), rel.clone()], vec![], 878),
        (vec![deliver.clone(), rel.clone()], vec![&executor], 979),
        (vec![compute_unit_limit_ix(CU), deliver.clone(), rel.clone()], vec![&executor], 1019),
    ];
    for (ixs, signers, expected) in cases {
        let tx = signed_transaction(&env.ctx, &ixs, &signers, blockhash);
        assert_eq!(wire_size(&tx), expected);
        assert!(wire_size(&tx) <= PACKET_DATA_SIZE);
    }
    let tx = signed_transaction(&env.ctx, &[compute_unit_limit_ix(CU), deliver, rel], &[&executor], blockhash);
    env.ctx.banks_client.process_transaction(tx).await.unwrap();
    assert!(matches!(
        read_job(&mut env.ctx, &job_pda(&JOB_ID)).await.status,
        EscrowStatus::Released { .. }
    ));
}

// PoC-6: every terminal state rejects every instruction, on-chain.
#[tokio::test]
async fn poc6_terminal_states_reject_every_instruction() {
    for (terminal, code) in [
        ("Released", E_ALREADY_RELEASED),
        ("RefundedOnFail", E_ALREADY_REFUNDED),
        ("RefundedOnTimeout", E_ALREADY_REFUNDED),
    ] {
        let delivered = if terminal == "RefundedOnFail" { "fail" } else { "pass" };
        let (mut env, executor_token) = scenario(Some(delivered), true).await;
        match terminal {
            "Released" => {
                let ix = release(&env, &executor_token, "pass");
                send(&mut env.ctx, &[compute_unit_limit_ix(CU), ix], &[]).await.unwrap();
            }
            "RefundedOnFail" => {
                let ix = refund_fail(&env, "fail");
                send(&mut env.ctx, &[compute_unit_limit_ix(CU), ix], &[]).await.unwrap();
            }
            _ => {
                warp(&mut env.ctx, DEADLINE + 1).await;
                let ix = refund_ix(JOB_ID, &env.mint, &env.buyer_token);
                send(&mut env.ctx, &[ix], &[]).await.unwrap();
            }
        }
        let keys = watched(&env, &executor_token);
        let buyer = env.buyer.insecure_clone();
        let executor = env.executor.insecure_clone();
        let fund = fund_ix(&buyer.pubkey(), JOB_ID, &env.mint, &env.buyer_token, AMOUNT);
        expect_code(&mut env, &[fund], &[&buyer], code, &keys).await;
        let deliver = deliver_ix(&executor.pubkey(), JOB_ID, fixture("fail").artifact_hash());
        expect_code(&mut env, &[deliver], &[&executor], code, &keys).await;
        let ix = release(&env, &executor_token, "pass");
        expect_code(&mut env, &[compute_unit_limit_ix(CU), ix], &[], code, &keys).await;
        let ix = refund_fail(&env, "fail");
        expect_code(&mut env, &[compute_unit_limit_ix(CU), ix], &[], code, &keys).await;
        if current_slot(&mut env.ctx).await <= DEADLINE {
            warp(&mut env.ctx, DEADLINE + 1).await;
        }
        let ix = refund_ix(JOB_ID, &env.mint, &env.buyer_token);
        expect_code(&mut env, &[ix], &[], code, &keys).await;
        let slot = current_slot(&mut env.ctx).await;
        let create = create_job_ix(&buyer.pubkey(), &env.mint, JOB_ID, executor.pubkey(), AMOUNT, slot + MIN_WINDOW);
        expect_code(&mut env, &[create], &[&buyer], SYSTEM_ACCOUNT_ALREADY_IN_USE, &keys).await;
    }
}

// PoC-7: forged Job, fake vault, Token-2022 destination and substituted
// system program in release; a non-writable Job meta.
#[tokio::test]
async fn poc7_settlement_account_substitutions() {
    let (mut env, executor_token) = scenario(Some("pass"), true).await;
    let job = job_pda(&JOB_ID);
    let vault = vault_pda(&job);
    let fake_vault = create_token_account(&mut env.ctx, &env.mint, &job).await;
    let t22_authority = Keypair::new();
    let t22_mint = create_token_2022_mint(&mut env.ctx, &t22_authority.pubkey()).await;
    let executor = env.executor.pubkey();
    let t22_account = {
        let account = Keypair::new();
        let rent = env.ctx.banks_client.get_rent().await.unwrap();
        let create = system_instruction::create_account(
            &env.ctx.payer.pubkey(),
            &account.pubkey(),
            rent.minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN as u64,
            &spl_token_2022::ID,
        );
        let init =
            spl_token_2022::instruction::initialize_account3(&spl_token_2022::ID, &account.pubkey(), &t22_mint, &executor)
                .unwrap();
        send(&mut env.ctx, &[create, init], &[&account]).await.unwrap();
        account.pubkey()
    };
    // A byte-for-byte copy of the Job account owned by another program.
    let forged_job = Keypair::new().pubkey();
    let mut forged = env.ctx.banks_client.get_account(job).await.unwrap().unwrap();
    forged.owner = system_program::ID;
    env.ctx.set_account(&forged_job, &forged.into());

    let keys = vec![job, vault, env.buyer_token, executor_token, fake_vault];
    let base = release(&env, &executor_token, "pass");
    let fake_system = Pubkey::new_unique();
    let mut readonly_job = base.clone();
    assert_eq!(readonly_job.accounts[0].pubkey, job);
    readonly_job.accounts[0].is_writable = false;
    let cases = [
        (substitute(base.clone(), &job, &forged_job), ANCHOR_ACCOUNT_OWNED_BY_WRONG_PROGRAM),
        (substitute(base.clone(), &vault, &fake_vault), ANCHOR_CONSTRAINT_SEEDS),
        (substitute(base.clone(), &executor_token, &t22_account), ANCHOR_ACCOUNT_OWNED_BY_WRONG_PROGRAM),
        (substitute(base.clone(), &system_program::ID, &fake_system), ANCHOR_INVALID_PROGRAM_ID),
        (readonly_job, ANCHOR_CONSTRAINT_MUT),
    ];
    for (ix, code) in cases {
        expect_code(&mut env, &[compute_unit_limit_ix(CU), ix], &[], code, &keys).await;
    }
}
