//! Pure semantic types for the restricted VeriCode MVP.
//!
//! This crate deliberately defines no public journal wire format. It models
//! the meaning of `JournalV1` and validates its public commitments only.

#![forbid(unsafe_code)]

/// Semantic schema version used by [`JournalV1`].
pub const JOURNAL_V1_SCHEMA_VERSION: u32 = 1;

/// A fixed-size 32-byte value used for public cryptographic commitments.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Hash32([u8; 32]);

impl Hash32 {
    /// Creates a commitment from its exact 32-byte value.
    #[must_use]
    pub const fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Returns the exact bytes without assigning a serialization format.
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
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Verdict {
    /// The artifact satisfied the fixed specification and harness.
    Pass,
    /// The artifact did not satisfy the fixed specification and harness.
    Fail,
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
/// This type does not define byte layout, endianness, encoding, hashing, or
/// receipt format. Those remain a separate Draft v0 gate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    const JOB_ID: JobId = JobId::new([0x11; 32]);
    const SPEC_HASH: Hash32 = Hash32::new([0x22; 32]);
    const HARNESS_HASH: Hash32 = Hash32::new([0x33; 32]);
    const IMAGE_ID: ImageId = ImageId::new([0x55; 32]);

    // Development-only fixture for one restricted, versioned value record.
    // It is data, not source code and not a claim of arbitrary-code support.
    const RESTRICTED_DEVELOPMENT_FIXTURE: &[u8] =
        b"vericode:restricted-artifact:v1:input=7;expected=14";
    const FIXTURE_ARTIFACT_HASH: Hash32 = Hash32::new([
        0xf6, 0x2b, 0xbe, 0xa1, 0x30, 0x8c, 0x1c, 0x0d, 0x38, 0xba, 0xa8, 0xcc, 0x62, 0x63,
        0x01, 0x63, 0x76, 0x74, 0x5a, 0xf4, 0x5e, 0x93, 0x1a, 0xab, 0x86, 0xdd, 0x1c, 0x35,
        0xa7, 0x84, 0xfb, 0x26,
    ]);

    fn expected() -> JournalV1Commitments {
        JournalV1Commitments::new(
            JOB_ID,
            SPEC_HASH,
            HARNESS_HASH,
            FIXTURE_ARTIFACT_HASH,
            IMAGE_ID,
        )
    }

    fn journal(verdict: Verdict) -> JournalV1 {
        JournalV1::new(
            JOB_ID,
            SPEC_HASH,
            HARNESS_HASH,
            FIXTURE_ARTIFACT_HASH,
            IMAGE_ID,
            verdict,
        )
    }

    #[test]
    fn construction_is_deterministic_for_the_restricted_fixture() {
        let first = journal(Verdict::Pass);
        let second = journal(Verdict::Pass);

        assert_eq!(RESTRICTED_DEVELOPMENT_FIXTURE.len(), 51);
        assert_eq!(first, second);
        assert_eq!(first.schema_version(), JOURNAL_V1_SCHEMA_VERSION);
        assert_eq!(first.artifact_hash(), FIXTURE_ARTIFACT_HASH);
    }

    #[test]
    fn pass_is_a_normal_verified_value() {
        assert_eq!(
            journal(Verdict::Pass).validate_against(&expected()),
            Ok(Verdict::Pass)
        );
    }

    #[test]
    fn fail_is_a_normal_verified_value() {
        assert_eq!(
            journal(Verdict::Fail).validate_against(&expected()),
            Ok(Verdict::Fail)
        );
    }

    #[test]
    fn rejects_a_different_job_id() {
        let actual = JournalV1::new(
            JobId::new([0x99; 32]),
            SPEC_HASH,
            HARNESS_HASH,
            FIXTURE_ARTIFACT_HASH,
            IMAGE_ID,
            Verdict::Pass,
        );

        assert_eq!(
            actual.validate_against(&expected()),
            Err(JournalValidationError::JobIdMismatch)
        );
    }

    #[test]
    fn rejects_a_different_spec_hash() {
        let actual = JournalV1::new(
            JOB_ID,
            Hash32::new([0x99; 32]),
            HARNESS_HASH,
            FIXTURE_ARTIFACT_HASH,
            IMAGE_ID,
            Verdict::Pass,
        );

        assert_eq!(
            actual.validate_against(&expected()),
            Err(JournalValidationError::SpecHashMismatch)
        );
    }

    #[test]
    fn rejects_a_different_harness_hash() {
        let actual = JournalV1::new(
            JOB_ID,
            SPEC_HASH,
            Hash32::new([0x99; 32]),
            FIXTURE_ARTIFACT_HASH,
            IMAGE_ID,
            Verdict::Pass,
        );

        assert_eq!(
            actual.validate_against(&expected()),
            Err(JournalValidationError::HarnessHashMismatch)
        );
    }

    #[test]
    fn rejects_a_different_artifact_hash() {
        let actual = JournalV1::new(
            JOB_ID,
            SPEC_HASH,
            HARNESS_HASH,
            Hash32::new([0x99; 32]),
            IMAGE_ID,
            Verdict::Pass,
        );

        assert_eq!(
            actual.validate_against(&expected()),
            Err(JournalValidationError::ArtifactHashMismatch)
        );
    }

    #[test]
    fn rejects_a_different_image_id() {
        let actual = JournalV1::new(
            JOB_ID,
            SPEC_HASH,
            HARNESS_HASH,
            FIXTURE_ARTIFACT_HASH,
            ImageId::new([0x99; 32]),
            Verdict::Pass,
        );

        assert_eq!(
            actual.validate_against(&expected()),
            Err(JournalValidationError::ImageIdMismatch)
        );
    }

    #[test]
    fn rejects_a_different_schema_version() {
        let mut actual = journal(Verdict::Pass);
        actual.schema_version = JOURNAL_V1_SCHEMA_VERSION + 1;

        assert_eq!(
            actual.validate_against(&expected()),
            Err(JournalValidationError::SchemaVersionMismatch)
        );
    }
}
