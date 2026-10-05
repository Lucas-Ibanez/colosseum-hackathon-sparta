//! In-process tests of `release` and `refund_on_fail` with the real RISC Zero
//! Verifier Router and Groth16 verifier (`risc0-solana v3.0.0`, commit
//! `ee415935`) and the versioned Groth16 fixtures of Job `0x11`.
//!
//! `SBF_OUT_DIR` must hold `vericode_escrow.so`, `verifier_router.so` and
//! `groth_16_verifier.so`. The Router state and its Groth16 entry are created
//! in genesis with the layout of `verifier_router::state`, owned by an
//! in-memory owner; `verify` checks the entry, its selector and its verifier
//! program, not the upgrade authority that `add_verifier` checks (exercised in
//! the D2d spike). Negative cases run before the positive ones, and every
//! rejection leaves the Job, the vault and the token balances unchanged.

mod common;

use common::*;

// In-memory Router owner; no key is needed because no owner instruction runs.
const ROUTER_OWNER: Pubkey = Pubkey::new_from_array([0x0d; 32]);
const UNKNOWN_SELECTOR: [u8; 4] = [0xde, 0xad, 0xbe, 0xef];
// Margin over the measured cost (about 137 k units, below the default
// 200 k; see `settlement_fits_the_default_compute_limit`).
const COMPUTE_UNIT_LIMIT: u32 = 400_000;
// Groth16 verifier `VerifierError::VerificationError`, as observed in D2d.
const VERIFIER_VERIFICATION_ERROR: u32 = 6000;
// Groth16 verifier `VerifierError::PairingError`, as observed in D2d.
const VERIFIER_PAIRING_ERROR: u32 = 6003;
// Router `RouterError::SelectorDeactivated`.
const ROUTER_SELECTOR_DEACTIVATED: u32 = 6001;

fn account_discriminator(name: &str) -> [u8; 8] {
    sha256(format!("account:{name}").as_bytes()).to_bytes()[..8]
        .try_into()
        .unwrap()
}

fn add_data_account(program_test: &mut ProgramTest, address: Pubkey, owner: Pubkey, data: &[u8]) {
    let lamports = Rent::default().minimum_balance(data.len());
    program_test.add_account_with_base64_data(address, lamports, owner, &base64(data));
}

/// Escrow, Router and Groth16 verifier, with the Router state and the entry
/// of `GROTH16_SELECTOR` in genesis.
fn router_program_test(estopped: bool) -> ProgramTest {
    let mut program_test = program_test();
    program_test.add_program("verifier_router", VERIFIER_ROUTER_ID, None);
    program_test.add_program("groth_16_verifier", GROTH16_VERIFIER_ID, None);

    // `VerifierRouter { ownership: Ownership { owner, pending_owner } }` in
    // the 8 + 33 + 33 bytes that `initialize` allocates.
    let mut router = account_discriminator("VerifierRouter").to_vec();
    router.push(1);
    router.extend_from_slice(ROUTER_OWNER.as_ref());
    router.push(0);
    router.resize(8 + 33 + 33, 0);
    add_data_account(&mut program_test, ROUTER_PDA, VERIFIER_ROUTER_ID, &router);

    // `VerifierEntry { selector, verifier, estopped }`, 8 + 4 + 32 + 1 bytes.
    let mut entry = account_discriminator("VerifierEntry").to_vec();
    entry.extend_from_slice(&GROTH16_SELECTOR);
    entry.extend_from_slice(GROTH16_VERIFIER_ID.as_ref());
    entry.push(u8::from(estopped));
    add_data_account(&mut program_test, GROTH16_VERIFIER_ENTRY, VERIFIER_ROUTER_ID, &entry);
    program_test
}

struct Scenario {
    env: Env,
    executor_token: Pubkey,
}

