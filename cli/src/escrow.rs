//! Constants, instruction builders and account decoders of the deployed
//! `vericode_escrow` program.
//!
//! Instructions are assembled by hand from the account lists and
//! discriminators of the D4a IDL (`e8ce2c20…`); `tests/instructions.rs`
//! checks every byte against the Anchor builders of
//! `anchor/tests-local/tests/common/mod.rs`. Commitments come from
//! `vericode-core`, never from this client.

use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;
use vericode_core::{
    hash_harness_version, hash_restricted_artifact, hash_restricted_spec, RestrictedArtifactV1,
    DETERMINISTIC_HARNESS_VERSION, RESTRICTED_SPEC_V1,
};

/// Escrow program on devnet; immutable since D4b (upgrade authority `none`).
pub const PROGRAM_ID: Pubkey = Pubkey::from_str_const("GZqbL2TbeDVHcNRosngaRfCwzV9YJT6iEbckYr8uwkCH");
pub const PROGRAM_DATA_ID: Pubkey = Pubkey::from_str_const("B7s9JJVyD2j8PNgKgjhhB36iSdX9cUpSfZHbbd8nLmbc");
/// SHA-256 and length of the deployed program bytes (D4a `.so`, D4b dump).
pub const PROGRAM_SHA256: &str = "cdf6967f3abc63d0385e36909fe61b36f639203e2679209be60c4875114d8133";
pub const PROGRAM_LEN: usize = 395_064;
/// RISC Zero Groth16 verifier of `risc0-solana v3.0.0`; immutable on devnet.
pub const VERIFIER_ID: Pubkey = Pubkey::from_str_const("THq1qFYQoh7zgcjXoMXduDBqiZRCPeg3PvvMbrVQUge");
pub const VERIFIER_PROGRAM_DATA_ID: Pubkey = Pubkey::from_str_const("ENdLkqHpzKcrQy4XzFUhkN7H3C3Mz1cugjJBpiNEDxpn");
/// SHA-256 and length of the verifier program bytes on devnet (D4a dump; the
/// `.so` the suite loads from devnet).
pub const VERIFIER_PROGRAM_SHA256: &str = "34ae6e5c9d63dfe67c48fa04cad04e9752ad9f1cfbc8b4d66e99941df7666cd1";
pub const VERIFIER_PROGRAM_LEN: usize = 199_256;
/// The only mint a Job admits: devnet Test USDC (6 decimals, no freeze authority).
pub const ADMITTED_MINT: Pubkey = Pubkey::from_str_const("9TE2VPFmgrNxT22yS3sEZyRcMxLgJkwzgAoquWRXwV2F");
pub const MINT_DECIMALS: u8 = 6;
pub const TOKEN_PROGRAM_ID: Pubkey = Pubkey::from_str_const("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
pub const ATA_PROGRAM_ID: Pubkey = Pubkey::from_str_const("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
pub const SYSTEM_PROGRAM_ID: Pubkey = Pubkey::from_str_const("11111111111111111111111111111111");
pub const LOADER_V3_ID: Pubkey = Pubkey::from_str_const("BPFLoaderUpgradeab1e11111111111111111111111");
/// Genesis hash of Solana devnet: the client refuses any other cluster.
pub const DEVNET_GENESIS_HASH: &str = "EtWTRABZaYq6iMfeYKouRu166VU2xqa1wcaWoxPkrZBG";

/// `ADMITTED_IMAGE_ID_V1`: ImageID of the deterministic D1c2b guest.
pub const ADMITTED_IMAGE_ID_V1: [u8; 32] = [
    0x4d, 0xa0, 0x6f, 0x90, 0xda, 0x75, 0xec, 0x89, 0x80, 0xc9, 0x43, 0xce, 0x01, 0x7d, 0x69, 0xc4,
    0x83, 0x70, 0xfd, 0xdb, 0xf3, 0xaa, 0x27, 0x68, 0x9d, 0x37, 0x5d, 0x78, 0xfa, 0xc0, 0xfb, 0x1a,
];
/// Admitted v1 specification and harness commitments, as published in
/// `docs/manifest-schema.md`; [`admitted_terms`] recomputes them with the core.
pub const SPEC_HASH_V1_HEX: &str = "af642b561ac73a0f78ae767123f95a0c6ece6ce89f2a5e5e20cb86bec49fb778";
pub const HARNESS_HASH_V1_HEX: &str = "01124025c6ad84bb8490f216e95ff241d862dc2faf316d0e28bcdb55b0996b50";
/// Selector of the Groth16 verifier parameters (`risc0-zkvm 3.0.3`).
pub const GROTH16_SELECTOR: [u8; 4] = [0x73, 0xc4, 0x57, 0xba];

pub const JOB_SEED: &[u8] = b"job";
pub const VAULT_SEED: &[u8] = b"vault";
/// Size of the Job account: 8-byte discriminator + `JobAccount::INIT_SPACE`.
pub const JOB_ACCOUNT_LEN: usize = 8 + 276;
pub const TOKEN_ACCOUNT_LEN: usize = 165;
pub const MINT_LEN: usize = 82;
/// Header of an upgradeable-loader ProgramData account.
pub const PROGRAM_DATA_HEADER_LEN: usize = 45;
/// Wire limit of a Solana transaction.
pub const MAX_TRANSACTION_SIZE: usize = 1_232;
/// Creation window of the core, in slots, and the client margin for the
/// slots that pass between reading the slot and landing the transaction.
pub const MIN_DEADLINE_WINDOW_SLOTS: u64 = vericode_core::escrow::MIN_DEADLINE_WINDOW_SLOTS;
pub const MAX_DEADLINE_WINDOW_SLOTS: u64 = vericode_core::escrow::MAX_DEADLINE_WINDOW_SLOTS;
pub const LANDING_MARGIN_SLOTS: u64 = 60;

/// Why `slot` leaves no room to land `action`, which the program accepts only
/// up to `deadline_slot` inclusive; `None` when at least
/// `LANDING_MARGIN_SLOTS` remain.
pub fn deadline_margin_problem(slot: u64, deadline_slot: u64, action: &str) -> Option<String> {
    if slot.saturating_add(LANDING_MARGIN_SLOTS) <= deadline_slot {
        None
    } else if slot > deadline_slot {
        Some(format!("slot {slot} is past the deadline {deadline_slot}; the program no longer accepts {action}"))
    } else {
        Some(format!("slot {slot} is too close to the deadline {deadline_slot} for {action}"))
    }
}

/// Anchor discriminators (`sha256("global:<name>")[..8]`, IDL `e8ce2c20…`).
pub const CREATE_JOB_DISCRIMINATOR: [u8; 8] = [0xb2, 0x82, 0xd9, 0x6e, 0x64, 0x1b, 0x52, 0x77];
pub const FUND_DISCRIMINATOR: [u8; 8] = [0xda, 0xbc, 0x6f, 0xdd, 0x98, 0x71, 0xae, 0x07];
pub const DELIVER_DISCRIMINATOR: [u8; 8] = [0xfa, 0x83, 0xde, 0x39, 0xd3, 0xe5, 0xd1, 0x93];
pub const RELEASE_DISCRIMINATOR: [u8; 8] = [0xfd, 0xf9, 0x0f, 0xce, 0x1c, 0x7f, 0xc1, 0xf1];
pub const REFUND_ON_FAIL_DISCRIMINATOR: [u8; 8] = [0xdd, 0xfd, 0x3d, 0x7e, 0x22, 0x01, 0x52, 0x9d];
pub const REFUND_ON_TIMEOUT_DISCRIMINATOR: [u8; 8] = [0x9a, 0xe2, 0xda, 0x99, 0x66, 0xb9, 0xca, 0x02];
/// `sha256("account:JobAccount")[..8]`.
pub const JOB_ACCOUNT_DISCRIMINATOR: [u8; 8] = [0x5b, 0x10, 0xa2, 0x05, 0x2d, 0xd2, 0x7d, 0x41];

/// Commitments a v1 Job must carry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Terms {
    pub spec_hash: [u8; 32],
    pub harness_hash: [u8; 32],
    pub image_id: [u8; 32],
}

/// The admitted v1 terms, computed by the core; panics only if the core
/// disagrees with the published values, which would be a build defect.
pub fn admitted_terms() -> Terms {
    let spec_hash = hash_restricted_spec(&RESTRICTED_SPEC_V1).expect("spec commitment").into_bytes();
    let harness_hash = hash_harness_version(DETERMINISTIC_HARNESS_VERSION)
        .expect("harness commitment")
        .into_bytes();
    assert_eq!(crate::hex(&spec_hash), SPEC_HASH_V1_HEX, "core spec_hash is not the admitted v1 value");
    assert_eq!(crate::hex(&harness_hash), HARNESS_HASH_V1_HEX, "core harness_hash is not the admitted v1 value");
    Terms {
        spec_hash,
        harness_hash,
        image_id: ADMITTED_IMAGE_ID_V1,
    }
}

/// `hash_restricted_artifact` of `(input, claimed_output)`.
pub fn artifact_hash(input: u32, claimed_output: u32) -> [u8; 32] {
    hash_restricted_artifact(&RestrictedArtifactV1::new(input, claimed_output))
        .expect("artifact commitment")
        .into_bytes()
}

pub fn job_pda(job_id: &[u8; 32]) -> Pubkey {
    Pubkey::find_program_address(&[JOB_SEED, job_id], &PROGRAM_ID).0
}

pub fn vault_pda(job: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[VAULT_SEED, job.as_ref()], &PROGRAM_ID).0
}

