//! Negative runs and sending, offline (gate D10a; findings RD7-01, RD7-04,
//! RD7-06 and RD7-07 of the R-D7).
//!
//! - `expected_failure`: an `--expect-error PROGRAM:CODE` matches only when
//!   that program is the innermost failure; a verifier failure is never
//!   reported as an escrow rejection with an overlapping code (6000–6003).
//! - `tx::run` against a fake RPC on 127.0.0.1 (`fake_rpc`): an overlapping
//!   code is refused before sending, and an "already processed" answer is
//!   resolved by the signature status, never as a false error or a false
//!   success.
//! - Log file mode and hard-linked keypair files.
//!
//! Nothing here uses devnet or a keypair file: transactions are signed by
//! in-memory keypairs and go only to the fake server.

mod fake_rpc;

use std::{
    fs,
    os::unix::fs::PermissionsExt,
    path::PathBuf,
    process::Command,
};

use fake_rpc::{already_processed, answers, custom, failed, signature_of, FakeRpc, LANDED_SLOT};
use serde_json::{json, Value};
use solana_keypair::Keypair;
use solana_pubkey::Pubkey;
use vericode_cli::{
    escrow::{self as cli, PROGRAM_ID as ESCROW, TOKEN_PROGRAM_ID as TOKEN, VERIFIER_ID as VERIFIER},
    keys,
    rpc::Rpc,
    tx::{self, expected_failure, Expect},
};

fn on(program: Pubkey, code: u32) -> Expect {
    Expect::Failure { program, code }
}

fn invoke(program: &Pubkey, depth: u8) -> String {
    format!("Program {program} invoke [{depth}]")
}

/// Logs of a settlement whose verifier CPI rejects the proof, as in W5 of
/// the D9 (`HXgnGUhh…`): the verifier line first, then the escrow line with
/// the same code.
fn verifier_rejection(code: u32) -> Vec<String> {
    vec![
        invoke(&ESCROW, 1),
        "Program log: Instruction: Release".into(),
        invoke(&VERIFIER, 2),
        "Program log: Error: PairingError".into(),
        failed(&VERIFIER, code),
        failed(&ESCROW, code),
    ]
}

#[test]
fn an_escrow_code_matches_only_an_innermost_escrow_failure() {
    let own = vec![
        invoke(&ESCROW, 1),
        "Program log: AnchorError occurred. Error Code: JournalJobIdMismatch. Error Number: 6014.".into(),
        failed(&ESCROW, 6014),
    ];
    assert!(expected_failure(&on(ESCROW, 6014), &custom(1, 6014), &own), "escrow:6014 by the escrow");
    assert!(!expected_failure(&on(VERIFIER, 6014), &custom(1, 6014), &own), "verifier:6014 without the verifier");

    // The verifier codes 6000–6003 overlap escrow codes (RD7-01).
    for code in 6000..=6003 {
        let logs = verifier_rejection(code);
        assert!(!expected_failure(&on(ESCROW, code), &custom(1, code), &logs), "escrow:{code} on a verifier failure");
        assert!(expected_failure(&on(VERIFIER, code), &custom(1, code), &logs), "verifier:{code}");
    }

    // Right code, but only another program's line.
    let other = vec![invoke(&ESCROW, 1), failed(&VERIFIER, 6003)];
    assert!(!expected_failure(&on(ESCROW, 6003), &custom(1, 6003), &other));
    // An escrow line followed by another failing program: never an escrow
    // rejection, and the other program is not the innermost failure either.
    let reversed = vec![failed(&ESCROW, 6003), failed(&VERIFIER, 6003)];
    assert!(!expected_failure(&on(ESCROW, 6003), &custom(1, 6003), &reversed));
    assert!(!expected_failure(&on(VERIFIER, 6003), &custom(1, 6003), &reversed));
    // An SPL Token error inside the escrow belongs to the Token program.
    let token = vec![invoke(&ESCROW, 1), invoke(&TOKEN, 2), failed(&TOKEN, 1), failed(&ESCROW, 1)];
    assert!(expected_failure(&on(TOKEN, 1), &custom(0, 1), &token));
    assert!(!expected_failure(&on(ESCROW, 1), &custom(0, 1), &token));
}

#[test]
fn codes_lines_and_successes_must_all_agree() {
    let logs = vec![invoke(&ESCROW, 1), failed(&ESCROW, 6021)];
    let expect = on(ESCROW, 6021);
    assert!(expected_failure(&expect, &custom(0, 6021), &logs));
    assert!(!expected_failure(&expect, &custom(0, 6022), &logs), "another Custom code");
    assert!(!expected_failure(&on(ESCROW, 6022), &custom(0, 6022), &logs), "another line code");
    assert!(!expected_failure(&expect, &json!("AccountInUse"), &logs), "not a Custom error");
    assert!(!expected_failure(&expect, &Value::Null, &[invoke(&ESCROW, 1)]), "unexpected success");
    assert!(expected_failure(&Expect::Success, &Value::Null, &[]));
    assert!(!expected_failure(&Expect::Success, &custom(0, 6021), &logs), "unexpected failure");
    // Program output never counts as a runtime failure line.
    let spoof = vec![
        invoke(&ESCROW, 1),
        format!("Program log: {}", failed(&VERIFIER, 6021)),
        "Program data: failed: x".into(),
        failed(&ESCROW, 6021),
    ];
    assert!(expected_failure(&expect, &custom(0, 6021), &spoof));
    assert!(!expected_failure(&on(VERIFIER, 6021), &custom(0, 6021), &spoof));
}