impl Scenario {
    /// Job `0x11` created and funded, with the executor's associated token
    /// account, and the artifact of `delivered` (a fixture name) delivered.
    async fn new(delivered: Option<&str>, estopped: bool) -> Self {
        let mut env = start(router_program_test(estopped)).await;
        create_default_job(&mut env).await;
        fund_default_job(&mut env).await;
        if let Some(name) = delivered {
            deliver_artifact(&mut env, fixture(name).artifact_hash()).await;
        }
        let executor = env.executor.pubkey();
        let executor_token = create_ata(&mut env.ctx, &executor, &env.mint).await;
        Self { env, executor_token }
    }

    fn watched(&self) -> Vec<Pubkey> {
        let job = job_pda(&JOB_ID);
        vec![job, vault_pda(&job), self.env.buyer_token, self.executor_token]
    }

    async fn snapshot(&mut self) -> Vec<Option<(u64, Vec<u8>)>> {
        let watched = self.watched();
        snapshot(&mut self.env.ctx, &watched).await
    }

    fn release(&self, fixture: &Fixture) -> Instruction {
        release_ix(JOB_ID, &self.env.mint, &self.executor_token, fixture.journal.clone(), router_seal(fixture))
    }

    fn refund_on_fail(&self, fixture: &Fixture) -> Instruction {
        refund_on_fail_ix(JOB_ID, &self.env.mint, &self.env.buyer_token, fixture.journal.clone(), router_seal(fixture))
    }

    /// Asserts that `ix` fails in `program` with `code` and moves nothing.
    async fn reject(&mut self, ix: Instruction, program: &Pubkey, code: u32) -> Vec<String> {
        let before = self.snapshot().await;
        let logs = assert_failure(
            &mut self.env.ctx,
            &[compute_unit_limit_ix(COMPUTE_UNIT_LIMIT), ix],
            &[],
            program,
            code,
        )
        .await;
        assert_eq!(self.snapshot().await, before, "a rejection moved state");
        logs
    }

    /// Asserts a rejection by the escrow before any call to the Router.
    async fn reject_before_router(&mut self, ix: Instruction, code: u32) {
        let logs = self.reject(ix, &vericode_escrow::ID, code).await;
        assert!(!invoked(&logs, &VERIFIER_ROUTER_ID), "the Router must not be called: {logs:#?}");
    }

    async fn settle(&mut self, ix: Instruction) -> u64 {
        compute_units(&mut self.env.ctx, &[compute_unit_limit_ix(COMPUTE_UNIT_LIMIT), ix], &[]).await
    }
}

#[tokio::test]
async fn refund_on_fail_rejects_a_valid_fail_of_an_undelivered_artifact() {
    // R-D2 PoC-1 on-chain: the FAIL fixture is a real, verifiable proof for
    // Job 0x11, but of `(7,15)`, while the executor delivered `(7,14)`.
    let mut scenario = Scenario::new(Some("pass"), false).await;
    let fail = fixture("fail");
    let ix = scenario.refund_on_fail(&fail);
    scenario.reject_before_router(ix, E_JOURNAL_ARTIFACT_HASH_MISMATCH).await;
}

#[tokio::test]
async fn release_rejects_a_valid_pass_of_an_undelivered_artifact() {
    let mut scenario = Scenario::new(Some("fail"), false).await;
    let pass = fixture("pass");
    let ix = scenario.release(&pass);
    scenario.reject_before_router(ix, E_JOURNAL_ARTIFACT_HASH_MISMATCH).await;
}

#[tokio::test]
async fn settlement_by_verdict_requires_a_delivery() {
    let mut scenario = Scenario::new(None, false).await;
    let release = scenario.release(&fixture("pass"));
    scenario.reject_before_router(release, E_NOT_DELIVERED).await;
    let refund = scenario.refund_on_fail(&fixture("fail"));
    scenario.reject_before_router(refund, E_NOT_DELIVERED).await;
}