/// Canonical associated token account of `owner` for the admitted mint: the
/// only destination the escrow pays.
pub fn ata(owner: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[owner.as_ref(), TOKEN_PROGRAM_ID.as_ref(), ADMITTED_MINT.as_ref()],
        &ATA_PROGRAM_ID,
    )
    .0
}

/// Groth16 seal in the Borsh layout of the program argument `Groth16Seal`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Groth16Seal {
    pub selector: [u8; 4],
    pub pi_a: [u8; 64],
    pub pi_b: [u8; 128],
    pub pi_c: [u8; 64],
}

impl Groth16Seal {
    /// Encodes a raw 256-byte Groth16 seal as the verifier expects it: `pi_a`
    /// negated, as the upstream client does.
    pub fn from_raw(selector: [u8; 4], raw: &[u8; 256]) -> Self {
        Self {
            selector,
            pi_a: negate_g1(&raw[0..64]),
            pi_b: raw[64..192].try_into().expect("128 bytes"),
            pi_c: raw[192..256].try_into().expect("64 bytes"),
        }
    }

    /// The same seal with one bit of `pi_c` flipped (`pi_c[10] ^= 1`), the
    /// mutation of the suite (`d4b_receipts.rs`). Only for `--tamper-seal`
    /// negative runs: the verifier must reject it.
    pub fn tampered(&self) -> Self {
        let mut seal = *self;
        seal.pi_c[10] ^= 0x01;
        seal
    }

