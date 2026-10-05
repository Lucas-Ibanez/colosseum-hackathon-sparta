//! Host checks of the stable interface of `vericode_escrow` (gates D2b.1,
//! D2e, D4a).
//!
//! Error codes, `EscrowStatus` Borsh tags and the `JobAccount` size are part
//! of the on-chain interface. They are compared here with literals, so a
//! reordered or inserted variant fails instead of shifting silently. No
//! program is executed.

use std::{fs, path::PathBuf};

use anchor_lang::solana_program::hash::hash as sha256;
use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_lang::{AnchorDeserialize, AnchorSerialize, Space};
use vericode_core::escrow::{
    EscrowState, RefundReason, MAX_DEADLINE_WINDOW_SLOTS, MIN_DEADLINE_WINDOW_SLOTS,
};
use vericode_core::{
    hash_harness_version, hash_restricted_spec, Hash32, DETERMINISTIC_HARNESS_VERSION,
    RESTRICTED_SPEC_V1,
};
use vericode_escrow::{
    EscrowStatus, Groth16Seal, JobAccount, VericodeEscrowError, ADMITTED_IMAGE_ID_V1,
    ADMITTED_MINT, ATA_PROGRAM_ID, GROTH16_SELECTOR, GROTH16_VERIFIER_ID, VERIFY_DISCRIMINATOR,
};

const IMAGE_ID_HEX: &str = "4da06f90da75ec8980c943ce017d69c48370fddbf3aa27689d375d78fac0fb1a";
const SPEC_HASH_HEX: &str = "af642b561ac73a0f78ae767123f95a0c6ece6ce89f2a5e5e20cb86bec49fb778";
const HARNESS_HASH_HEX: &str = "01124025c6ad84bb8490f216e95ff241d862dc2faf316d0e28bcdb55b0996b50";

fn to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[test]
fn error_codes_are_stable_literals() {
    use VericodeEscrowError::*;
    let codes = [
        (AmountZero, 6000),
        (ZeroBuyer, 6001),
        (ZeroExecutor, 6002),
        (ZeroMint, 6003),
        (BuyerIsExecutor, 6004),
        (NotFunded, 6005),
        (AlreadyFunded, 6006),
        (AlreadyReleased, 6007),
        (AlreadyRefunded, 6008),
        (DepositorMismatch, 6009),
        (RecipientMismatch, 6010),
        (MintMismatch, 6011),
        (AmountMismatch, 6012),
        (JournalSchemaVersionMismatch, 6013),
        (JournalJobIdMismatch, 6014),
        (JournalSpecHashMismatch, 6015),
        (JournalHarnessHashMismatch, 6016),
        (JournalArtifactHashMismatch, 6017),
        (JournalImageIdMismatch, 6018),
        (VerdictNotPass, 6019),
        (VerdictNotFail, 6020),
        (DeadlineNotReached, 6021),
        (DeadlinePassed, 6022),
        (UnsupportedAccountVersion, 6023),
        (MintHasFreezeAuthority, 6024),
        (NotDelivered, 6025),
        (AlreadyDelivered, 6026),
        (DelivererMismatch, 6027),
        (SpecNotAdmitted, 6028),
        (HarnessNotAdmitted, 6029),
        (ImageIdNotAdmitted, 6030),
        (DeadlineOutOfWindow, 6031),
        (ExecutorIsProgramAccount, 6032),
        (UnexpectedSelector, 6033),
        (JournalMalformed, 6034),
        (DestinationNotCanonical, 6035),
        (MintNotAdmitted, 6036),
    ];
    for (error, code) in codes {
        let name = format!("{error:?}");
        assert_eq!(u32::from(error), code, "{name}");
    }
}

