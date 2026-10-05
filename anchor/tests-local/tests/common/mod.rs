//! Helpers shared by the in-process tests of the SBF build of
//! `vericode_escrow`.
//!
//! Programs are loaded from `SBF_OUT_DIR` by `solana-program-test`; no
//! validator, cluster, RPC or keypair file is used. Expected error codes are
//! literals, not values derived from the program; `tests/layout.rs` binds
//! each VeriCode code to its enum variant. The admitted Test USDC mint is
//! injected in genesis at its fixed address, with an in-memory authority.

// Each test crate uses a different subset of these helpers.
#![allow(dead_code, unused_imports)]

use std::{collections::HashMap, fs, path::PathBuf};

pub use anchor_lang::solana_program::clock::Clock;
pub use anchor_lang::solana_program::hash::hash as sha256;
pub use anchor_lang::solana_program::instruction::{AccountMeta, Instruction, InstructionError};
pub use anchor_lang::solana_program::program_pack::Pack;
pub use anchor_lang::solana_program::pubkey::Pubkey;
pub use anchor_lang::solana_program::rent::Rent;
pub use anchor_lang::{AccountDeserialize, InstructionData, ToAccountMetas};
pub use anchor_spl::token::spl_token;
pub use anchor_spl::token_2022::spl_token_2022;
pub use solana_keypair::Keypair;
pub use solana_program_test::{tokio, BanksClientError, ProgramTest, ProgramTestContext};
pub use solana_signer::Signer;
pub use solana_system_interface::instruction as system_instruction;
pub use solana_system_interface::program as system_program;
pub use solana_transaction::Transaction;
pub use solana_transaction_error::TransactionError;
pub use vericode_core::{hash_restricted_artifact, JournalV1, RestrictedArtifactV1};
pub use vericode_escrow::{
    accounts, instruction, EscrowStatus, Groth16Seal, JobAccount, ADMITTED_MINT, ATA_PROGRAM_ID,
    GROTH16_SELECTOR, GROTH16_VERIFIER_ID, JOB_ACCOUNT_VERSION, JOB_SEED, VAULT_SEED,
};
use spl_token::solana_program::program_option::COption;

pub const PROGRAM_NAME: &str = "vericode_escrow";
pub const DECIMALS: u8 = 6;
pub const AMOUNT: u64 = 1_000_000;
pub const BUYER_START_BALANCE: u64 = 5_000_000;
// Inside the creation window of a Job created at the first slots of the test.
pub const DEADLINE: u64 = 5_000;
// Job of the versioned Groth16 fixtures.
pub const JOB_ID: [u8; 32] = [0x11; 32];
// Final D1c2b guest ImageID, the only image a v1 Job admits.
pub const IMAGE_ID_HEX: &str = "4da06f90da75ec8980c943ce017d69c48370fddbf3aa27689d375d78fac0fb1a";
// Creation window decided for D2b.1, in slots.
pub const MIN_WINDOW: u64 = 1_500;
pub const MAX_WINDOW: u64 = 1_512_000;
pub const OTHER: [u8; 32] = [0x99; 32];