    fn encode(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.selector);
        out.extend_from_slice(&self.pi_a);
        out.extend_from_slice(&self.pi_b);
        out.extend_from_slice(&self.pi_c);
    }
}

// BN254 base field modulus `q`, big-endian.
const BN254_BASE_FIELD_MODULUS: [u8; 32] = [
    0x30, 0x64, 0x4e, 0x72, 0xe1, 0x31, 0xa0, 0x29, 0xb8, 0x50, 0x45, 0xb6, 0x81, 0x81, 0x58, 0x5d,
    0x97, 0x81, 0x6a, 0x91, 0x68, 0x71, 0xca, 0x8d, 0x3c, 0x20, 0x8c, 0x16, 0xd8, 0x7c, 0xfd, 0x47,
];

/// Negates a BN254 G1 point `(x, y)` encoded as two 32-byte big-endian
/// coordinates: `(x, q - y)`.
pub fn negate_g1(point: &[u8]) -> [u8; 64] {
    let mut out = [0_u8; 64];
    out[..32].copy_from_slice(&point[..32]);
    let mut borrow = 0_i16;
    for index in (0..32).rev() {
        let difference = i16::from(BN254_BASE_FIELD_MODULUS[index]) - i16::from(point[32 + index]) - borrow;
        out[32 + index] = difference.rem_euclid(256) as u8;
        borrow = i16::from(difference < 0);
    }
    out
}