#[tokio::test]
async fn malformed_journals_are_rejected() {
    let mut scenario = Scenario::new(Some("pass"), false).await;
    let pass = fixture("pass");
    let seal = router_seal(&pass);
    let mut truncated = pass.journal.clone();
    truncated.pop();
    let mut extended = pass.journal.clone();
    extended.push(0);
    let mut unknown_verdict = pass.journal.clone();
    unknown_verdict[164] = 2;

    for journal in [truncated, extended, unknown_verdict] {
        let ix = release_ix(JOB_ID, &scenario.env.mint, &scenario.executor_token, journal, seal);
        scenario.reject_before_router(ix, E_JOURNAL_MALFORMED).await;
    }
}

#[tokio::test]
async fn each_instruction_accepts_only_its_verdict() {
    let mut delivered_fail = Scenario::new(Some("fail"), false).await;
    let fail = fixture("fail");
    let release_fail = release_ix(
        JOB_ID,
        &delivered_fail.env.mint,
        &delivered_fail.executor_token,
        fail.journal.clone(),
        router_seal(&fail),
    );
    delivered_fail.reject_before_router(release_fail, E_VERDICT_NOT_PASS).await;

    let mut delivered_pass = Scenario::new(Some("pass"), false).await;
    let pass = fixture("pass");
    let refund_pass = refund_on_fail_ix(
        JOB_ID,
        &delivered_pass.env.mint,
        &delivered_pass.env.buyer_token,
        pass.journal.clone(),
        router_seal(&pass),
    );
    delivered_pass.reject_before_router(refund_pass, E_VERDICT_NOT_FAIL).await;
}

#[tokio::test]
async fn release_after_the_deadline_is_rejected() {
    let mut scenario = Scenario::new(Some("pass"), false).await;
    warp(&mut scenario.env.ctx, DEADLINE + 1).await;
    let ix = scenario.release(&fixture("pass"));
    scenario.reject_before_router(ix, E_DEADLINE_PASSED).await;
    assert_eq!(current_slot(&mut scenario.env.ctx).await, DEADLINE + 1);
}

#[tokio::test]
async fn settlement_pays_only_the_canonical_account_of_the_paid_party() {
    let mut scenario = Scenario::new(Some("pass"), false).await;
    let pass = fixture("pass");
    let seal = router_seal(&pass);
    let executor = scenario.env.executor.pubkey();
    let mint = scenario.env.mint;
    // Another account of the executor with the Job mint (R-D2 F-06).
    let non_canonical = create_token_account(&mut scenario.env.ctx, &mint, &executor).await;
    // The executor's canonical account of another mint.
    let other_authority = Keypair::new();
    let other_mint = create_mint(&mut scenario.env.ctx, &other_authority.pubkey()).await;
    let other_mint_ata = create_ata(&mut scenario.env.ctx, &executor, &other_mint).await;

    let cases = [
        (release_ix(JOB_ID, &mint, &non_canonical, pass.journal.clone(), seal), E_DESTINATION_NOT_CANONICAL),
        (release_ix(JOB_ID, &mint, &other_mint_ata, pass.journal.clone(), seal), E_MINT_MISMATCH),
        // A foreign mint account is bound to the Job mint (R-D2 F-08).
        (
            release_ix(JOB_ID, &other_mint, &scenario.executor_token, pass.journal.clone(), seal),
            E_MINT_MISMATCH,
        ),
        // The buyer is not the party a `Pass` pays.
        (
            release_ix(JOB_ID, &mint, &scenario.env.buyer_token, pass.journal.clone(), seal),
            E_RECIPIENT_MISMATCH,
        ),
    ];
    for (ix, code) in cases {
        scenario.reject_before_router(ix, code).await;
    }
}

#[tokio::test]
async fn seals_of_another_selector_are_rejected_by_the_escrow() {
    // R-D2 F-03: a verifier registered by the Router owner under another
    // selector can never decide a VeriCode settlement.
    let mut scenario = Scenario::new(Some("pass"), false).await;
    let pass = fixture("pass");
    let mut seal = router_seal(&pass);
    seal.selector = UNKNOWN_SELECTOR;
    let ix = release_ix(JOB_ID, &scenario.env.mint, &scenario.executor_token, pass.journal.clone(), seal);
    scenario.reject_before_router(ix, E_UNEXPECTED_SELECTOR).await;
}

