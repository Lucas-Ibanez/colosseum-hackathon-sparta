//! Pure deterministic core for the restricted VeriCode MVP.
//!
//! The crate defines the semantic `JournalV1`, a local candidate Borsh wire
//! format, canonical SHA-256 commitments, and one deliberately restricted
//! development harness. It has no payment authority and no blockchain or
//! zkVM dependency.

#![forbid(unsafe_code)]

use borsh::{BorshDeserialize, BorshSerialize};
use sha2::{Digest, Sha256};

/// Semantic schema version used by [`JournalV1`].
pub const JOURNAL_V1_SCHEMA_VERSION: u32 = 1;

/// Exact size of the local candidate `JournalV1` Borsh encoding.
pub const JOURNAL_V1_CANDIDATE_WIRE_SIZE: usize = 165;

/// Schema version of the restricted development artifact.
pub const RESTRICTED_ARTIFACT_V1_SCHEMA_VERSION: u32 = 1;

/// Schema version of the fixed restricted specification.
pub const RESTRICTED_SPEC_V1_SCHEMA_VERSION: u32 = 1;

/// Exact size of the restricted development artifact Borsh encoding.
pub const RESTRICTED_ARTIFACT_V1_WIRE_SIZE: usize = 12;

/// Version of the pure deterministic harness implemented by this crate.
pub const DETERMINISTIC_HARNESS_VERSION: u32 = 1;

const SPEC_HASH_DOMAIN: &[u8] = b"vericode:spec:v1\0";
const HARNESS_HASH_DOMAIN: &[u8] = b"vericode:harness:v1\0";
const ARTIFACT_HASH_DOMAIN: &[u8] = b"vericode:artifact:v1\0";

/// A fixed-size 32-byte value used for public cryptographic commitments.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Hash32([u8; 32]);

impl Hash32 {
    /// Creates a commitment from its exact 32-byte value.
    #[must_use]
    pub const fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Returns the exact commitment bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Consumes the commitment and returns its exact bytes.
    #[must_use]
    pub const fn into_bytes(self) -> [u8; 32] {
        self.0
    }
}

impl From<[u8; 32]> for Hash32 {
    fn from(bytes: [u8; 32]) -> Self {
        Self::new(bytes)
    }
}

/// Canonical semantic identifier of a VeriCode job.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct JobId(Hash32);

impl JobId {
    /// Creates a job identifier from its exact 32-byte value.
    #[must_use]
    pub const fn new(bytes: [u8; 32]) -> Self {
        Self(Hash32::new(bytes))
    }

    /// Creates a job identifier from an existing fixed-size value.
    #[must_use]
    pub const fn from_hash(hash: Hash32) -> Self {
        Self(hash)
    }

    /// Returns the underlying fixed-size value.
    #[must_use]
    pub const fn as_hash(&self) -> &Hash32 {
        &self.0
    }
}

/// Semantic identifier of the expected guest image.
///
/// This is not a Solana program ID.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct ImageId(Hash32);

impl ImageId {
    /// Creates an image identifier from its exact 32-byte value.
    #[must_use]
    pub const fn new(bytes: [u8; 32]) -> Self {
        Self(Hash32::new(bytes))
    }

    /// Creates an image identifier from an existing fixed-size value.
    #[must_use]
    pub const fn from_hash(hash: Hash32) -> Self {
        Self(hash)
    }

    /// Returns the underlying fixed-size value.
    #[must_use]
    pub const fn as_hash(&self) -> &Hash32 {
        &self.0
    }
}

/// Normal semantic outcome of evaluating the restricted artifact.
///
/// In the candidate Borsh format, declaration order fixes `Pass` to tag `0`
/// and `Fail` to tag `1`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, BorshDeserialize, BorshSerialize)]
pub enum Verdict {
    /// The artifact satisfied the fixed specification and harness.
    Pass,
    /// The artifact did not satisfy the fixed specification and harness.
    Fail,
}

/// Fixed specification for the single restricted development artifact.
#[derive(Clone, Copy, Debug, Eq, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct RestrictedSpecV1 {
    schema_version: u32,
    multiplier: u32,
    max_input: u32,
}