pub fn create_job_ix(
    buyer: &Pubkey,
    job_id: [u8; 32],
    executor: &Pubkey,
    amount: u64,
    deadline_slot: u64,
    terms: &Terms,
) -> Instruction {
    let job = job_pda(&job_id);
    let mut data = CREATE_JOB_DISCRIMINATOR.to_vec();
    data.extend_from_slice(&job_id);
    data.extend_from_slice(executor.as_ref());
    data.extend_from_slice(&amount.to_le_bytes());
    data.extend_from_slice(&deadline_slot.to_le_bytes());
    data.extend_from_slice(&terms.spec_hash);
    data.extend_from_slice(&terms.harness_hash);
    data.extend_from_slice(&terms.image_id);
    Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(*buyer, true),
            AccountMeta::new_readonly(ADMITTED_MINT, false),
            AccountMeta::new(job, false),
            AccountMeta::new(vault_pda(&job), false),
            AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false),
            AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false),
        ],
        data,
    }
}

pub fn fund_ix(buyer: &Pubkey, job_id: [u8; 32], amount: u64) -> Instruction {
    let job = job_pda(&job_id);
    let mut data = FUND_DISCRIMINATOR.to_vec();
    data.extend_from_slice(&amount.to_le_bytes());
    Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            AccountMeta::new_readonly(*buyer, true),
            AccountMeta::new(job, false),
            AccountMeta::new_readonly(ADMITTED_MINT, false),
            AccountMeta::new(ata(buyer), false),
            AccountMeta::new(vault_pda(&job), false),
            AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false),
        ],
        data,
    }
}

pub fn deliver_ix(executor: &Pubkey, job_id: [u8; 32], artifact_hash: [u8; 32]) -> Instruction {
    let mut data = DELIVER_DISCRIMINATOR.to_vec();
    data.extend_from_slice(&artifact_hash);
    Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            AccountMeta::new_readonly(*executor, true),
            AccountMeta::new(job_pda(&job_id), false),
        ],
        data,
    }
}

fn settle_ix(
    discriminator: [u8; 8],
    job_id: [u8; 32],
    recipient_token: &Pubkey,
    journal: &[u8],
    seal: &Groth16Seal,
) -> Instruction {
    let job = job_pda(&job_id);
    let mut data = discriminator.to_vec();
    data.extend_from_slice(&u32::try_from(journal.len()).expect("journal length").to_le_bytes());
    data.extend_from_slice(journal);
    seal.encode(&mut data);
    Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(job, false),
            AccountMeta::new_readonly(ADMITTED_MINT, false),
            AccountMeta::new(vault_pda(&job), false),
            AccountMeta::new(*recipient_token, false),
            AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false),
            AccountMeta::new_readonly(VERIFIER_ID, false),
            AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false),
        ],
        data,
    }
}

/// `release(journal, seal)`; `recipient_token` must be the executor's ATA.
pub fn release_ix(job_id: [u8; 32], recipient_token: &Pubkey, journal: &[u8], seal: &Groth16Seal) -> Instruction {
    settle_ix(RELEASE_DISCRIMINATOR, job_id, recipient_token, journal, seal)
}

