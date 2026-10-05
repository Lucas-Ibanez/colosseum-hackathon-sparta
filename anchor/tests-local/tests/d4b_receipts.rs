//! The D4b devnet receipts as versioned fixtures (R-D4a RD4A-07 f).
//!
//! `fixtures/groth16/d4b/` holds the public vectors of the receipts that
//! settled Jobs S, A and B on devnet (and A′, the FAIL of Job A that was
//! never delivered). These tests bind each vector to its devnet Job and replay
//! the D4b settlements in process, with the real Groth16 verifier: negatives
//! first, then the settlements, then the double settlements of invariant 9.
//! `SBF_OUT_DIR` must hold `vericode_escrow.so` and `groth_16_verifier.so`.

mod common;

use common::*;
use vericode_core::{
    hash_harness_version, hash_restricted_spec, ImageId, JobId, JournalV1Commitments, Verdict,
    DETERMINISTIC_HARNESS_VERSION, RESTRICTED_SPEC_V1,
};

const CU: u32 = 400_000;
const VERIFIER_VERIFICATION_ERROR: u32 = 6000;
const VERIFIER_PAIRING_ERROR: u32 = 6003;
const E_JOURNAL_JOB_ID_MISMATCH: u32 = 6014;
const JOB_S: &str = "fe6d25fe77784e80d64ddd99b95fc0095011355cafbf93beac6cea678ce0959b";
const JOB_A: &str = "3f0dd1c833714e193744c5b95879662c1d8b440cdd3c6467a1585ecc60baec8a";
const JOB_B: &str = "5a25ae4808b6da928f57c8787c4fbb173791239d18342b540d4c485647fbc309";

/// Name, devnet `job_id`, claimed output of `(7, _)`, verdict, SHA-256 of the
/// journal and of the seal (as in `d4/logs/receipts-out.sha256`).
const VECTORS: [(&str, &str, u32, Verdict, &str, &str); 4] = [
    ("S", JOB_S, 14, Verdict::Pass,
     "20b3353cf3ba872e6bdb7c62e4685545f3784dd76c396ee3431bc6176e26c0b1",
     "4f1131c29b091faf285bb95720d42a477565eb6bc9d1156c66309d0157de98ef"),
    ("A", JOB_A, 14, Verdict::Pass,
     "a1060efe594b409e934d338987adcbc8fe2b864b49715fe1b0f33ae7a9dc5f47",
     "610704b1bb2be5d49f5626718990b73393e9bb85ded982533dcbaa8b02da8cf0"),
    ("A-fail", JOB_A, 15, Verdict::Fail,
     "e04f62d92a2a6c40e6f3da7ab3f1ad3257523175959393313069a117402c163c",
     "7c5962252392c005093dbbcb8cfe91000864413afeed20ec979adeb361c85036"),
    ("B", JOB_B, 15, Verdict::Fail,
     "e931596b57fec1f96512d19a9f5c5d5baf214243484e8b20315d9d5245ec7741",
     "462c4855b78897b44e0f3fccd1f2bff35a68ac1c83a5430c44ca5dbfc395d63f"),
];

fn to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn d4b(name: &str) -> Fixture {
    fixture(&format!("d4b/{name}"))
}

fn job_of(fixture: &Fixture) -> [u8; 32] {
    JournalV1::decode_candidate(&fixture.journal)
        .unwrap()
        .job_id()
        .as_hash()
        .into_bytes()
}

#[test]
fn d4b_vectors_are_bound_to_their_devnet_jobs() {
    for (name, job_id, claimed_output, verdict, journal_sha, seal_sha) in VECTORS {
        let fixture = d4b(name);
        assert_eq!(fixture.selector, GROTH16_SELECTOR, "{name}: selector");
        assert_eq!(to_hex(&fixture.image_id), IMAGE_ID_HEX, "{name}: image_id");
        assert_eq!(fixture.journal.len(), 165, "{name}: journal length");
        assert_eq!(sha256(&fixture.journal).to_bytes(), fixture.journal_digest, "{name}: journal_digest");
        assert_eq!(to_hex(&sha256(&fixture.journal).to_bytes()), journal_sha, "{name}: journal hash");
        assert_eq!(to_hex(&sha256(&fixture.seal).to_bytes()), seal_sha, "{name}: seal hash");
        let journal = JournalV1::decode_candidate(&fixture.journal).unwrap();
        let expected = JournalV1Commitments::new(
            JobId::new(hex32(job_id)),
            hash_restricted_spec(&RESTRICTED_SPEC_V1).unwrap(),
            hash_harness_version(DETERMINISTIC_HARNESS_VERSION).unwrap(),
            hash_restricted_artifact(&RestrictedArtifactV1::new(7, claimed_output)).unwrap(),
            ImageId::new(fixture.image_id),
        );
        assert_eq!(journal.validate_against(&expected), Ok(verdict), "{name}: binding");
        assert_ne!(job_of(&fixture), JOB_ID, "{name}: devnet Jobs never use the 0x11 fixture id");
    }
}