impl RestrictedSpecV1 {
    /// Creates a specification value for commitment calculation.
    ///
    /// The production harness always evaluates against [`RESTRICTED_SPEC_V1`].
    #[must_use]
    pub const fn new(multiplier: u32, max_input: u32) -> Self {
        Self {
            schema_version: RESTRICTED_SPEC_V1_SCHEMA_VERSION,
            multiplier,
            max_input,
        }
    }

    /// Returns the restricted specification schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    /// Returns the required multiplication factor.
    #[must_use]
    pub const fn multiplier(&self) -> u32 {
        self.multiplier
    }

    /// Returns the inclusive maximum accepted input.
    #[must_use]
    pub const fn max_input(&self) -> u32 {
        self.max_input
    }
}

/// The only specification evaluated by the D1c2a development harness.
///
/// A valid artifact claims that `claimed_output == input * 2`, with input no
/// greater than one million.
pub const RESTRICTED_SPEC_V1: RestrictedSpecV1 = RestrictedSpecV1::new(2, 1_000_000);

/// One restricted, versioned development artifact.
///
/// This is a value record, not source code, a file, a repository, or a patch.
#[derive(Clone, Copy, Debug, Eq, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct RestrictedArtifactV1 {
    schema_version: u32,
    input: u32,
    claimed_output: u32,
}

impl RestrictedArtifactV1 {
    /// Creates a canonical V1 development artifact.
    #[must_use]
    pub const fn new(input: u32, claimed_output: u32) -> Self {
        Self {
            schema_version: RESTRICTED_ARTIFACT_V1_SCHEMA_VERSION,
            input,
            claimed_output,
        }
    }

    /// Returns the artifact schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    /// Returns the input value.
    #[must_use]
    pub const fn input(&self) -> u32 {
        self.input
    }

    /// Returns the output claimed by the artifact.
    #[must_use]
    pub const fn claimed_output(&self) -> u32 {
        self.claimed_output
    }

    /// Encodes the artifact with the local candidate Borsh format.
    pub fn encode_candidate(&self) -> Result<Vec<u8>, ArtifactWireError> {
        let bytes = self
            .try_to_vec()
            .map_err(|_| ArtifactWireError::CodecFailure)?;

        if bytes.len() != RESTRICTED_ARTIFACT_V1_WIRE_SIZE {
            return Err(ArtifactWireError::UnexpectedLength {
                expected: RESTRICTED_ARTIFACT_V1_WIRE_SIZE,
                actual: bytes.len(),
            });
        }

        Ok(bytes)
    }

    /// Decodes an exact, canonical V1 development artifact.
    pub fn decode_candidate(bytes: &[u8]) -> Result<Self, ArtifactWireError> {
        if bytes.len() < RESTRICTED_ARTIFACT_V1_WIRE_SIZE {
            return Err(ArtifactWireError::Truncated {
                expected: RESTRICTED_ARTIFACT_V1_WIRE_SIZE,
                actual: bytes.len(),
            });
        }
        if bytes.len() > RESTRICTED_ARTIFACT_V1_WIRE_SIZE {
            return Err(ArtifactWireError::UnexpectedLength {
                expected: RESTRICTED_ARTIFACT_V1_WIRE_SIZE,
                actual: bytes.len(),
            });
        }

        let schema_version =
            u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        if schema_version != RESTRICTED_ARTIFACT_V1_SCHEMA_VERSION {
            return Err(ArtifactWireError::UnsupportedSchemaVersion {
                expected: RESTRICTED_ARTIFACT_V1_SCHEMA_VERSION,
                actual: schema_version,
            });
        }

        Self::try_from_slice(bytes).map_err(|_| ArtifactWireError::CodecFailure)
    }
}

/// Failure to encode or decode the restricted artifact candidate format.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArtifactWireError {
    /// The payload ended before the fixed-size record was complete.
    Truncated { expected: usize, actual: usize },
    /// The payload contained bytes outside the fixed-size record.
    UnexpectedLength { expected: usize, actual: usize },
    /// The payload names an artifact schema this crate does not evaluate.
    UnsupportedSchemaVersion { expected: u32, actual: u32 },
    /// Borsh could not encode or decode the fixed record.
    CodecFailure,
}