/// `refund_on_fail(journal, seal)`; `recipient_token` must be the buyer's ATA.
pub fn refund_on_fail_ix(job_id: [u8; 32], recipient_token: &Pubkey, journal: &[u8], seal: &Groth16Seal) -> Instruction {
    settle_ix(REFUND_ON_FAIL_DISCRIMINATOR, job_id, recipient_token, journal, seal)
}

pub fn refund_on_timeout_ix(job_id: [u8; 32], buyer_token: &Pubkey) -> Instruction {
    let job = job_pda(&job_id);
    Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(job, false),
            AccountMeta::new_readonly(ADMITTED_MINT, false),
            AccountMeta::new(vault_pda(&job), false),
            AccountMeta::new(*buyer_token, false),
            AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false),
        ],
        data: REFUND_ON_TIMEOUT_DISCRIMINATOR.to_vec(),
    }
}

/// `CreateIdempotent` of the Associated Token Account program for `owner`
/// and the admitted mint: succeeds without change if the account exists.
pub fn create_ata_idempotent_ix(payer: &Pubkey, owner: &Pubkey) -> Instruction {
    Instruction {
        program_id: ATA_PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(*payer, true),
            AccountMeta::new(ata(owner), false),
            AccountMeta::new_readonly(*owner, false),
            AccountMeta::new_readonly(ADMITTED_MINT, false),
            AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false),
            AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false),
        ],
        data: vec![1],
    }
}

/// Escrow status of a Job account (`EscrowStatus`, Borsh tags 0 to 5).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Status {
    Created,
    Funded,
    Released { artifact_hash: [u8; 32] },
    RefundedOnFail { artifact_hash: [u8; 32] },
    RefundedOnTimeout,
    Delivered { artifact_hash: [u8; 32] },
}

impl Status {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Created => "Created",
            Self::Funded => "Funded",
            Self::Released { .. } => "Released",
            Self::RefundedOnFail { .. } => "RefundedOnFail",
            Self::RefundedOnTimeout => "RefundedOnTimeout",
            Self::Delivered { .. } => "Delivered",
        }
    }

    pub fn artifact_hash(&self) -> Option<[u8; 32]> {
        match self {
            Self::Released { artifact_hash }
            | Self::RefundedOnFail { artifact_hash }
            | Self::Delivered { artifact_hash } => Some(*artifact_hash),
            _ => None,
        }
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Released { .. } | Self::RefundedOnFail { .. } | Self::RefundedOnTimeout)
    }
}

/// Decoded `JobAccount`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JobView {
    pub version: u8,
    pub bump: u8,
    pub vault_bump: u8,
    pub job_id: [u8; 32],
    pub buyer: Pubkey,
    pub executor: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub deadline_slot: u64,
    pub spec_hash: [u8; 32],
    pub harness_hash: [u8; 32],
    pub image_id: [u8; 32],
    pub status: Status,
}

fn array<const N: usize>(data: &[u8], offset: usize) -> [u8; N] {
    data[offset..offset + N].try_into().expect("slice length")
}

fn pubkey_at(data: &[u8], offset: usize) -> Pubkey {
    Pubkey::new_from_array(array(data, offset))
}

fn u64_at(data: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes(array(data, offset))
}

