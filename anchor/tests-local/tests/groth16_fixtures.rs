//! Consistency checks of the versioned Groth16 fixtures (gate D2c.1).
//!
//! These checks bind the public vectors to the VeriCode Job used by the
//! escrow tests. They do not verify the Groth16 proof itself; that is done by
//! the Verifier Router in process (D2d spike, D2e program tests).

use std::{collections::HashMap, fs, path::PathBuf};

use anchor_lang::solana_program::hash::hash as sha256;
use vericode_core::{
    hash_harness_version, hash_restricted_artifact, hash_restricted_spec, Hash32, ImageId, JobId,
    JournalV1, JournalV1Commitments, RestrictedArtifactV1, Verdict,
    DETERMINISTIC_HARNESS_VERSION, JOURNAL_V1_CANDIDATE_WIRE_SIZE, RESTRICTED_SPEC_V1,
};

const JOB_ID: [u8; 32] = [0x11; 32];
const SELECTOR: [u8; 4] = [0x73, 0xc4, 0x57, 0xba];
const IMAGE_ID_HEX: &str = "4da06f90da75ec8980c943ce017d69c48370fddbf3aa27689d375d78fac0fb1a";
const PASS_SEAL_SHA256: &str = "4373517058bdf048484d2f57913740df9c611e113e9a6b212f41d905a9f578c2";
const FAIL_SEAL_SHA256: &str = "93a12d12a8ed406a69b54c1b91aa49bd1f67d663723188dd7b03888e2d343ec5";
const PASS_JOURNAL_SHA256: &str = "7c3f596eec21cafeef9aa7eab421582f97819ae7961aa1250e6526315b3f7de1";
const FAIL_JOURNAL_SHA256: &str = "29ff43b03973adf789f1426f31b3bbcd095ece94e3d984c92ec53be511127475";

fn decode_hex(text: &str) -> Vec<u8> {
    assert!(text.len() % 2 == 0, "odd hex length");
    (0..text.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&text[i..i + 2], 16).expect("hex digit"))
        .collect()
}

fn to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn load(name: &str) -> HashMap<String, Vec<u8>> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/groth16")
        .join(format!("{name}.txt"));
    fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("missing fixture {}: {e}", path.display()))
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
        .map(|line| {
            let (key, value) = line.split_once('=').expect("key=hex");
            (key.to_string(), decode_hex(value.trim()))
        })
        .collect()
}

fn check(name: &str, claimed_output: u32, verdict: Verdict, seal_sha: &str, journal_sha: &str) {
    let fixture = load(name);
    let field = |key: &str| fixture.get(key).unwrap_or_else(|| panic!("{name}: missing {key}"));

    assert_eq!(field("selector").as_slice(), &SELECTOR, "{name}: selector");
    assert_eq!(field("seal").len(), 256, "{name}: seal length");
    assert_eq!(to_hex(&sha256(field("seal")).to_bytes()), seal_sha, "{name}: seal hash");

    let journal_bytes = field("journal");
    assert_eq!(journal_bytes.len(), JOURNAL_V1_CANDIDATE_WIRE_SIZE, "{name}: journal length");
    assert_eq!(to_hex(&sha256(journal_bytes).to_bytes()), journal_sha, "{name}: journal hash");
    assert_eq!(
        sha256(journal_bytes).to_bytes().as_slice(),
        field("journal_digest").as_slice(),
        "{name}: journal_digest must be SHA-256(journal)"
    );

    let image_id: [u8; 32] = field("image_id").as_slice().try_into().expect("image_id length");
    assert_eq!(to_hex(&image_id), IMAGE_ID_HEX, "{name}: image_id");

    let journal = JournalV1::decode_candidate(journal_bytes).expect("decode JournalV1");
    let artifact = hash_restricted_artifact(&RestrictedArtifactV1::new(7, claimed_output)).unwrap();
    let expected = JournalV1Commitments::new(
        JobId::new(JOB_ID),
        hash_restricted_spec(&RESTRICTED_SPEC_V1).unwrap(),
        hash_harness_version(DETERMINISTIC_HARNESS_VERSION).unwrap(),
        artifact,
        ImageId::new(image_id),
    );
    assert_eq!(journal.validate_against(&expected), Ok(verdict), "{name}: journal binding");
    assert_eq!(journal.artifact_hash(), artifact);
    assert_ne!(journal.artifact_hash(), Hash32::new([0; 32]));
}

#[test]
fn pass_fixture_is_bound_to_the_vericode_job() {
    check("pass", 14, Verdict::Pass, PASS_SEAL_SHA256, PASS_JOURNAL_SHA256);
}

#[test]
fn fail_fixture_is_bound_to_the_vericode_job() {
    check("fail", 15, Verdict::Fail, FAIL_SEAL_SHA256, FAIL_JOURNAL_SHA256);
}
