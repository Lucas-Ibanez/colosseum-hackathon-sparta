//! Client guards added in gate D10a for the informative findings of the
//! R-D7 (RD7-09 and RD7-10), offline.
//!
//! - A positive settlement (`Expect::Verified`) must invoke the Groth16
//!   verifier: without it nothing is sent, and a landed transaction without
//!   it is never PASS.
//! - Deadline messages tell "past the deadline" from "too close".
//! - `--rpc-url` accepts only `https://`, or `http://` on the loopback
//!   interface for local test servers.

mod fake_rpc;

use std::process::Command;

use fake_rpc::{answers, failed, signature_of, FakeRpc};
use serde_json::{json, Value};
use solana_keypair::Keypair;
use solana_pubkey::Pubkey;
use vericode_cli::{
    escrow::{self as cli, deadline_margin_problem, PROGRAM_ID as ESCROW, VERIFIER_ID as VERIFIER},
    rpc::{check_url, Rpc},
    tx::{self, expected_failure, innermost_failure, Expect},
};

fn invoke(program: &Pubkey, depth: u8) -> String {
    format!("Program {program} invoke [{depth}]")
}

#[test]
fn a_positive_settlement_must_invoke_the_verifier() {
    let verified = vec![invoke(&ESCROW, 1), invoke(&VERIFIER, 2), format!("Program {VERIFIER} success")];
    let unverified = vec![invoke(&ESCROW, 1), format!("Program {ESCROW} success")];
    assert!(expected_failure(&Expect::Verified, &Value::Null, &verified));
    assert!(!expected_failure(&Expect::Verified, &Value::Null, &unverified));
    assert!(!expected_failure(&Expect::Verified, &json!("AccountInUse"), &verified));
    assert!(!Expect::Verified.is_failure());

    let ix = cli::refund_on_timeout_ix([0x42; 32], &cli::ata(&Pubkey::new_from_array([0xb1; 32])));
    // Simulation without the verifier: nothing is sent.
    let fake = FakeRpc::start(answers(
        (Value::Null, unverified.clone()),
        Some((Value::Null, unverified.clone())),
        |params| Ok(json!(signature_of(params))),
    ));
    let rpc = Rpc::new(&fake.url, None).unwrap();
    let error = tx::run(&rpc, "release", &[ix.clone()], &Keypair::new(), &[], Expect::Verified, &[]).unwrap_err();
    assert!(error.contains("the Groth16 verifier was not invoked") && error.contains("nothing was sent"), "{error}");
    assert_eq!(fake.count("sendTransaction"), 0);
    // Landed without the verifier although the simulation had it: UNEXPECTED.
    let fake = FakeRpc::start(answers(
        (Value::Null, verified),
        Some((Value::Null, unverified)),
        |params| Ok(json!(signature_of(params))),
    ));
    let rpc = Rpc::new(&fake.url, None).unwrap();
    let error = tx::run(&rpc, "release", &[ix], &Keypair::new(), &[], Expect::Verified, &[]).unwrap_err();
    assert!(error.contains("landed transaction did not match Verified (the Groth16 verifier was not invoked)"), "{error}");
}

#[test]
fn the_innermost_failure_is_the_first_runtime_failure_line() {
    let logs = vec![
        invoke(&ESCROW, 1),
        format!("Program log: {}", failed(&ESCROW, 6000)),
        invoke(&VERIFIER, 2),
        failed(&VERIFIER, 6003),
        failed(&ESCROW, 6003),
    ];
    assert_eq!(innermost_failure(&logs), Some(failed(&VERIFIER, 6003).as_str()));
    assert_eq!(innermost_failure(&logs[..3]), None);
}

#[test]
fn deadline_messages_tell_past_from_too_close() {
    assert_eq!(deadline_margin_problem(1_000, 1_060, "a release"), None);
    let close = deadline_margin_problem(1_001, 1_060, "a release").unwrap();
    assert!(close.contains("too close to the deadline 1060"), "{close}");
    // The program accepts a release up to the deadline slot inclusive.
    let at = deadline_margin_problem(1_060, 1_060, "a release").unwrap();
    assert!(at.contains("too close"), "{at}");
    let past = deadline_margin_problem(1_061, 1_060, "a release").unwrap();
    assert!(past.contains("past the deadline 1060"), "{past}");
}

#[test]
fn rpc_urls_are_https_or_loopback_http() {
    for url in [
        "https://api.devnet.solana.com",
        "https://api.devnet.solana.com/",
        "http://127.0.0.1:8899",
        "http://localhost:8899",
        "http://[::1]:8899",
    ] {
        assert!(check_url(url).is_ok(), "{url}");
    }
    for url in [
        "http://api.devnet.solana.com",
        "http://127.0.0.1.example.com:8899",
        "http://10.0.0.1:8899",
        "ftp://127.0.0.1",
        "api.devnet.solana.com",
        "",
    ] {
        assert!(check_url(url).is_err(), "{url}");
    }
}

#[test]
fn a_plain_http_rpc_url_is_refused_before_any_request() {
    let output = Command::new(env!("CARGO_BIN_EXE_vericode"))
        .args(["--rpc-url", "http://api.devnet.solana.com", "check"])
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(1), "{stderr}");
    assert!(stderr.contains("only https://"), "{stderr}");
    assert!(output.stdout.is_empty(), "nothing is printed before the refusal");
}