fn rpc(fake: &FakeRpc) -> Rpc {
    Rpc::new(&fake.url, None).unwrap()
}

fn refund_ix() -> solana_instruction::Instruction {
    cli::refund_on_timeout_ix([0x42; 32], &cli::ata(&Pubkey::new_from_array([0xb1; 32])))
}

#[test]
fn an_overlapping_escrow_code_is_refused_before_sending() {
    for code in [6000, 6003] {
        let logs = verifier_rejection(code);
        let fake = FakeRpc::start(answers(
            (custom(1, code), logs.clone()),
            Some((custom(1, code), logs)),
            |params| Ok(json!(signature_of(params))),
        ));
        let payer = Keypair::new();
        let error = tx::run(&rpc(&fake), "overlap", &[refund_ix()], &payer, &[], on(ESCROW, code), &[]).unwrap_err();
        assert!(error.contains("nothing was sent"), "escrow:{code}: {error}");
        assert_eq!(fake.count("sendTransaction"), 0, "escrow:{code} was sent");

        let outcome = tx::run(&rpc(&fake), "overlap", &[refund_ix()], &payer, &[], on(VERIFIER, code), &[]).unwrap();
        assert_eq!((outcome.slot, outcome.verifier_invoked), (LANDED_SLOT, true), "verifier:{code}");
    }
}

#[test]
fn already_processed_is_resolved_by_the_signature_status() {
    let ok = (Value::Null, vec![invoke(&ESCROW, 1), format!("Program {ESCROW} success")]);
    let fake = FakeRpc::start(answers(ok.clone(), Some(ok), |_| Err(already_processed())));
    let payer = Keypair::new();
    let outcome = tx::run(&rpc(&fake), "already", &[refund_ix()], &payer, &[], Expect::Success, &[])
        .expect("an already processed send that landed is not an error");
    assert_eq!(outcome.slot, LANDED_SLOT);
    assert!(fake.count("getSignatureStatuses") >= 1 && fake.count("getTransaction") >= 1);
}

#[test]
fn already_processed_never_turns_a_failed_transaction_into_a_success() {
    let logs = vec![invoke(&ESCROW, 1), failed(&ESCROW, 6021)];
    let fake = FakeRpc::start(answers(
        (Value::Null, vec![invoke(&ESCROW, 1)]),
        Some((custom(0, 6021), logs)),
        |_| Err(already_processed()),
    ));
    let error = tx::run(&rpc(&fake), "already", &[refund_ix()], &Keypair::new(), &[], Expect::Success, &[]).unwrap_err();
    assert!(error.contains("landed transaction did not match Success"), "{error}");
}

#[test]
fn already_processed_without_a_landed_transaction_is_not_landed() {
    let fake = FakeRpc::start(answers((Value::Null, vec![]), None, |_| Err(already_processed())));
    let error = tx::run(&rpc(&fake), "already", &[refund_ix()], &Keypair::new(), &[], Expect::Success, &[]).unwrap_err();
    assert!(error.contains("did not land before its blockhash expired"), "{error}");
}

#[test]
fn a_signature_other_than_ours_is_refused() {
    let ok = (Value::Null, vec![]);
    let fake = FakeRpc::start(answers(ok.clone(), Some(ok), |_| Ok(json!(fake_rpc::base58(&[9_u8; 64])))));
    let error = tx::run(&rpc(&fake), "wrongsig", &[refund_ix()], &Keypair::new(), &[], Expect::Success, &[]).unwrap_err();
    assert!(error.contains("RPC returned signature"), "{error}");
}

fn scratch(name: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("negative-runs");
    fs::create_dir_all(&dir).unwrap();
    let path = dir.join(name);
    let _ = fs::remove_file(&path);
    path
}

#[test]
fn an_existing_log_file_is_set_to_0600() {
    let path = scratch("existing.jsonl");
    fs::write(&path, "").unwrap();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).unwrap();
    // Building the client contacts nothing.
    Rpc::new("https://api.devnet.solana.com", Some(&path)).unwrap();
    assert_eq!(fs::metadata(&path).unwrap().permissions().mode() & 0o777, 0o600);
}

#[test]
fn a_hard_linked_keypair_file_is_refused() {
    let first = scratch("linked-a.json");
    let second = scratch("linked-b.json");
    fs::write(&first, "[]").unwrap();
    fs::set_permissions(&first, fs::Permissions::from_mode(0o600)).unwrap();
    fs::hard_link(&first, &second).unwrap();
    for path in [&first, &second] {
        match keys::load(path, "buyer") {
            Ok(_) => panic!("a hard-linked file was accepted"),
            Err(error) => assert!(error.contains("hard link"), "{error}"),
        }
    }
}

#[test]
fn tamper_seal_needs_expect_error_and_is_refused_before_any_rpc() {
    let output = Command::new(env!("CARGO_BIN_EXE_vericode"))
        .args(["job", "settle", "--job-id", &"42".repeat(32), "--receipt", "/nonexistent", "--tamper-seal"])
        .args(["--payer-keypair", "/nonexistent"])
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(1), "{stderr}");
    assert!(stderr.contains("--tamper-seal is only for negative runs"), "{stderr}");
}