#[tokio::test]
async fn router_accounts_are_fixed() {
    let mut scenario = Scenario::new(Some("pass"), false).await;
    let pass = fixture("pass");
    let unknown_entry =
        Pubkey::find_program_address(&[b"verifier", UNKNOWN_SELECTOR.as_ref()], &VERIFIER_ROUTER_ID).0;
    let substitutes = [
        (VERIFIER_ROUTER_ID, spl_token::ID),
        (ROUTER_PDA, Pubkey::new_unique()),
        (GROTH16_VERIFIER_ENTRY, unknown_entry),
        (GROTH16_VERIFIER_ID, spl_token::ID),
    ];
    for (original, replacement) in substitutes {
        let ix = substitute(scenario.release(&pass), &original, &replacement);
        scenario.reject_before_router(ix, ANCHOR_CONSTRAINT_ADDRESS).await;
    }
}

#[tokio::test]
async fn invalid_proofs_are_rejected_by_the_verifier() {
    let mut scenario = Scenario::new(Some("pass"), false).await;
    let pass = fixture("pass");
    let fail = fixture("fail");

    let mut tampered = router_seal(&pass);
    tampered.pi_c[10] ^= 0x01;
    let tampered_ix =
        release_ix(JOB_ID, &scenario.env.mint, &scenario.executor_token, pass.journal.clone(), tampered);
    let logs = scenario.reject(tampered_ix, &GROTH16_VERIFIER_ID, VERIFIER_PAIRING_ERROR).await;
    assert!(invoked(&logs, &VERIFIER_ROUTER_ID));

    // A valid seal of the other journal: the claim does not match the bytes.
    let swapped_ix =
        release_ix(JOB_ID, &scenario.env.mint, &scenario.executor_token, pass.journal.clone(), router_seal(&fail));
    scenario.reject(swapped_ix, &GROTH16_VERIFIER_ID, VERIFIER_VERIFICATION_ERROR).await;
}

#[tokio::test]
async fn an_emergency_stop_blocks_proofs_but_not_the_timeout() {
    // Residual trust in the Router owner: a stopped selector freezes
    // settlement by verdict, and the timeout still returns the funds.
    let mut scenario = Scenario::new(Some("pass"), true).await;
    let ix = scenario.release(&fixture("pass"));
    let logs = scenario.reject(ix, &VERIFIER_ROUTER_ID, ROUTER_SELECTOR_DEACTIVATED).await;
    assert!(!invoked(&logs, &GROTH16_VERIFIER_ID));

    warp(&mut scenario.env.ctx, DEADLINE + 1).await;
    let refund = refund_ix(JOB_ID, &scenario.env.mint, &scenario.env.buyer_token);
    send(&mut scenario.env.ctx, &[refund], &[]).await.unwrap();
    assert_eq!(
        token_balance(&mut scenario.env.ctx, &scenario.env.buyer_token.clone()).await,
        BUYER_START_BALANCE
    );
}

