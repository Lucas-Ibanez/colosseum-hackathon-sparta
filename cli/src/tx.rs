//! Simulate, send, confirm and record one transaction.
//!
//! Positive transactions are sent only after a successful simulation.
//! Negative transactions (`--expect-error`) are sent with `skipPreflight`
//! only after the simulation shows the expected program error as the
//! innermost failure, and must land with that error and leave every watched
//! account byte for byte unchanged. An "already processed" answer to the send
//! is resolved by the signature status, like any other send; the outcome is
//! always that of the landed transaction (`getTransaction`).

use std::{str::FromStr, thread::sleep, time::Duration};

use serde_json::{json, Value};
use solana_hash::Hash;
use solana_instruction::Instruction;
use solana_keypair::Keypair;
use solana_pubkey::Pubkey;
use solana_signer::Signer;
use solana_transaction::Transaction;

use crate::{
    escrow::{MAX_TRANSACTION_SIZE, PROGRAM_ID, SYSTEM_PROGRAM_ID, TOKEN_PROGRAM_ID, VERIFIER_ID},
    explorer_tx,
    rpc::{Rpc, Sent},
};

const POLL: Duration = Duration::from_secs(2);
const REBROADCAST_EVERY_POLLS: u32 = 3;

/// Expected outcome of a transaction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Expect {
    Success,
    /// Success that invokes the Groth16 verifier (positive settlements): a
    /// simulation without the verifier is never sent, and a landed
    /// transaction without it is `UNEXPECTED`.
    Verified,
    /// `Custom(code)` returned by `program` as the innermost failure.
    Failure { program: Pubkey, code: u32 },
}

impl Expect {
    /// Parses `escrow:6014`, `verifier:6000`, `token:16` or `system:0`.
    pub fn parse(text: &str) -> Result<Self, String> {
        let (program, code) = text
            .split_once(':')
            .ok_or_else(|| format!("--expect-error {text:?}: expected <program>:<code>"))?;
        let program = match program {
            "escrow" => PROGRAM_ID,
            "verifier" => VERIFIER_ID,
            "token" => TOKEN_PROGRAM_ID,
            "system" => SYSTEM_PROGRAM_ID,
            other => return Err(format!("--expect-error: unknown program {other:?}")),
        };
        let code = code.parse().map_err(|_| format!("--expect-error: invalid code {code:?}"))?;
        Ok(Self::Failure { program, code })
    }

    pub fn is_failure(&self) -> bool {
        matches!(self, Self::Failure { .. })
    }
}

/// What landed.
#[derive(Clone, Debug)]
pub struct Outcome {
    pub signature: String,
    pub slot: u64,
    pub units: Option<u64>,
    pub fee: Option<u64>,
    pub size: usize,
    pub verifier_invoked: bool,
    pub logs: Vec<String>,
}

/// Wire bytes of a legacy transaction (fewer than 128 signatures).
pub fn wire(transaction: &Transaction) -> Vec<u8> {
    let mut out = vec![u8::try_from(transaction.signatures.len()).expect("signature count")];
    for signature in &transaction.signatures {
        out.extend_from_slice(signature.as_ref());
    }
    out.extend_from_slice(&transaction.message_data());
    out
}

/// Signs `instructions` with `payer` as fee payer, plus `others`.
pub fn sign(instructions: &[Instruction], payer: &Keypair, others: &[&Keypair], blockhash: Hash) -> Transaction {
    let mut signers: Vec<&Keypair> = vec![payer];
    for other in others {
        if !signers.iter().any(|signer| signer.pubkey() == other.pubkey()) {
            signers.push(other);
        }
    }
    Transaction::new_signed_with_payer(instructions, Some(&payer.pubkey()), &signers, blockhash)
}

fn failed_line(program: &Pubkey, code: u32) -> String {
    format!("Program {program} failed: custom program error: {code:#x}")
}

/// The `Custom` code of an `InstructionError`, if that is the error.
fn custom_code(err: &Value) -> Option<u32> {
    err.get("InstructionError")?.get(1)?.get("Custom")?.as_u64().and_then(|code| u32::try_from(code).ok())
}

fn logs_of(value: &Value) -> Vec<String> {
    value
        .as_array()
        .map(|lines| lines.iter().filter_map(|line| line.as_str().map(str::to_string)).collect())
        .unwrap_or_default()
}

/// The program of a runtime line `Program <id> failed: …`. Program output is
/// always prefixed (`Program log: …`, `Program data: …`), so it never parses
/// as a failure line.
fn failing_program(line: &str) -> Option<Pubkey> {
    let (id, rest) = line.strip_prefix("Program ")?.split_once(' ')?;
    if rest.starts_with("failed: ") {
        id.parse().ok()
    } else {
        None
    }
}