/// Simulates and sends `instructions`, expecting `code` from `program`, and
/// checks that the watched accounts did not change.
async fn reject(
    env: &mut Env,
    label: &str,
    instructions: &[Instruction],
    signers: &[&Keypair],
    watched: &[Pubkey],
    program: &Pubkey,
    code: u32,
) -> Vec<String> {
    let before = snapshot(&mut env.ctx, watched).await;
    let logs = assert_failure(&mut env.ctx, instructions, signers, program, code).await;
    assert_eq!(snapshot(&mut env.ctx, watched).await, before, "{label}: a rejection moved state");
    logs
}

#[tokio::test]
async fn d4b_settlements_replay_with_negatives_and_double_settlements() {
    let mut env = start(verifier_program_test()).await;
    let s = d4b("S");
    let a = d4b("A");
    let a_fail = d4b("A-fail");
    let b = d4b("B");
    let (job_s, job_a, job_b) = (job_of(&s), job_of(&a), job_of(&b));
    let job_c: [u8; 32] = sha256(b"D7 replay of Job C (local only)").to_bytes();
    let buyer = env.buyer.insecure_clone();
    let executor = env.executor.insecure_clone();
    let mint = env.mint;
    let buyer_token = env.buyer_token;
    let slot = current_slot(&mut env.ctx).await;
    for (job_id, deadline) in [(job_s, slot + 3_000), (job_a, slot + 3_000), (job_b, slot + 3_000), (job_c, slot + MIN_WINDOW + 100)] {
        let create = create_job_ix(&buyer.pubkey(), &mint, job_id, executor.pubkey(), AMOUNT, deadline);
        let fund = fund_ix(&buyer.pubkey(), job_id, &mint, &buyer_token, AMOUNT);
        send(&mut env.ctx, &[create, fund], &[&buyer]).await.unwrap();
    }
    let executor_token = create_ata(&mut env.ctx, &executor.pubkey(), &mint).await;
    let watched = |job_id: &[u8; 32]| {
        let job = job_pda(job_id);
        vec![job, vault_pda(&job), buyer_token, executor_token]
    };
    let cu = || compute_unit_limit_ix(CU);
    let release = |job_id: [u8; 32], journal: &Fixture, seal: Groth16Seal| {
        release_ix(job_id, &mint, &executor_token, journal.journal.clone(), seal)
    };
    let refund_on_fail = |job_id: [u8; 32], journal: &Fixture, seal: Groth16Seal| {
        refund_on_fail_ix(job_id, &mint, &buyer_token, journal.journal.clone(), seal)
    };
    let escrow = vericode_escrow::ID;

    // Job S: deliver + release in one transaction, as on devnet.
    let deliver_s = deliver_ix(&executor.pubkey(), job_s, s.artifact_hash());
    send(&mut env.ctx, &[cu(), deliver_s, release(job_s, &s, groth16_seal(&s))], &[&executor]).await.unwrap();

    // Job A, delivered (7,14): the D4b negatives, then the release.
    send(&mut env.ctx, &[deliver_ix(&executor.pubkey(), job_a, a.artifact_hash())], &[&executor]).await.unwrap();
    let keys = watched(&job_a);
    let ix = refund_on_fail(job_a, &a_fail, groth16_seal(&a_fail));
    let logs = reject(&mut env, "A: A′", &[cu(), ix], &[], &keys, &escrow, E_JOURNAL_ARTIFACT_HASH_MISMATCH).await;
    assert!(!invoked(&logs, &GROTH16_VERIFIER_ID));
    let ix = release(job_a, &s, groth16_seal(&s));
    let logs = reject(&mut env, "A: Job S", &[cu(), ix], &[], &keys, &escrow, E_JOURNAL_JOB_ID_MISMATCH).await;
    assert!(!invoked(&logs, &GROTH16_VERIFIER_ID));
    let ix = release(job_a, &a, groth16_seal(&s));
    let logs = reject(&mut env, "A: seal S", &[cu(), ix], &[], &keys, &GROTH16_VERIFIER_ID, VERIFIER_VERIFICATION_ERROR).await;
    assert!(invoked(&logs, &GROTH16_VERIFIER_ID));
    let mut tampered = groth16_seal(&a);
    tampered.pi_c[10] ^= 0x01;
    let ix = release(job_a, &a, tampered);
    reject(&mut env, "A: tampered", &[cu(), ix], &[], &keys, &GROTH16_VERIFIER_ID, VERIFIER_PAIRING_ERROR).await;
    let mut selector = groth16_seal(&a);
    selector.selector = [0; 4];
    let ix = release(job_a, &a, selector);
    let logs = reject(&mut env, "A: selector", &[cu(), ix], &[], &keys, &escrow, E_UNEXPECTED_SELECTOR).await;
    assert!(!invoked(&logs, &GROTH16_VERIFIER_ID));
    send(&mut env.ctx, &[cu(), release(job_a, &a, groth16_seal(&a))], &[]).await.unwrap();

    // Job B, delivered (7,15): the D4b negatives, then the refund on FAIL.
    send(&mut env.ctx, &[deliver_ix(&executor.pubkey(), job_b, b.artifact_hash())], &[&executor]).await.unwrap();
    let keys = watched(&job_b);
    let ix = release(job_b, &b, groth16_seal(&b));
    reject(&mut env, "B: release FAIL", &[cu(), ix], &[], &keys, &escrow, E_VERDICT_NOT_PASS).await;
    let ix = refund_on_fail(job_b, &a_fail, groth16_seal(&a_fail));
    reject(&mut env, "B: FAIL of A", &[cu(), ix], &[], &keys, &escrow, E_JOURNAL_JOB_ID_MISMATCH).await;
    let ix = refund_ix(job_b, &mint, &buyer_token);
    reject(&mut env, "B: early timeout", &[ix], &[], &keys, &escrow, E_DEADLINE_NOT_REACHED).await;
    send(&mut env.ctx, &[cu(), refund_on_fail(job_b, &b, groth16_seal(&b))], &[]).await.unwrap();

    // Invariant 9: a terminal Job accepts no second settlement, even with
    // its own valid receipt (the D7 devnet cases are A → 6007 and B → 6008).
    let ix = release(job_a, &a, groth16_seal(&a));
    let logs = reject(&mut env, "A: release again", &[cu(), ix], &[], &watched(&job_a), &escrow, E_ALREADY_RELEASED).await;
    assert!(!invoked(&logs, &GROTH16_VERIFIER_ID));
    let ix = refund_on_fail(job_b, &b, groth16_seal(&b));
    let logs = reject(&mut env, "B: refund again", &[cu(), ix], &[], &watched(&job_b), &escrow, E_ALREADY_REFUNDED).await;
    assert!(!invoked(&logs, &GROTH16_VERIFIER_ID));
    let ix = release(job_s, &s, groth16_seal(&s));
    reject(&mut env, "S: release again", &[cu(), ix], &[], &watched(&job_s), &escrow, E_ALREADY_RELEASED).await;

    // Job C: timeout after its deadline; then no terminal Job refunds on timeout.
    warp(&mut env.ctx, slot + 3_001).await;
    send(&mut env.ctx, &[refund_ix(job_c, &mint, &buyer_token)], &[]).await.unwrap();
    for (label, job_id, code) in [
        ("A: timeout after release", job_a, E_ALREADY_RELEASED),
        ("B: timeout after refund", job_b, E_ALREADY_REFUNDED),
        ("C: timeout again", job_c, E_ALREADY_REFUNDED),
    ] {
        let ix = refund_ix(job_id, &mint, &buyer_token);
        reject(&mut env, label, &[ix], &[], &watched(&job_id), &escrow, code).await;
    }

    for (job_id, expected) in [
        (job_s, EscrowStatus::Released { artifact_hash: s.artifact_hash() }),
        (job_a, EscrowStatus::Released { artifact_hash: a.artifact_hash() }),
        (job_b, EscrowStatus::RefundedOnFail { artifact_hash: b.artifact_hash() }),
        (job_c, EscrowStatus::RefundedOnTimeout),
    ] {
        assert_eq!(read_job(&mut env.ctx, &job_pda(&job_id)).await.status, expected);
        assert_eq!(token_balance(&mut env.ctx, &vault_pda(&job_pda(&job_id))).await, 0);
    }
    assert_eq!(token_balance(&mut env.ctx, &executor_token).await, 2 * AMOUNT);
    assert_eq!(token_balance(&mut env.ctx, &buyer_token).await, BUYER_START_BALANCE - 2 * AMOUNT);
}