// `VericodeEscrowError` codes.
pub const E_AMOUNT_ZERO: u32 = 6000;
pub const E_ZERO_EXECUTOR: u32 = 6002;
pub const E_BUYER_IS_EXECUTOR: u32 = 6004;
pub const E_NOT_FUNDED: u32 = 6005;
pub const E_ALREADY_FUNDED: u32 = 6006;
pub const E_ALREADY_RELEASED: u32 = 6007;
pub const E_ALREADY_REFUNDED: u32 = 6008;
pub const E_DEPOSITOR_MISMATCH: u32 = 6009;
pub const E_RECIPIENT_MISMATCH: u32 = 6010;
pub const E_MINT_MISMATCH: u32 = 6011;
pub const E_AMOUNT_MISMATCH: u32 = 6012;
pub const E_JOURNAL_ARTIFACT_HASH_MISMATCH: u32 = 6017;
pub const E_VERDICT_NOT_PASS: u32 = 6019;
pub const E_VERDICT_NOT_FAIL: u32 = 6020;
pub const E_DEADLINE_NOT_REACHED: u32 = 6021;
pub const E_DEADLINE_PASSED: u32 = 6022;
pub const E_UNSUPPORTED_ACCOUNT_VERSION: u32 = 6023;
pub const E_MINT_HAS_FREEZE_AUTHORITY: u32 = 6024;
pub const E_NOT_DELIVERED: u32 = 6025;
pub const E_ALREADY_DELIVERED: u32 = 6026;
pub const E_DELIVERER_MISMATCH: u32 = 6027;
pub const E_SPEC_NOT_ADMITTED: u32 = 6028;
pub const E_HARNESS_NOT_ADMITTED: u32 = 6029;
pub const E_IMAGE_ID_NOT_ADMITTED: u32 = 6030;
pub const E_DEADLINE_OUT_OF_WINDOW: u32 = 6031;
pub const E_EXECUTOR_IS_PROGRAM_ACCOUNT: u32 = 6032;
pub const E_UNEXPECTED_SELECTOR: u32 = 6033;
pub const E_JOURNAL_MALFORMED: u32 = 6034;
pub const E_DESTINATION_NOT_CANONICAL: u32 = 6035;
pub const E_MINT_NOT_ADMITTED: u32 = 6036;
// Anchor 0.31.1 framework errors.
pub const ANCHOR_CONSTRAINT_SEEDS: u32 = 2006;
pub const ANCHOR_CONSTRAINT_ADDRESS: u32 = 2012;
pub const ANCHOR_ACCOUNT_OWNED_BY_WRONG_PROGRAM: u32 = 3007;
pub const ANCHOR_INVALID_PROGRAM_ID: u32 = 3008;
pub const ANCHOR_ACCOUNT_NOT_SIGNER: u32 = 3010;
// System Program `AccountAlreadyInUse`.
pub const SYSTEM_ACCOUNT_ALREADY_IN_USE: u32 = 0;

// BN254 base field modulus, to negate `pi_a` as the upstream client does.
const BN254_BASE_FIELD_MODULUS: &str =
    "30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47";
const COMPUTE_BUDGET_PROGRAM_ID: Pubkey =
    Pubkey::from_str_const("ComputeBudget111111111111111111111111111111");

pub struct Env {
    pub ctx: ProgramTestContext,
    pub buyer: Keypair,
    pub executor: Keypair,
    /// The admitted Test USDC mint, at [`ADMITTED_MINT`].
    pub mint: Pubkey,
    pub mint_authority: Keypair,
    /// Canonical associated token account of the buyer for the Job mint.
    pub buyer_token: Pubkey,
}

/// Commitments sent to `create_job`.
#[derive(Clone, Copy)]
pub struct Commitments {
    pub spec_hash: [u8; 32],
    pub harness_hash: [u8; 32],
    pub image_id: [u8; 32],
}

/// One versioned Groth16 fixture (`fixtures/groth16/<name>.txt`).
pub struct Fixture {
    pub selector: [u8; 4],
    pub image_id: [u8; 32],
    pub journal: Vec<u8>,
    pub journal_digest: [u8; 32],
    /// Raw seal, `pi_a` not negated.
    pub seal: [u8; 256],
}

impl Fixture {
    /// Artifact commitment of the fixture journal.
    pub fn artifact_hash(&self) -> [u8; 32] {
        JournalV1::decode_candidate(&self.journal)
            .unwrap()
            .artifact_hash()
            .into_bytes()
    }
}

pub fn decode_hex(hex: &str) -> Vec<u8> {
    assert!(hex.len() % 2 == 0, "odd hex length");
    (0..hex.len())
        .step_by(2)
        .map(|index| u8::from_str_radix(&hex[index..index + 2], 16).unwrap())
        .collect()
}