/// The innermost failure: the first `Program <id> failed: …` line. A failing
/// CPI logs its own line before the line of each caller.
pub fn innermost_failure(logs: &[String]) -> Option<&str> {
    logs.iter().map(String::as_str).find(|line| failing_program(line).is_some())
}

/// Whether `err` and `logs`, of a simulation or of a landed transaction, are
/// the outcome `expect` names.
///
/// A failure matches only when the error is `Custom(code)` and the innermost
/// failing program is `program`, with that code. For the escrow, no other
/// program may have failed at all: when the verifier rejects a proof, the
/// escrow logs the same code after it, and the verifier codes 6000–6003
/// overlap escrow codes, so such a failure is never an escrow rejection
/// (RD7-01).
pub fn expected_failure(expect: &Expect, err: &Value, logs: &[String]) -> bool {
    match expect {
        Expect::Success => err.is_null(),
        Expect::Verified => err.is_null() && invoked(logs, &VERIFIER_ID),
        Expect::Failure { program, code } => {
            let expected = failed_line(program, *code);
            custom_code(err) == Some(*code)
                && innermost_failure(logs) == Some(expected.as_str())
                && (*program != PROGRAM_ID || logs.iter().filter_map(|log| failing_program(log)).all(|id| id == PROGRAM_ID))
        }
    }
}

/// Why a successful result still misses `expect`, for the messages.
fn verifier_note(expect: &Expect, err: &Value) -> &'static str {
    if *expect == Expect::Verified && err.is_null() {
        " (the Groth16 verifier was not invoked)"
    } else {
        ""
    }
}

fn invoked(logs: &[String], program: &Pubkey) -> bool {
    let prefix = format!("Program {program} invoke");
    logs.iter().any(|line| line.starts_with(&prefix))
}

fn print_failure_logs(label: &str, logs: &[String]) {
    for line in logs.iter().filter(|line| line.contains("failed") || line.contains("Error") || line.contains("invoke")) {
        println!("[{label}]     {line}");
    }
}

