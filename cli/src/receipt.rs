//! Receipt directory written by `vericode-prover compress`.

use std::{fs, path::Path};

use vericode_core::{JournalV1, Verdict, JOURNAL_V1_CANDIDATE_WIRE_SIZE};

use crate::{escrow::Groth16Seal, hex, sha256};

/// Public vectors of one Groth16 receipt.
#[derive(Clone, Debug)]
pub struct Receipt {
    pub journal: Vec<u8>,
    pub decoded: JournalV1,
    /// Raw 256-byte seal, `pi_a` not negated.
    pub seal: [u8; 256],
    pub selector: [u8; 4],
}

fn read(dir: &Path, name: &str) -> Result<Vec<u8>, String> {
    fs::read(dir.join(name)).map_err(|error| format!("{}: {error}", dir.join(name).display()))
}

impl Receipt {
    /// Loads `journal`, `seal` and `selector`, and checks `image_id` and
    /// `journal_digest` against the journal when they are present.
    pub fn load(dir: &Path) -> Result<Self, String> {
        let journal = read(dir, "journal")?;
        if journal.len() != JOURNAL_V1_CANDIDATE_WIRE_SIZE {
            return Err(format!("journal has {} bytes, expected {JOURNAL_V1_CANDIDATE_WIRE_SIZE}", journal.len()));
        }
        let decoded = JournalV1::decode_candidate(&journal).map_err(|error| format!("journal: {error:?}"))?;
        let seal: [u8; 256] = read(dir, "seal")?
            .try_into()
            .map_err(|bytes: Vec<u8>| format!("seal has {} bytes, expected 256", bytes.len()))?;
        let selector: [u8; 4] = read(dir, "selector")?
            .try_into()
            .map_err(|bytes: Vec<u8>| format!("selector has {} bytes, expected 4", bytes.len()))?;
        if dir.join("journal_digest").exists() && read(dir, "journal_digest")? != sha256(&journal) {
            return Err("journal_digest is not SHA-256(journal)".into());
        }
        if dir.join("image_id").exists() && read(dir, "image_id")? != decoded.image_id().as_hash().as_bytes() {
            return Err("image_id file differs from the journal image_id".into());
        }
        Ok(Self {
            journal,
            decoded,
            seal,
            selector,
        })
    }

    pub fn job_id(&self) -> [u8; 32] {
        *self.decoded.job_id().as_hash().as_bytes()
    }

    pub fn artifact_hash(&self) -> [u8; 32] {
        self.decoded.artifact_hash().into_bytes()
    }

    pub fn image_id(&self) -> [u8; 32] {
        *self.decoded.image_id().as_hash().as_bytes()
    }

    pub fn verdict(&self) -> Verdict {
        self.decoded.verdict()
    }

    /// Seal as the program argument (`pi_a` negated).
    pub fn seal(&self) -> Groth16Seal {
        Groth16Seal::from_raw(self.selector, &self.seal)
    }

    pub fn describe(&self) -> Vec<String> {
        vec![
            format!("receipt.job_id={}", hex(&self.job_id())),
            format!("receipt.spec_hash={}", hex(self.decoded.spec_hash().as_bytes())),
            format!("receipt.harness_hash={}", hex(self.decoded.harness_hash().as_bytes())),
            format!("receipt.artifact_hash={}", hex(&self.artifact_hash())),
            format!("receipt.image_id={}", hex(&self.image_id())),
            format!("receipt.verdict={:?}", self.verdict()),
            format!("receipt.selector={}", hex(&self.selector)),
            format!("receipt.journal_sha256={}", hex(&sha256(&self.journal))),
            format!("receipt.seal_sha256={}", hex(&sha256(&self.seal))),
        ]
    }
}