pub fn hex32(hex: &str) -> [u8; 32] {
    decode_hex(hex).try_into().expect("32 bytes")
}

pub fn fixture(name: &str) -> Fixture {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/groth16")
        .join(format!("{name}.txt"));
    let text = fs::read_to_string(&path).unwrap();
    let fields: HashMap<&str, Vec<u8>> = text
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
        .map(|line| {
            let (key, value) = line.split_once('=').expect("key=hex");
            (key, decode_hex(value.trim()))
        })
        .collect();
    let field = |key: &str| fields.get(key).unwrap_or_else(|| panic!("{name}: missing {key}")).clone();
    Fixture {
        selector: field("selector").try_into().unwrap(),
        image_id: field("image_id").try_into().unwrap(),
        journal: field("journal"),
        journal_digest: field("journal_digest").try_into().unwrap(),
        seal: field("seal").try_into().unwrap(),
    }
}

/// Negates a BN254 G1 point (`y' = q - y`), as the upstream client does for
/// `pi_a` before verification.
pub fn negate_g1(point: &[u8]) -> [u8; 64] {
    let modulus = decode_hex(BN254_BASE_FIELD_MODULUS);
    let mut out = [0_u8; 64];
    out[..32].copy_from_slice(&point[..32]);
    let mut borrow = 0_i16;
    for index in (0..32).rev() {
        let difference = i16::from(modulus[index]) - i16::from(point[32 + index]) - borrow;
        out[32 + index] = difference.rem_euclid(256) as u8;
        borrow = i16::from(difference < 0);
    }
    out
}

/// Groth16 seal of a fixture, encoded like the upstream client.
pub fn groth16_seal(fixture: &Fixture) -> Groth16Seal {
    Groth16Seal {
        selector: fixture.selector,
        pi_a: negate_g1(&fixture.seal[0..64]),
        pi_b: fixture.seal[64..192].try_into().unwrap(),
        pi_c: fixture.seal[192..256].try_into().unwrap(),
    }
}

/// Standard base64 with padding, for `ProgramTest::add_account_with_base64_data`.
pub fn base64(bytes: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in bytes.chunks(3) {
        let padded = [chunk[0], *chunk.get(1).unwrap_or(&0), *chunk.get(2).unwrap_or(&0)];
        let bits = u32::from(padded[0]) << 16 | u32::from(padded[1]) << 8 | u32::from(padded[2]);
        for index in 0..4 {
            if index <= chunk.len() {
                out.push(TABLE[(bits >> (18 - 6 * index)) as usize & 63] as char);
            } else {
                out.push('=');
            }
        }
    }
    out
}

pub fn admitted() -> Commitments {
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
pub fn delivered_artifact_hash() -> [u8; 32] {
    hash_restricted_artifact(&RestrictedArtifactV1::new(7, 14))
        .unwrap()
        .into_bytes()
}

pub fn job_pda(job_id: &[u8; 32]) -> Pubkey {
    Pubkey::find_program_address(&[JOB_SEED, job_id], &vericode_escrow::ID).0
}

pub fn vault_pda(job: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[VAULT_SEED, job.as_ref()], &vericode_escrow::ID).0
}

pub fn ata_address(owner: &Pubkey, mint: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[owner.as_ref(), spl_token::ID.as_ref(), mint.as_ref()],
        &ATA_PROGRAM_ID,
    )
    .0
}

/// Replaces one account of an instruction, keeping its signer and writable flags.
pub fn substitute(mut ix: Instruction, original: &Pubkey, replacement: &Pubkey) -> Instruction {
    let meta = ix
        .accounts
        .iter_mut()
        .find(|meta| meta.pubkey == *original)
        .expect("account in instruction");
    meta.pubkey = *replacement;
    ix
}

/// Clears the signer flag of one account, as a caller without its key would.
pub fn unsigned(mut ix: Instruction, key: &Pubkey) -> Instruction {
    let meta = ix
        .accounts
        .iter_mut()
        .find(|meta| meta.pubkey == *key)
        .expect("account in instruction");
    meta.is_signer = false;
    ix
}