/// Failure to encode a value before calculating its canonical commitment.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CommitmentError {
    /// Borsh could not encode the fixed value.
    CodecFailure,
}

/// Calculates the canonical SHA-256 commitment to a specification value.
pub fn hash_restricted_spec(spec: &RestrictedSpecV1) -> Result<Hash32, CommitmentError> {
    hash_borsh_value(SPEC_HASH_DOMAIN, spec)
}

/// Calculates the canonical SHA-256 commitment to a harness version.
pub fn hash_harness_version(version: u32) -> Result<Hash32, CommitmentError> {
    hash_borsh_value(HARNESS_HASH_DOMAIN, &version)
}

/// Calculates the canonical SHA-256 commitment to a restricted artifact.
pub fn hash_restricted_artifact(
    artifact: &RestrictedArtifactV1,
) -> Result<Hash32, CommitmentError> {
    hash_borsh_value(ARTIFACT_HASH_DOMAIN, artifact)
}

fn hash_borsh_value<T: BorshSerialize>(
    domain: &[u8],
    value: &T,
) -> Result<Hash32, CommitmentError> {
    let payload = value
        .try_to_vec()
        .map_err(|_| CommitmentError::CodecFailure)?;
    let mut hasher = Sha256::new();
    hasher.update(domain);
    hasher.update(payload);
    Ok(Hash32::new(hasher.finalize().into()))
}

/// Public commitments authorized for a single `JournalV1` evaluation.
///
/// Executor and mint are intentionally absent: they belong to future Job
/// state and on-chain authorization, not to this journal contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JournalV1Commitments {
    schema_version: u32,
    job_id: JobId,
    spec_hash: Hash32,
    harness_hash: Hash32,
    artifact_hash: Hash32,
    image_id: ImageId,
}

impl JournalV1Commitments {
    /// Creates the expected commitments for the semantic V1 contract.
    #[must_use]
    pub const fn new(
        job_id: JobId,
        spec_hash: Hash32,
        harness_hash: Hash32,
        artifact_hash: Hash32,
        image_id: ImageId,
    ) -> Self {
        Self {
            schema_version: JOURNAL_V1_SCHEMA_VERSION,
            job_id,
            spec_hash,
            harness_hash,
            artifact_hash,
            image_id,
        }
    }

    /// Returns the expected semantic schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    /// Returns the expected job identifier.
    #[must_use]
    pub const fn job_id(&self) -> JobId {
        self.job_id
    }

    /// Returns the expected specification commitment.
    #[must_use]
    pub const fn spec_hash(&self) -> Hash32 {
        self.spec_hash
    }

    /// Returns the expected harness commitment.
    #[must_use]
    pub const fn harness_hash(&self) -> Hash32 {
        self.harness_hash
    }

    /// Returns the expected restricted-artifact commitment.
    #[must_use]
    pub const fn artifact_hash(&self) -> Hash32 {
        self.artifact_hash
    }

    /// Returns the expected guest image identifier.
    #[must_use]
    pub const fn image_id(&self) -> ImageId {
        self.image_id
    }
}

/// Semantic fields publicly committed by a VeriCode V1 evaluation.
///
/// The Borsh representation implemented here is a local candidate, not a
/// frozen Anchor or Router ABI.
#[derive(Clone, Copy, Debug, Eq, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct JournalV1 {
    schema_version: u32,
    job_id: JobId,
    spec_hash: Hash32,
    harness_hash: Hash32,
    artifact_hash: Hash32,
    image_id: ImageId,
    verdict: Verdict,
}

impl JournalV1 {
    /// Constructs a semantic V1 journal deterministically from exact values.
    #[must_use]
    pub const fn new(
        job_id: JobId,
        spec_hash: Hash32,
        harness_hash: Hash32,
        artifact_hash: Hash32,
        image_id: ImageId,
        verdict: Verdict,
    ) -> Self {
        Self {
            schema_version: JOURNAL_V1_SCHEMA_VERSION,
            job_id,
            spec_hash,
            harness_hash,
            artifact_hash,
            image_id,
            verdict,
        }
    }