#[tokio::test]
async fn release_pays_the_executor_once() {
    let mut scenario = Scenario::new(Some("pass"), false).await;
    let pass = fixture("pass");
    let fail = fixture("fail");
    let job = job_pda(&JOB_ID);

    let ix = scenario.release(&pass);
    scenario.settle(ix).await;
    assert_eq!(
        read_job(&mut scenario.env.ctx, &job).await.status,
        EscrowStatus::Released {
            artifact_hash: pass.artifact_hash()
        }
    );
    assert_eq!(token_balance(&mut scenario.env.ctx, &vault_pda(&job)).await, 0);
    assert_eq!(token_balance(&mut scenario.env.ctx, &scenario.executor_token.clone()).await, AMOUNT);
    assert_eq!(
        token_balance(&mut scenario.env.ctx, &scenario.env.buyer_token.clone()).await,
        BUYER_START_BALANCE - AMOUNT
    );

    // Double settlement and late operations.
    let again = scenario.release(&pass);
    scenario.reject_before_router(again, E_ALREADY_RELEASED).await;
    let refund_fail = scenario.refund_on_fail(&fail);
    scenario.reject_before_router(refund_fail, E_ALREADY_RELEASED).await;
    let executor = scenario.env.executor.insecure_clone();
    let before = scenario.snapshot().await;
    let deliver = deliver_ix(&executor.pubkey(), JOB_ID, fail.artifact_hash());
    assert_custom(send(&mut scenario.env.ctx, &[deliver], &[&executor]).await, E_ALREADY_RELEASED);
    warp(&mut scenario.env.ctx, DEADLINE + 1).await;
    let timeout = refund_ix(JOB_ID, &scenario.env.mint, &scenario.env.buyer_token);
    assert_custom(send(&mut scenario.env.ctx, &[timeout], &[]).await, E_ALREADY_RELEASED);
    assert_eq!(scenario.snapshot().await, before);
}

#[tokio::test]
async fn refund_on_fail_returns_the_amount_to_the_buyer_before_the_deadline() {
    let mut scenario = Scenario::new(Some("fail"), false).await;
    let fail = fixture("fail");
    let job = job_pda(&JOB_ID);

    let ix = scenario.refund_on_fail(&fail);
    scenario.settle(ix).await;
    assert_eq!(
        read_job(&mut scenario.env.ctx, &job).await.status,
        EscrowStatus::RefundedOnFail {
            artifact_hash: fail.artifact_hash()
        }
    );
    assert_eq!(token_balance(&mut scenario.env.ctx, &vault_pda(&job)).await, 0);
    assert_eq!(
        token_balance(&mut scenario.env.ctx, &scenario.env.buyer_token.clone()).await,
        BUYER_START_BALANCE
    );
    assert_eq!(token_balance(&mut scenario.env.ctx, &scenario.executor_token.clone()).await, 0);

    let release = scenario.release(&fixture("pass"));
    scenario.reject_before_router(release, E_ALREADY_REFUNDED).await;
}

#[tokio::test]
async fn refund_on_fail_is_accepted_after_the_deadline() {
    let mut scenario = Scenario::new(Some("fail"), false).await;
    warp(&mut scenario.env.ctx, DEADLINE + 1).await;
    let ix = scenario.refund_on_fail(&fixture("fail"));
    scenario.settle(ix).await;
    assert_eq!(
        token_balance(&mut scenario.env.ctx, &scenario.env.buyer_token.clone()).await,
        BUYER_START_BALANCE
    );
}

#[tokio::test]
async fn release_after_a_timeout_refund_is_rejected() {
    let mut scenario = Scenario::new(Some("pass"), false).await;
    warp(&mut scenario.env.ctx, DEADLINE + 1).await;
    let refund = refund_ix(JOB_ID, &scenario.env.mint, &scenario.env.buyer_token);
    send(&mut scenario.env.ctx, &[refund], &[]).await.unwrap();

    let release = scenario.release(&fixture("pass"));
    scenario.reject_before_router(release, E_ALREADY_REFUNDED).await;
}

#[tokio::test]
async fn settlement_fits_the_default_compute_limit() {
    // Without a Compute Budget instruction: one instruction gets 200 000 units.
    let mut released = Scenario::new(Some("pass"), false).await;
    let release = released.release(&fixture("pass"));
    let release_units = compute_units(&mut released.env.ctx, &[release], &[]).await;
    let mut refunded = Scenario::new(Some("fail"), false).await;
    let refund = refunded.refund_on_fail(&fixture("fail"));
    let refund_units = compute_units(&mut refunded.env.ctx, &[refund], &[]).await;
    println!("release compute units: {release_units}");
    println!("refund_on_fail compute units: {refund_units}");
    assert!(release_units < 200_000, "release uses {release_units} compute units");
    assert!(refund_units < 200_000, "refund_on_fail uses {refund_units} compute units");
}