/// `SetComputeUnitLimit` of the Compute Budget program.
pub fn compute_unit_limit_ix(units: u32) -> Instruction {
    let mut data = vec![2_u8];
    data.extend_from_slice(&units.to_le_bytes());
    Instruction {
        program_id: COMPUTE_BUDGET_PROGRAM_ID,
        accounts: vec![],
        data,
    }
}

pub fn signed_transaction(
    ctx: &ProgramTestContext,
    instructions: &[Instruction],
    signers: &[&Keypair],
    blockhash: anchor_lang::solana_program::hash::Hash,
) -> Transaction {
    let mut all_signers: Vec<&Keypair> = vec![&ctx.payer];
    all_signers.extend_from_slice(signers);
    Transaction::new_signed_with_payer(instructions, Some(&ctx.payer.pubkey()), &all_signers, blockhash)
}

pub async fn send(
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
pub async fn compute_units(ctx: &mut ProgramTestContext, instructions: &[Instruction], signers: &[&Keypair]) -> u64 {
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

pub fn assert_custom(result: Result<(), BanksClientError>, expected: u32) {
    match result {
        Err(BanksClientError::TransactionError(TransactionError::InstructionError(
            _,
            InstructionError::Custom(code),
        ))) => assert_eq!(code, expected, "unexpected custom error code"),
        other => panic!("expected custom error {expected}, got {other:?}"),
    }
}

/// Simulates a failing transaction to read its logs, then sends it.
///
/// Custom codes of different programs overlap (the verifier and VeriCode
/// both start at 6000), so the logs name the program that failed.
/// Returns the simulation logs.
pub async fn assert_failure(
    ctx: &mut ProgramTestContext,
    instructions: &[Instruction],
    signers: &[&Keypair],
    program: &Pubkey,
    code: u32,
) -> Vec<String> {
    let blockhash = ctx.get_new_latest_blockhash().await.unwrap();
    let transaction = signed_transaction(ctx, instructions, signers, blockhash);
    let simulation = ctx
        .banks_client
        .simulate_transaction(transaction.clone())
        .await
        .unwrap();
    let logs = simulation.simulation_details.unwrap().logs;
    let failed = format!("Program {program} failed: custom program error: {code:#x}");
    assert!(logs.iter().any(|line| *line == failed), "expected `{failed}` in {logs:#?}");
    assert_custom(ctx.banks_client.process_transaction(transaction).await, code);
    logs
}

/// Whether the logs show an invocation of `program`.
pub fn invoked(logs: &[String], program: &Pubkey) -> bool {
    let prefix = format!("Program {program} invoke");
    logs.iter().any(|line| line.starts_with(&prefix))
}

pub async fn current_slot(ctx: &mut ProgramTestContext) -> u64 {
    ctx.banks_client.get_sysvar::<Clock>().await.unwrap().slot
}

pub async fn warp(ctx: &mut ProgramTestContext, slot: u64) {
    ctx.warp_to_slot(slot).unwrap();
    assert_eq!(current_slot(ctx).await, slot, "Clock.slot after warp");
}

pub async fn transfer_lamports(ctx: &mut ProgramTestContext, to: &Pubkey, lamports: u64) {
    let ix = system_instruction::transfer(&ctx.payer.pubkey(), to, lamports);
    send(ctx, &[ix], &[]).await.unwrap();
}

pub async fn create_mint(ctx: &mut ProgramTestContext, authority: &Pubkey) -> Pubkey {
    create_mint_with_freeze(ctx, authority, None).await
}

pub async fn create_mint_with_freeze(
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
pub async fn create_token_2022_mint(ctx: &mut ProgramTestContext, authority: &Pubkey) -> Pubkey {
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

/// Token account at a fresh keypair address, not the canonical one.
pub async fn create_token_account(ctx: &mut ProgramTestContext, mint: &Pubkey, owner: &Pubkey) -> Pubkey {
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

/// Canonical associated token account, created through the Associated Token
/// Account program that `solana-program-test` embeds.
pub async fn create_ata(ctx: &mut ProgramTestContext, owner: &Pubkey, mint: &Pubkey) -> Pubkey {
    let ata = ata_address(owner, mint);
    let ix = Instruction {
        program_id: ATA_PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(ctx.payer.pubkey(), true),
            AccountMeta::new(ata, false),
            AccountMeta::new_readonly(*owner, false),
            AccountMeta::new_readonly(*mint, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
        ],
        // Empty data is the `Create` instruction.
        data: vec![],
    };
    send(ctx, &[ix], &[]).await.unwrap();
    ata
}

pub async fn mint_to(ctx: &mut ProgramTestContext, mint: &Pubkey, to: &Pubkey, authority: &Keypair, amount: u64) {
    let ix =
        spl_token::instruction::mint_to(&spl_token::ID, mint, to, &authority.pubkey(), &[], amount).unwrap();
    send(ctx, &[ix], &[authority]).await.unwrap();
}

pub async fn token_balance(ctx: &mut ProgramTestContext, account: &Pubkey) -> u64 {
    let data = ctx.banks_client.get_account(*account).await.unwrap().unwrap().data;
    spl_token::state::Account::unpack(&data).unwrap().amount
}

pub async fn read_job(ctx: &mut ProgramTestContext, job: &Pubkey) -> JobAccount {
    let data = ctx.banks_client.get_account(*job).await.unwrap().unwrap().data;
    JobAccount::try_deserialize(&mut data.as_slice()).unwrap()
}

/// Raw bytes and lamports of the given accounts, for unchanged-state checks.
pub async fn snapshot(ctx: &mut ProgramTestContext, keys: &[Pubkey]) -> Vec<Option<(u64, Vec<u8>)>> {
    let mut out = Vec::new();
    for key in keys {
        let account = ctx.banks_client.get_account(*key).await.unwrap();
        out.push(account.map(|account| (account.lamports, account.data)));
    }
    out
}

#[allow(clippy::too_many_arguments)]
pub fn create_job_ix_with(
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

pub fn create_job_ix(
    buyer: &Pubkey,
    mint: &Pubkey,
    job_id: [u8; 32],
    executor: Pubkey,
    amount: u64,
    deadline_slot: u64,
) -> Instruction {
    create_job_ix_with(buyer, mint, job_id, executor, amount, deadline_slot, admitted())
}

pub fn fund_ix(buyer: &Pubkey, job_id: [u8; 32], mint: &Pubkey, buyer_token: &Pubkey, amount: u64) -> Instruction {
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

pub fn deliver_ix(executor: &Pubkey, job_id: [u8; 32], artifact_hash: [u8; 32]) -> Instruction {
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

pub fn refund_ix(job_id: [u8; 32], mint: &Pubkey, buyer_token: &Pubkey) -> Instruction {
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

fn settle_accounts(job_id: [u8; 32], mint: &Pubkey, recipient_token: &Pubkey) -> Vec<AccountMeta> {
    let job = job_pda(&job_id);
    accounts::SettleWithProof {
        job,
        mint: *mint,
        vault: vault_pda(&job),
        recipient_token: *recipient_token,
        token_program: spl_token::ID,
        verifier_program: GROTH16_VERIFIER_ID,
        system_program: system_program::ID,
    }
    .to_account_metas(None)
}

pub fn release_ix(
    job_id: [u8; 32],
    mint: &Pubkey,
    recipient_token: &Pubkey,
    journal: Vec<u8>,
    seal: Groth16Seal,
) -> Instruction {
    Instruction {
        program_id: vericode_escrow::ID,
        accounts: settle_accounts(job_id, mint, recipient_token),
        data: instruction::Release { journal, seal }.data(),
    }
}

pub fn refund_on_fail_ix(
    job_id: [u8; 32],
    mint: &Pubkey,
    recipient_token: &Pubkey,
    journal: Vec<u8>,
    seal: Groth16Seal,
) -> Instruction {
    Instruction {
        program_id: vericode_escrow::ID,
        accounts: settle_accounts(job_id, mint, recipient_token),
        data: instruction::RefundOnFail { journal, seal }.data(),
    }
}

/// The escrow program alone, preferring its SBF build.
pub fn program_test() -> ProgramTest {
    let mut program_test = ProgramTest::new(PROGRAM_NAME, vericode_escrow::ID, None);
    program_test.prefer_bpf(true);
    program_test
}

/// The escrow and the Groth16 verifier of `risc0-solana v3.0.0` (commit
/// `ee415935`) at its fixed address. `SBF_OUT_DIR` must also hold
/// `groth_16_verifier.so`: the offline rebuild or the bytes dumped from
/// devnet.
pub fn verifier_program_test() -> ProgramTest {
    let mut program_test = program_test();
    program_test.add_program("groth_16_verifier", GROTH16_VERIFIER_ID, None);
    program_test
}

/// Adds the admitted Test USDC mint to genesis at [`ADMITTED_MINT`]: 6
/// decimals, no supply, `authority` as mint authority.
pub fn add_admitted_mint(program_test: &mut ProgramTest, authority: &Pubkey, freeze_authority: Option<Pubkey>) {
    let mint = spl_token::state::Mint {
        mint_authority: COption::Some(*authority),
        supply: 0,
        decimals: DECIMALS,
        is_initialized: true,
        freeze_authority: freeze_authority.map_or(COption::None, COption::Some),
    };
    let mut data = vec![0_u8; spl_token::state::Mint::LEN];
    spl_token::state::Mint::pack(mint, &mut data).unwrap();
    let lamports = Rent::default().minimum_balance(data.len());
    program_test.add_account_with_base64_data(ADMITTED_MINT, lamports, spl_token::ID, &base64(&data));
}

/// Starts the bank with the admitted mint in genesis and creates a funded
/// buyer, an executor and the buyer's associated token account.
pub async fn start(mut program_test: ProgramTest) -> Env {
    let mint_authority = Keypair::new();
    add_admitted_mint(&mut program_test, &mint_authority.pubkey(), None);
    let mut ctx = program_test.start_with_context().await;

    let buyer = Keypair::new();
    let executor = Keypair::new();
    transfer_lamports(&mut ctx, &buyer.pubkey(), 1_000_000_000).await;
    let mint = ADMITTED_MINT;
    let buyer_token = create_ata(&mut ctx, &buyer.pubkey(), &mint).await;
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

pub async fn setup() -> Env {
    start(program_test()).await
}

pub async fn create_job_with_deadline(env: &mut Env, job_id: [u8; 32], deadline_slot: u64) {
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

pub async fn create_default_job(env: &mut Env) {
    create_job_with_deadline(env, JOB_ID, DEADLINE).await;
}

pub async fn fund_job(env: &mut Env, job_id: [u8; 32]) {
    let ix = fund_ix(&env.buyer.pubkey(), job_id, &env.mint, &env.buyer_token, AMOUNT);
    let buyer = env.buyer.insecure_clone();
    send(&mut env.ctx, &[ix], &[&buyer]).await.unwrap();
}

pub async fn fund_default_job(env: &mut Env) {
    fund_job(env, JOB_ID).await;
}

pub async fn deliver_artifact(env: &mut Env, artifact_hash: [u8; 32]) {
    let executor = env.executor.insecure_clone();
    let ix = deliver_ix(&executor.pubkey(), JOB_ID, artifact_hash);
    send(&mut env.ctx, &[ix], &[&executor]).await.unwrap();
}

pub async fn deliver_default_job(env: &mut Env) {
    deliver_artifact(env, delivered_artifact_hash()).await;
}