    /// Encodes this journal with the local candidate Borsh format.
    pub fn encode_candidate(&self) -> Result<Vec<u8>, JournalWireError> {
        if self.schema_version != JOURNAL_V1_SCHEMA_VERSION {
            return Err(JournalWireError::UnsupportedSchemaVersion {
                expected: JOURNAL_V1_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }

        let bytes = self
            .try_to_vec()
            .map_err(|_| JournalWireError::CodecFailure)?;
        if bytes.len() != JOURNAL_V1_CANDIDATE_WIRE_SIZE {
            return Err(JournalWireError::UnexpectedLength {
                expected: JOURNAL_V1_CANDIDATE_WIRE_SIZE,
                actual: bytes.len(),
            });
        }

        Ok(bytes)
    }

    /// Decodes an exact journal from the local candidate Borsh format.
    pub fn decode_candidate(bytes: &[u8]) -> Result<Self, JournalWireError> {
        if bytes.len() < JOURNAL_V1_CANDIDATE_WIRE_SIZE {
            return Err(JournalWireError::Truncated {
                expected: JOURNAL_V1_CANDIDATE_WIRE_SIZE,
                actual: bytes.len(),
            });
        }
        if bytes.len() > JOURNAL_V1_CANDIDATE_WIRE_SIZE {
            return Err(JournalWireError::UnexpectedLength {
                expected: JOURNAL_V1_CANDIDATE_WIRE_SIZE,
                actual: bytes.len(),
            });
        }

        let schema_version =
            u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        if schema_version != JOURNAL_V1_SCHEMA_VERSION {
            return Err(JournalWireError::UnsupportedSchemaVersion {
                expected: JOURNAL_V1_SCHEMA_VERSION,
                actual: schema_version,
            });
        }

        let verdict_tag = bytes[JOURNAL_V1_CANDIDATE_WIRE_SIZE - 1];
        if verdict_tag > 1 {
            return Err(JournalWireError::UnknownVerdictTag(verdict_tag));
        }

        Self::try_from_slice(bytes).map_err(|_| JournalWireError::CodecFailure)
    }

    /// Validates every V1 commitment and returns either normal verdict.
    ///
    /// `Ok(Verdict::Fail)` means that the commitments match and the evaluated
    /// artifact failed. It is not an error and does not authorize release.
    pub fn validate_against(
        &self,
        expected: &JournalV1Commitments,
    ) -> Result<Verdict, JournalValidationError> {
        if self.schema_version != expected.schema_version {
            return Err(JournalValidationError::SchemaVersionMismatch);
        }
        if self.job_id != expected.job_id {
            return Err(JournalValidationError::JobIdMismatch);
        }
        if self.spec_hash != expected.spec_hash {
            return Err(JournalValidationError::SpecHashMismatch);
        }
        if self.harness_hash != expected.harness_hash {
            return Err(JournalValidationError::HarnessHashMismatch);
        }
        if self.artifact_hash != expected.artifact_hash {
            return Err(JournalValidationError::ArtifactHashMismatch);
        }
        if self.image_id != expected.image_id {
            return Err(JournalValidationError::ImageIdMismatch);
        }

        Ok(self.verdict)
    }

    /// Returns the semantic schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    /// Returns the job identifier.
    #[must_use]
    pub const fn job_id(&self) -> JobId {
        self.job_id
    }

    /// Returns the specification commitment.
    #[must_use]
    pub const fn spec_hash(&self) -> Hash32 {
        self.spec_hash
    }

    /// Returns the harness commitment.
    #[must_use]
    pub const fn harness_hash(&self) -> Hash32 {
        self.harness_hash
    }

    /// Returns the restricted-artifact commitment.
    #[must_use]
    pub const fn artifact_hash(&self) -> Hash32 {
        self.artifact_hash
    }

    /// Returns the guest image identifier.
    #[must_use]
    pub const fn image_id(&self) -> ImageId {
        self.image_id
    }

    /// Returns the normal semantic outcome.
    #[must_use]
    pub const fn verdict(&self) -> Verdict {
        self.verdict
    }
}

/// Failure to encode or decode the local `JournalV1` candidate format.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JournalWireError {
    /// The payload ended before the fixed-size journal was complete.
    Truncated { expected: usize, actual: usize },
    /// The payload contained bytes outside the fixed-size journal.
    UnexpectedLength { expected: usize, actual: usize },
    /// The payload names a journal schema this crate does not understand.
    UnsupportedSchemaVersion { expected: u32, actual: u32 },
    /// The final Borsh enum ordinal is neither `Pass` nor `Fail`.
    UnknownVerdictTag(u8),
    /// Borsh could not encode or decode the fixed journal.
    CodecFailure,
}

/// Reason why a journal does not match the commitments authorized for a job.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JournalValidationError {
    /// The journal claims a different semantic schema version.
    SchemaVersionMismatch,
    /// The journal belongs to a different job.
    JobIdMismatch,
    /// The journal commits to a different specification.
    SpecHashMismatch,
    /// The journal commits to a different harness.
    HarnessHashMismatch,
    /// The journal commits to a different restricted artifact.
    ArtifactHashMismatch,
    /// The journal identifies a different guest image.
    ImageIdMismatch,
}