/// Decodes a Job account; requires the exact length and discriminator.
pub fn decode_job(data: &[u8]) -> Result<JobView, String> {
    if data.len() != JOB_ACCOUNT_LEN {
        return Err(format!("Job account has {} bytes, expected {JOB_ACCOUNT_LEN}", data.len()));
    }
    if data[..8] != JOB_ACCOUNT_DISCRIMINATOR {
        return Err("account is not a JobAccount (discriminator)".into());
    }
    let artifact_hash = array(data, 252);
    let status = match data[251] {
        0 => Status::Created,
        1 => Status::Funded,
        2 => Status::Released { artifact_hash },
        3 => Status::RefundedOnFail { artifact_hash },
        4 => Status::RefundedOnTimeout,
        5 => Status::Delivered { artifact_hash },
        tag => return Err(format!("unknown escrow status tag {tag}")),
    };
    Ok(JobView {
        version: data[8],
        bump: data[9],
        vault_bump: data[10],
        job_id: array(data, 11),
        buyer: pubkey_at(data, 43),
        executor: pubkey_at(data, 75),
        mint: pubkey_at(data, 107),
        amount: u64_at(data, 139),
        deadline_slot: u64_at(data, 147),
        spec_hash: array(data, 155),
        harness_hash: array(data, 187),
        image_id: array(data, 219),
        status,
    })
}

fn coption_pubkey(data: &[u8], offset: usize) -> Result<Option<Pubkey>, String> {
    match u32::from_le_bytes(array(data, offset)) {
        0 => Ok(None),
        1 => Ok(Some(pubkey_at(data, offset + 4))),
        tag => Err(format!("invalid COption tag {tag}")),
    }
}

/// Decoded SPL Token account (165 bytes).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TokenAccountView {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
    pub delegate: Option<Pubkey>,
    pub state: u8,
    pub is_native: Option<u64>,
    pub delegated_amount: u64,
    pub close_authority: Option<Pubkey>,
}

pub fn decode_token_account(data: &[u8]) -> Result<TokenAccountView, String> {
    if data.len() != TOKEN_ACCOUNT_LEN {
        return Err(format!("token account has {} bytes, expected {TOKEN_ACCOUNT_LEN}", data.len()));
    }
    let is_native = match u32::from_le_bytes(array(data, 109)) {
        0 => None,
        1 => Some(u64_at(data, 113)),
        tag => return Err(format!("invalid COption tag {tag}")),
    };
    Ok(TokenAccountView {
        mint: pubkey_at(data, 0),
        owner: pubkey_at(data, 32),
        amount: u64_at(data, 64),
        delegate: coption_pubkey(data, 72)?,
        state: data[108],
        is_native,
        delegated_amount: u64_at(data, 121),
        close_authority: coption_pubkey(data, 129)?,
    })
}

/// Decoded SPL Token mint (82 bytes).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MintView {
    pub mint_authority: Option<Pubkey>,
    pub supply: u64,
    pub decimals: u8,
    pub is_initialized: bool,
    pub freeze_authority: Option<Pubkey>,
}

pub fn decode_mint(data: &[u8]) -> Result<MintView, String> {
    if data.len() != MINT_LEN {
        return Err(format!("mint has {} bytes, expected {MINT_LEN}", data.len()));
    }
    Ok(MintView {
        mint_authority: coption_pubkey(data, 0)?,
        supply: u64_at(data, 36),
        decimals: data[44],
        is_initialized: data[45] == 1,
        freeze_authority: coption_pubkey(data, 46)?,
    })
}

/// `UpgradeableLoaderState::Program { programdata_address }`.
pub fn decode_program_account(data: &[u8]) -> Result<Pubkey, String> {
    if data.len() != 36 || u32::from_le_bytes(array(data, 0)) != 2 {
        return Err("account is not an upgradeable program".into());
    }
    Ok(pubkey_at(data, 4))
}

/// Header of `UpgradeableLoaderState::ProgramData { slot, upgrade_authority }`.
pub fn decode_program_data_header(data: &[u8]) -> Result<(u64, Option<Pubkey>), String> {
    if data.len() < PROGRAM_DATA_HEADER_LEN || u32::from_le_bytes(array(data, 0)) != 3 {
        return Err("account is not upgradeable ProgramData".into());
    }
    let authority = match data[12] {
        0 => None,
        1 => Some(pubkey_at(data, 13)),
        tag => return Err(format!("invalid Option tag {tag}")),
    };
    Ok((u64_at(data, 4), authority))
}