/// Simulates and, if the simulation matches `expect`, sends and confirms the
/// transaction. `watch`: accounts that a negative transaction must not change.
pub fn run(
    rpc: &Rpc,
    label: &str,
    instructions: &[Instruction],
    payer: &Keypair,
    others: &[&Keypair],
    expect: Expect,
    watch: &[Pubkey],
) -> Result<Outcome, String> {
    let (blockhash, last_valid) = rpc.latest_blockhash()?;
    let blockhash_value = Hash::from_str(&blockhash).map_err(|_| "invalid blockhash")?;
    let transaction = sign(instructions, payer, others, blockhash_value);
    let bytes = wire(&transaction);
    let signature = transaction.signatures[0].to_string();
    let programs: Vec<String> = instructions.iter().map(|ix| ix.program_id.to_string()).collect();
    if bytes.len() > MAX_TRANSACTION_SIZE {
        return Err(format!("[{label}] transaction of {} bytes exceeds {MAX_TRANSACTION_SIZE}", bytes.len()));
    }
    let mut entry = json!({
        "kind": "tx", "label": label, "expect": format!("{expect:?}"), "payer": payer.pubkey().to_string(),
        "signature": signature, "size": bytes.len(), "programs": programs, "blockhash": blockhash,
    });

    let simulation = rpc.simulate(&bytes)?;
    let sim_err = simulation["err"].clone();
    let sim_logs = logs_of(&simulation["logs"]);
    let sim_units = simulation["unitsConsumed"].as_u64();
    entry["simulation"] = json!({"err": sim_err, "units": sim_units, "logs": sim_logs});
    println!(
        "[{label}] simulated err={sim_err} units={} size={} B verifier_invoked={}",
        sim_units.map_or("?".into(), |units| units.to_string()),
        bytes.len(),
        invoked(&sim_logs, &VERIFIER_ID)
    );
    if !expected_failure(&expect, &sim_err, &sim_logs) {
        print_failure_logs(label, &sim_logs);
        if let Some(line) = innermost_failure(&sim_logs) {
            println!("[{label}] innermost failure `{line}`");
        }
        entry["outcome"] = json!("SIMULATION_UNEXPECTED_NOT_SENT");
        rpc.record(entry);
        return Err(format!(
            "[{label}] simulation did not match {expect:?}{}; nothing was sent",
            verifier_note(&expect, &sim_err)
        ));
    }
    if let Expect::Failure { program, code } = expect {
        println!("[{label}] simulation shows `{}` as the innermost failure", failed_line(&program, code));
    }

    let before = if watch.is_empty() { None } else { Some(rpc.accounts(watch, None)?) };
    let skip_preflight = expect.is_failure();
    match rpc.send(&bytes, skip_preflight)? {
        Sent::Signature(sent) if sent != signature => {
            return Err(format!("[{label}] RPC returned signature {sent}, expected {signature}"));
        }
        Sent::Signature(_) => println!("[{label}] sent signature={signature} skip_preflight={skip_preflight}"),
        // A retried send whose first attempt reached the node: the same
        // signed bytes are already there. Not an error and not a success:
        // the status loop and `getTransaction` decide, as for any send.
        Sent::AlreadyProcessed => {
            entry["send"] = json!("ALREADY_PROCESSED");
            println!("[{label}] sendTransaction answered \"already processed\"; resolving signature={signature} by its status");
        }
    }

    let mut polls = 0_u32;
    loop {
        sleep(POLL);
        polls += 1;
        let status = rpc.signature_status(&signature, false)?;
        let confirmation = status["confirmationStatus"].as_str().unwrap_or("");
        if confirmation == "confirmed" || confirmation == "finalized" {
            break;
        }
        if rpc.block_height()? > last_valid {
            let status = rpc.signature_status(&signature, true)?;
            let confirmation = status["confirmationStatus"].as_str().unwrap_or("");
            if confirmation == "confirmed" || confirmation == "finalized" {
                break;
            }
            entry["outcome"] = json!("NOT_LANDED_BLOCKHASH_EXPIRED");
            rpc.record(entry);
            return Err(format!("[{label}] {signature} did not land before its blockhash expired"));
        }
        if polls % REBROADCAST_EVERY_POLLS == 0 {
            // Same signed bytes: a duplicate can never execute twice. Any
            // answer, "already processed" included, is ignored: the status
            // decides.
            let _ = rpc.send(&bytes, true);
        }
    }

    let mut landed = Value::Null;
    for _ in 0..30 {
        landed = rpc.transaction(&signature)?;
        if !landed.is_null() {
            break;
        }
        sleep(POLL);
    }
    if landed.is_null() {
        return Err(format!("[{label}] getTransaction {signature}: not available"));
    }
    let meta = &landed["meta"];
    let slot = landed["slot"].as_u64().ok_or("transaction slot")?;
    let err = meta["err"].clone();
    let logs = logs_of(&meta["logMessages"]);
    let units = meta["computeUnitsConsumed"].as_u64();
    let fee = meta["fee"].as_u64();
    let verifier_invoked = invoked(&logs, &VERIFIER_ID);
    let mut passed = expected_failure(&expect, &err, &logs);
    entry.as_object_mut().expect("object").extend([
        ("slot".to_string(), json!(slot)),
        ("err".to_string(), err.clone()),
        ("units".to_string(), json!(units)),
        ("fee".to_string(), json!(fee)),
        ("logs".to_string(), json!(logs)),
        ("verifier_invoked".to_string(), json!(verifier_invoked)),
        ("explorer".to_string(), json!(explorer_tx(&signature))),
        ("pre_token_balances".to_string(), meta["preTokenBalances"].clone()),
        ("post_token_balances".to_string(), meta["postTokenBalances"].clone()),
    ]);
    if let Some((before_slot, before_accounts)) = before {
        let (after_slot, after_accounts) = rpc.accounts(watch, Some(slot))?;
        let unchanged = before_accounts == after_accounts;
        entry["snapshot"] = json!({
            "keys": watch.iter().map(ToString::to_string).collect::<Vec<_>>(),
            "before_slot": before_slot, "after_slot": after_slot, "unchanged": unchanged,
        });
        println!("[{label}] watched accounts unchanged={unchanged} (slots {before_slot} -> {after_slot})");
        if expect.is_failure() {
            passed = passed && unchanged;
        }
    }
    entry["outcome"] = json!(if passed { "PASS" } else { "UNEXPECTED" });
    rpc.record(entry);
    println!(
        "[{label}] {} slot={slot} err={err} units={} fee={} size={} B verifier_invoked={verifier_invoked}",
        if passed { "PASS" } else { "UNEXPECTED" },
        units.map_or("?".into(), |units| units.to_string()),
        fee.map_or("?".into(), |fee| fee.to_string()),
        bytes.len()
    );
    for line in logs.iter().filter(|line| line.contains(" consumed ")) {
        println!("[{label}]     {line}");
    }
    if !err.is_null() {
        print_failure_logs(label, &logs);
        if let Some(line) = innermost_failure(&logs) {
            println!("[{label}] innermost failure `{line}`");
        }
    }
    println!("[{label}] explorer={}", explorer_tx(&signature));
    if !passed {
        return Err(format!(
            "[{label}] landed transaction did not match {expect:?}{}",
            verifier_note(&expect, &err)
        ));
    }
    Ok(Outcome {
        signature,
        slot,
        units,
        fee,
        size: bytes.len(),
        verifier_invoked,
        logs,
    })
}