#[test]
fn escrow_status_round_trips_with_stable_borsh_tags() {
    let hash = [0xab; 32];
    let cases = [
        (EscrowStatus::Created, EscrowState::Created, 0_u8, 1_usize),
        (EscrowStatus::Funded, EscrowState::Funded, 1, 1),
        (
            EscrowStatus::Released { artifact_hash: hash },
            EscrowState::Released {
                artifact_hash: Hash32::new(hash),
            },
            2,
            33,
        ),
        (
            EscrowStatus::RefundedOnFail { artifact_hash: hash },
            EscrowState::Refunded {
                reason: RefundReason::Fail {
                    artifact_hash: Hash32::new(hash),
                },
            },
            3,
            33,
        ),
        (
            EscrowStatus::RefundedOnTimeout,
            EscrowState::Refunded {
                reason: RefundReason::Timeout,
            },
            4,
            1,
        ),
        (
            EscrowStatus::Delivered { artifact_hash: hash },
            EscrowState::Delivered {
                artifact_hash: Hash32::new(hash),
            },
            5,
            33,
        ),
    ];
    for (status, state, tag, len) in cases {
        assert_eq!(status.to_core(), state);
        assert_eq!(EscrowStatus::from_core(state), status);

        let mut bytes = Vec::new();
        status.serialize(&mut bytes).unwrap();
        assert_eq!(bytes[0], tag, "{status:?}");
        assert_eq!(bytes.len(), len, "{status:?}");
        if len == 33 {
            assert_eq!(&bytes[1..], &hash);
        }
        assert_eq!(EscrowStatus::deserialize(&mut bytes.as_slice()).unwrap(), status);
    }
}

#[test]
fn job_account_space_is_unchanged() {
    assert_eq!(EscrowStatus::INIT_SPACE, 33);
    assert_eq!(JobAccount::INIT_SPACE, 276);
}

#[test]
fn admitted_terms_are_the_decided_v1_values() {
    assert_eq!(to_hex(&ADMITTED_IMAGE_ID_V1), IMAGE_ID_HEX);
    assert_eq!(
        to_hex(hash_restricted_spec(&RESTRICTED_SPEC_V1).unwrap().as_bytes()),
        SPEC_HASH_HEX
    );
    assert_eq!(
        to_hex(hash_harness_version(DETERMINISTIC_HARNESS_VERSION).unwrap().as_bytes()),
        HARNESS_HASH_HEX
    );
    assert_eq!(MIN_DEADLINE_WINDOW_SLOTS, 1_500);
    assert_eq!(MAX_DEADLINE_WINDOW_SLOTS, 1_512_000);

    // The versioned Groth16 fixtures were proved by the admitted guest.
    for name in ["pass", "fail"] {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("fixtures/groth16")
            .join(format!("{name}.txt"));
        let text = fs::read_to_string(&path).unwrap();
        let image_id = text
            .lines()
            .find_map(|line| line.strip_prefix("image_id="))
            .expect("image_id line");
        assert_eq!(image_id.trim(), IMAGE_ID_HEX, "{name}");
    }
}

#[test]
fn verifier_is_the_pinned_release() {
    // `risc0-solana v3.0.0` (commit `ee415935`) Groth16 verifier, deployed on
    // devnet with upgrade authority `None` (D4a). No Verifier Router.
    assert_eq!(GROTH16_VERIFIER_ID.to_string(), "THq1qFYQoh7zgcjXoMXduDBqiZRCPeg3PvvMbrVQUge");
    assert_eq!(ATA_PROGRAM_ID.to_string(), "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
    assert_eq!(GROTH16_SELECTOR, [0x73, 0xc4, 0x57, 0xba]);
    assert_eq!(VERIFY_DISCRIMINATOR, sha256(b"global:verify").to_bytes()[..8]);

    // The versioned Groth16 fixtures use the fixed selector.
    for name in ["pass", "fail"] {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("fixtures/groth16")
            .join(format!("{name}.txt"));
        let text = fs::read_to_string(&path).unwrap();
        let selector = text
            .lines()
            .find_map(|line| line.strip_prefix("selector="))
            .expect("selector line");
        assert_eq!(selector.trim(), to_hex(&GROTH16_SELECTOR), "{name}");
    }
}

#[test]
fn admitted_mint_is_the_devnet_test_usdc() {
    // Public key only; the mint keypair stays outside the repository (D4a).
    assert_eq!(ADMITTED_MINT.to_string(), "9TE2VPFmgrNxT22yS3sEZyRcMxLgJkwzgAoquWRXwV2F");
    assert_ne!(ADMITTED_MINT, Pubkey::default());
}

#[test]
fn groth16_seal_has_the_borsh_layout_of_the_upstream_seal() {
    // `Seal { selector: [u8; 4], proof: Proof { pi_a, pi_b, pi_c } }`.
    let seal = Groth16Seal {
        selector: [1; 4],
        pi_a: [2; 64],
        pi_b: [3; 128],
        pi_c: [4; 64],
    };
    let mut bytes = Vec::new();
    seal.serialize(&mut bytes).unwrap();
    let mut expected = vec![1; 4];
    expected.extend_from_slice(&[2; 64]);
    expected.extend_from_slice(&[3; 128]);
    expected.extend_from_slice(&[4; 64]);
    assert_eq!(bytes, expected);
}