/// Explicit failures that prevent the restricted harness from producing a verdict.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HarnessError {
    /// The artifact bytes are not the exact supported candidate format.
    ArtifactWire(ArtifactWireError),
    /// The input exceeds the fixed specification limit.
    InputOutOfRange { maximum: u32, actual: u32 },
    /// The fixed arithmetic could not be represented as `u32`.
    ArithmeticOverflow,
    /// A commitment input could not be encoded canonically.
    Commitment(CommitmentError),
}

/// Evaluates exactly one restricted artifact with the fixed pure harness.
///
/// Invalid input is returned as [`HarnessError`]. A valid but incorrect claim
/// produces a normal [`Verdict::Fail`] journal. This function performs no I/O
/// and grants no payment or release authority.
pub fn evaluate_restricted_artifact(
    job_id: JobId,
    image_id: ImageId,
    artifact_bytes: &[u8],
) -> Result<JournalV1, HarnessError> {
    let artifact = RestrictedArtifactV1::decode_candidate(artifact_bytes)
        .map_err(HarnessError::ArtifactWire)?;
    let spec = RESTRICTED_SPEC_V1;

    if artifact.input > spec.max_input {
        return Err(HarnessError::InputOutOfRange {
            maximum: spec.max_input,
            actual: artifact.input,
        });
    }

    let expected_output = match artifact.input.checked_mul(spec.multiplier) {
        Some(value) => value,
        None => return Err(HarnessError::ArithmeticOverflow),
    };
    let verdict = if artifact.claimed_output == expected_output {
        Verdict::Pass
    } else {
        Verdict::Fail
    };

    let spec_hash = hash_restricted_spec(&spec).map_err(HarnessError::Commitment)?;
    let harness_hash = hash_harness_version(DETERMINISTIC_HARNESS_VERSION)
        .map_err(HarnessError::Commitment)?;
    let artifact_hash =
        hash_restricted_artifact(&artifact).map_err(HarnessError::Commitment)?;

    Ok(JournalV1::new(
        job_id,
        spec_hash,
        harness_hash,
        artifact_hash,
        image_id,
        verdict,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::fmt::Debug;

    const JOB_ID: JobId = JobId::new([0x11; 32]);
    // Development placeholder only; D1c2a does not create a guest or ImageID.
    const IMAGE_ID: ImageId = ImageId::new([0x55; 32]);

    const SPEC_HASH_HEX: &str =
        "af642b561ac73a0f78ae767123f95a0c6ece6ce89f2a5e5e20cb86bec49fb778";
    const HARNESS_HASH_HEX: &str =
        "01124025c6ad84bb8490f216e95ff241d862dc2faf316d0e28bcdb55b0996b50";
    const PASS_ARTIFACT_HASH_HEX: &str =
        "d5aa9223d6d2a1ba23bd73ca325b411c75027a739c285a95ff63b963442a224c";
    const FAIL_ARTIFACT_HASH_HEX: &str =
        "343ad7781fe806e047dd7f965e2a41713ce0f668f4d88ebf351d21ef88ee55c6";

    const PASS_JOURNAL_HEX: &str = concat!(
        "01000000",
        "1111111111111111111111111111111111111111111111111111111111111111",
        "af642b561ac73a0f78ae767123f95a0c6ece6ce89f2a5e5e20cb86bec49fb778",
        "01124025c6ad84bb8490f216e95ff241d862dc2faf316d0e28bcdb55b0996b50",
        "d5aa9223d6d2a1ba23bd73ca325b411c75027a739c285a95ff63b963442a224c",
        "5555555555555555555555555555555555555555555555555555555555555555",
        "00",
    );
    const FAIL_JOURNAL_HEX: &str = concat!(
        "01000000",
        "1111111111111111111111111111111111111111111111111111111111111111",
        "af642b561ac73a0f78ae767123f95a0c6ece6ce89f2a5e5e20cb86bec49fb778",
        "01124025c6ad84bb8490f216e95ff241d862dc2faf316d0e28bcdb55b0996b50",
        "343ad7781fe806e047dd7f965e2a41713ce0f668f4d88ebf351d21ef88ee55c6",
        "5555555555555555555555555555555555555555555555555555555555555555",
        "01",
    );

    fn require_ok<T, E: Debug>(result: Result<T, E>) -> T {
        match result {
            Ok(value) => value,
            Err(error) => panic!("expected Ok, got {error:?}"),
        }
    }

    fn literal_hex<const N: usize>(hex: &str) -> [u8; N] {
        assert_eq!(hex.len(), N * 2, "literal hex has the wrong length");
        let mut output = [0_u8; N];
        let bytes = hex.as_bytes();
        let mut index = 0;
        while index < N {
            output[index] = (hex_nibble(bytes[index * 2]) << 4)
                | hex_nibble(bytes[index * 2 + 1]);
            index += 1;
        }
        output
    }

    fn hex_nibble(value: u8) -> u8 {
        match value {
            b'0'..=b'9' => value - b'0',
            b'a'..=b'f' => value - b'a' + 10,
            _ => panic!("invalid literal hex digit"),
        }
    }

    fn artifact_bytes(claimed_output: u32) -> Vec<u8> {
        require_ok(RestrictedArtifactV1::new(7, claimed_output).encode_candidate())
    }

    fn pass_journal() -> JournalV1 {
        require_ok(evaluate_restricted_artifact(
            JOB_ID,
            IMAGE_ID,
            &artifact_bytes(14),
        ))
    }

    fn expected_for(journal: &JournalV1) -> JournalV1Commitments {
        JournalV1Commitments::new(
            journal.job_id(),
            journal.spec_hash(),
            journal.harness_hash(),
            journal.artifact_hash(),
            journal.image_id(),
        )
    }

    #[test]
    fn restricted_artifact_has_literal_candidate_bytes() {
        assert_eq!(
            artifact_bytes(14),
            [
                0x01, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x0e, 0x00, 0x00, 0x00,
            ]
        );
    }

    #[test]
    fn construction_is_deterministic_for_the_restricted_artifact() {
        let bytes = artifact_bytes(14);
        let first = require_ok(evaluate_restricted_artifact(JOB_ID, IMAGE_ID, &bytes));
        let second = require_ok(evaluate_restricted_artifact(JOB_ID, IMAGE_ID, &bytes));

        assert_eq!(first, second);
        assert_eq!(first.schema_version(), JOURNAL_V1_SCHEMA_VERSION);
    }

    #[test]
    fn pass_is_a_normal_verified_value() {
        let journal = pass_journal();
        assert_eq!(journal.verdict(), Verdict::Pass);
        assert_eq!(
            journal.validate_against(&expected_for(&journal)),
            Ok(Verdict::Pass)
        );
    }

    #[test]
    fn fail_is_a_normal_verified_value() {
        let journal = require_ok(evaluate_restricted_artifact(
            JOB_ID,
            IMAGE_ID,
            &artifact_bytes(15),
        ));
        assert_eq!(journal.verdict(), Verdict::Fail);
        assert_eq!(
            journal.validate_against(&expected_for(&journal)),
            Ok(Verdict::Fail)
        );
    }

    #[test]
    fn invalid_input_is_an_explicit_error_not_a_fail_verdict() {
        let artifact = RestrictedArtifactV1::new(RESTRICTED_SPEC_V1.max_input() + 1, 0);
        let bytes = require_ok(artifact.encode_candidate());

        assert_eq!(
            evaluate_restricted_artifact(JOB_ID, IMAGE_ID, &bytes),
            Err(HarnessError::InputOutOfRange {
                maximum: RESTRICTED_SPEC_V1.max_input(),
                actual: RESTRICTED_SPEC_V1.max_input() + 1,
            })
        );
    }

    #[test]
    fn canonical_hashes_match_independent_literal_vectors() {
        assert_eq!(
            require_ok(hash_restricted_spec(&RESTRICTED_SPEC_V1)),
            Hash32::new(literal_hex::<32>(SPEC_HASH_HEX))
        );
        assert_eq!(
            require_ok(hash_harness_version(DETERMINISTIC_HARNESS_VERSION)),
            Hash32::new(literal_hex::<32>(HARNESS_HASH_HEX))
        );
        assert_eq!(
            require_ok(hash_restricted_artifact(&RestrictedArtifactV1::new(7, 14))),
            Hash32::new(literal_hex::<32>(PASS_ARTIFACT_HASH_HEX))
        );
    }

    #[test]
    fn changing_one_artifact_byte_changes_artifact_hash() {
        let pass_hash = require_ok(hash_restricted_artifact(&RestrictedArtifactV1::new(7, 14)));
        let changed_hash = require_ok(hash_restricted_artifact(&RestrictedArtifactV1::new(7, 15)));

        assert_ne!(pass_hash, changed_hash);
        assert_eq!(
            changed_hash,
            Hash32::new(literal_hex::<32>(FAIL_ARTIFACT_HASH_HEX))
        );
    }

    #[test]
    fn changing_the_specification_changes_spec_hash() {
        let changed = RestrictedSpecV1::new(3, 1_000_000);
        let changed_hash = require_ok(hash_restricted_spec(&changed));

        assert_ne!(
            require_ok(hash_restricted_spec(&RESTRICTED_SPEC_V1)),
            changed_hash
        );
        assert_eq!(
            changed_hash,
            Hash32::new(literal_hex::<32>(
                "3ae3ca9d296cdb907b5318fe2c4df9e8893be08bd21fdb5c3db6d04c1f50f3d6"
            ))
        );
    }

    #[test]
    fn changing_the_harness_version_changes_harness_hash() {
        let changed_hash = require_ok(hash_harness_version(2));

        assert_ne!(
            require_ok(hash_harness_version(DETERMINISTIC_HARNESS_VERSION)),
            changed_hash
        );
        assert_eq!(
            changed_hash,
            Hash32::new(literal_hex::<32>(
                "890480c6ec39ff8f3704114e30b317a20980fab27a4069d8178a713b26f7791e"
            ))
        );
    }

    #[test]
    fn journal_candidate_round_trip_preserves_every_field() {
        let original = pass_journal();
        let encoded = require_ok(original.encode_candidate());
        let decoded = require_ok(JournalV1::decode_candidate(&encoded));

        assert_eq!(decoded, original);
    }

    #[test]
    fn pass_journal_matches_the_literal_wire_vector() {
        assert_eq!(
            require_ok(pass_journal().encode_candidate()),
            literal_hex::<JOURNAL_V1_CANDIDATE_WIRE_SIZE>(PASS_JOURNAL_HEX)
        );
    }

    #[test]
    fn fail_journal_matches_the_literal_wire_vector() {
        let journal = require_ok(evaluate_restricted_artifact(
            JOB_ID,
            IMAGE_ID,
            &artifact_bytes(15),
        ));

        assert_eq!(
            require_ok(journal.encode_candidate()),
            literal_hex::<JOURNAL_V1_CANDIDATE_WIRE_SIZE>(FAIL_JOURNAL_HEX)
        );
    }

    #[test]
    fn truncated_journal_payload_is_rejected() {
        let mut bytes = literal_hex::<JOURNAL_V1_CANDIDATE_WIRE_SIZE>(PASS_JOURNAL_HEX).to_vec();
        let removed = bytes.pop();
        assert_eq!(removed, Some(0));

        assert_eq!(
            JournalV1::decode_candidate(&bytes),
            Err(JournalWireError::Truncated {
                expected: JOURNAL_V1_CANDIDATE_WIRE_SIZE,
                actual: JOURNAL_V1_CANDIDATE_WIRE_SIZE - 1,
            })
        );
    }

    #[test]
    fn unknown_verdict_tag_is_rejected() {
        let mut bytes = literal_hex::<JOURNAL_V1_CANDIDATE_WIRE_SIZE>(PASS_JOURNAL_HEX);
        bytes[JOURNAL_V1_CANDIDATE_WIRE_SIZE - 1] = 2;

        assert_eq!(
            JournalV1::decode_candidate(&bytes),
            Err(JournalWireError::UnknownVerdictTag(2))
        );
    }

    #[test]
    fn incompatible_schema_version_is_rejected() {
        let mut bytes = literal_hex::<JOURNAL_V1_CANDIDATE_WIRE_SIZE>(PASS_JOURNAL_HEX);
        bytes[0..4].copy_from_slice(&2_u32.to_le_bytes());

        assert_eq!(
            JournalV1::decode_candidate(&bytes),
            Err(JournalWireError::UnsupportedSchemaVersion {
                expected: JOURNAL_V1_SCHEMA_VERSION,
                actual: 2,
            })
        );
    }

    #[test]
    fn decoded_journal_with_a_divergent_commitment_is_rejected() {
        let bytes = literal_hex::<JOURNAL_V1_CANDIDATE_WIRE_SIZE>(PASS_JOURNAL_HEX);
        let journal = require_ok(JournalV1::decode_candidate(&bytes));
        let expected = JournalV1Commitments::new(
            JOB_ID,
            journal.spec_hash(),
            journal.harness_hash(),
            Hash32::new([0x99; 32]),
            IMAGE_ID,
        );

        assert_eq!(
            journal.validate_against(&expected),
            Err(JournalValidationError::ArtifactHashMismatch)
        );
    }

    #[test]
    fn rejects_a_different_job_id() {
        let journal = pass_journal();
        let expected = JournalV1Commitments::new(
            JobId::new([0x99; 32]),
            journal.spec_hash(),
            journal.harness_hash(),
            journal.artifact_hash(),
            IMAGE_ID,
        );

        assert_eq!(
            journal.validate_against(&expected),
            Err(JournalValidationError::JobIdMismatch)
        );
    }

    #[test]
    fn rejects_a_different_spec_hash() {
        let journal = pass_journal();
        let expected = JournalV1Commitments::new(
            JOB_ID,
            Hash32::new([0x99; 32]),
            journal.harness_hash(),
            journal.artifact_hash(),
            IMAGE_ID,
        );

        assert_eq!(
            journal.validate_against(&expected),
            Err(JournalValidationError::SpecHashMismatch)
        );
    }

    #[test]
    fn rejects_a_different_harness_hash() {
        let journal = pass_journal();
        let expected = JournalV1Commitments::new(
            JOB_ID,
            journal.spec_hash(),
            Hash32::new([0x99; 32]),
            journal.artifact_hash(),
            IMAGE_ID,
        );

        assert_eq!(
            journal.validate_against(&expected),
            Err(JournalValidationError::HarnessHashMismatch)
        );
    }

    #[test]
    fn rejects_a_different_image_id() {
        let journal = pass_journal();
        let expected = JournalV1Commitments::new(
            JOB_ID,
            journal.spec_hash(),
            journal.harness_hash(),
            journal.artifact_hash(),
            ImageId::new([0x99; 32]),
        );

        assert_eq!(
            journal.validate_against(&expected),
            Err(JournalValidationError::ImageIdMismatch)
        );
    }
}
