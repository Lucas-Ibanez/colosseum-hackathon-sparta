//! Pure escrow policy for an immutable [`JobV1`].
//!
//! This module only decides whether a requested transition is allowed by the
//! Job terms. It holds no funds, moves no tokens, reads no clock and has no
//! administrative override. It does not verify receipts, seals, Groth16
//! proofs, Router deployments or CPI: a future adapter must have verified the
//! receipt before passing its journal here. Refund, deadline and timeout
//! policies are pending decisions and are deliberately not implemented.

use crate::{Hash32, ImageId, JobId, JournalV1, JournalV1Commitments, JournalValidationError, Verdict};

/// 32-byte identity of the buyer that creates and funds a Job.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BuyerId([u8; 32]);

impl BuyerId {
    /// Creates a buyer identity from its exact 32-byte value.
    #[must_use]
    pub const fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Returns the exact identity bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

/// 32-byte identity of the only executor a Job may pay.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ExecutorId([u8; 32]);

impl ExecutorId {
    /// Creates an executor identity from its exact 32-byte value.
    #[must_use]
    pub const fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Returns the exact identity bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

/// 32-byte identity of the only token mint a Job accepts and pays.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MintId([u8; 32]);

impl MintId {
    /// Creates a mint identity from its exact 32-byte value.
    #[must_use]
    pub const fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Returns the exact identity bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

/// Non-zero escrow amount in the base units of the Job mint.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Amount(u64);

impl Amount {
    /// Creates a validated amount; zero is rejected.
    pub const fn new(base_units: u64) -> Result<Self, AmountError> {
        if base_units == 0 {
            return Err(AmountError::Zero);
        }
        Ok(Self(base_units))
    }

    /// Returns the amount in base units of the mint.
    #[must_use]
    pub const fn base_units(&self) -> u64 {
        self.0
    }
}

/// Reason why a value cannot be used as an escrow amount.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AmountError {
    /// An escrow of zero base units is not a valid Job.
    Zero,
}

/// Identity field of a Job, used to report construction errors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JobParty {
    /// The buyer identity.
    Buyer,
    /// The executor identity.
    Executor,
    /// The mint identity.
    Mint,
}

/// Reason why Job terms are rejected at construction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JobError {
    /// The identity is all zero bytes and cannot name a real party or mint.
    ZeroIdentity(JobParty),
    /// The buyer and executor are the same identity.
    BuyerIsExecutor,
}

/// Immutable terms of one escrowed VeriCode Job.
///
/// Fields are fixed at construction and only readable afterwards. There is no
/// deadline field: the deadline, timeout and refund policy are pending.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JobV1 {
    job_id: JobId,
    buyer: BuyerId,
    executor: ExecutorId,
    mint: MintId,
    amount: Amount,
    spec_hash: Hash32,
    harness_hash: Hash32,
    image_id: ImageId,
}

/// Explicit state of the escrow for one Job.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EscrowState {
    /// The Job exists but has not been funded.
    Created,
    /// The buyer deposited the exact amount of the Job mint.
    Funded,
    /// The executor registered the commitment of its single delivery.
    Delivered {
        /// Artifact commitment that the released journal must match.
        artifact_hash: Hash32,
    },
    /// The executor was paid; no further transition exists.
    Released,
}

impl EscrowState {
    /// Returns whether no transition can leave this state.
    #[must_use]
    pub const fn is_terminal(&self) -> bool {
        matches!(self, Self::Released)
    }
}

/// Reason why an escrow transition is rejected.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EscrowError {
    /// The Job has not been funded yet.
    NotFunded,
    /// The Job was already funded.
    AlreadyFunded,
    /// The Job is funded, but no delivery commitment was registered.
    DeliveryNotRegistered,
    /// A delivery commitment was already registered for this Job.
    DeliveryAlreadyRegistered,
    /// The Job was already released.
    AlreadyReleased,
    /// The Job is in a terminal state.
    TerminalState,
    /// The depositor is not the buyer defined in the Job.
    DepositorMismatch,
    /// The submitter is not the executor defined in the Job.
    SubmitterMismatch,
    /// The release recipient is not the executor defined in the Job.
    RecipientMismatch,
    /// The mint is not the mint defined in the Job.
    MintMismatch,
    /// The deposited amount is not the amount defined in the Job.
    AmountMismatch,
    /// The journal does not match the commitments authorized for the Job.
    Journal(JournalValidationError),
    /// The journal matches the Job, but its verdict does not authorize release.
    VerdictNotPass,
}

/// Transfer that a release authorizes, derived only from the Job terms.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Payout {
    recipient: ExecutorId,
    mint: MintId,
    amount: Amount,
}

impl Payout {
    /// Returns the executor defined in the Job.
    #[must_use]
    pub const fn recipient(&self) -> ExecutorId {
        self.recipient
    }

    /// Returns the mint defined in the Job.
    #[must_use]
    pub const fn mint(&self) -> MintId {
        self.mint
    }

    /// Returns the amount defined in the Job.
    #[must_use]
    pub const fn amount(&self) -> Amount {
        self.amount
    }
}

impl JobV1 {
    /// Creates immutable Job terms.
    ///
    /// All-zero identities and a buyer equal to the executor are rejected.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        job_id: JobId,
        buyer: BuyerId,
        executor: ExecutorId,
        mint: MintId,
        amount: Amount,
        spec_hash: Hash32,
        harness_hash: Hash32,
        image_id: ImageId,
    ) -> Result<Self, JobError> {
        if is_zero(buyer.as_bytes()) {
            return Err(JobError::ZeroIdentity(JobParty::Buyer));
        }
        if is_zero(executor.as_bytes()) {
            return Err(JobError::ZeroIdentity(JobParty::Executor));
        }
        if is_zero(mint.as_bytes()) {
            return Err(JobError::ZeroIdentity(JobParty::Mint));
        }
        if buyer.as_bytes() == executor.as_bytes() {
            return Err(JobError::BuyerIsExecutor);
        }

        Ok(Self {
            job_id,
            buyer,
            executor,
            mint,
            amount,
            spec_hash,
            harness_hash,
            image_id,
        })
    }

    /// Returns the Job identifier.
    #[must_use]
    pub const fn job_id(&self) -> JobId {
        self.job_id
    }

    /// Returns the buyer identity.
    #[must_use]
    pub const fn buyer(&self) -> BuyerId {
        self.buyer
    }

    /// Returns the executor identity.
    #[must_use]
    pub const fn executor(&self) -> ExecutorId {
        self.executor
    }

    /// Returns the mint identity.
    #[must_use]
    pub const fn mint(&self) -> MintId {
        self.mint
    }

    /// Returns the escrow amount.
    #[must_use]
    pub const fn amount(&self) -> Amount {
        self.amount
    }

    /// Returns the specification commitment agreed before funding.
    #[must_use]
    pub const fn spec_hash(&self) -> Hash32 {
        self.spec_hash
    }

    /// Returns the harness commitment agreed before funding.
    #[must_use]
    pub const fn harness_hash(&self) -> Hash32 {
        self.harness_hash
    }

    /// Returns the admitted guest image identifier.
    #[must_use]
    pub const fn image_id(&self) -> ImageId {
        self.image_id
    }

    /// Returns the journal commitments authorized for a registered delivery.
    #[must_use]
    pub const fn expected_commitments(&self, artifact_hash: Hash32) -> JournalV1Commitments {
        JournalV1Commitments::new(
            self.job_id,
            self.spec_hash,
            self.harness_hash,
            artifact_hash,
            self.image_id,
        )
    }

    /// Accepts the deposit of the exact Job amount and mint by the Job buyer.
    pub fn fund(
        &self,
        state: EscrowState,
        depositor: BuyerId,
        mint: MintId,
        amount: Amount,
    ) -> Result<EscrowState, EscrowError> {
        match state {
            EscrowState::Created => {}
            EscrowState::Funded | EscrowState::Delivered { .. } => {
                return Err(EscrowError::AlreadyFunded)
            }
            EscrowState::Released => return Err(EscrowError::TerminalState),
        }
        if depositor != self.buyer {
            return Err(EscrowError::DepositorMismatch);
        }
        if mint != self.mint {
            return Err(EscrowError::MintMismatch);
        }
        if amount != self.amount {
            return Err(EscrowError::AmountMismatch);
        }

        Ok(EscrowState::Funded)
    }

    /// Registers, once, the artifact commitment delivered by the Job executor.
    pub fn register_delivery(
        &self,
        state: EscrowState,
        submitter: ExecutorId,
        artifact_hash: Hash32,
    ) -> Result<EscrowState, EscrowError> {
        match state {
            EscrowState::Created => return Err(EscrowError::NotFunded),
            EscrowState::Funded => {}
            EscrowState::Delivered { .. } => return Err(EscrowError::DeliveryAlreadyRegistered),
            EscrowState::Released => return Err(EscrowError::TerminalState),
        }
        if submitter != self.executor {
            return Err(EscrowError::SubmitterMismatch);
        }

        Ok(EscrowState::Delivered { artifact_hash })
    }

    /// Checks whether a journal makes the Job eligible for release.
    ///
    /// Precondition: a future adapter has already verified the receipt that
    /// produced `journal` against the Job image. This function does not
    /// verify any receipt, seal, Groth16 proof, Router or CPI.
    ///
    /// Checks run in this order: state, journal commitments (schema, Job,
    /// specification, harness, registered artifact, image), `Verdict::Pass`,
    /// recipient and mint. The returned payout comes only from the Job terms.
    pub fn check_release(
        &self,
        state: EscrowState,
        journal: &JournalV1,
        recipient: ExecutorId,
        mint: MintId,
    ) -> Result<Payout, EscrowError> {
        let artifact_hash = match state {
            EscrowState::Created => return Err(EscrowError::NotFunded),
            EscrowState::Funded => return Err(EscrowError::DeliveryNotRegistered),
            EscrowState::Delivered { artifact_hash } => artifact_hash,
            EscrowState::Released => return Err(EscrowError::AlreadyReleased),
        };

        let verdict = journal
            .validate_against(&self.expected_commitments(artifact_hash))
            .map_err(EscrowError::Journal)?;
        if verdict != Verdict::Pass {
            return Err(EscrowError::VerdictNotPass);
        }
        if recipient != self.executor {
            return Err(EscrowError::RecipientMismatch);
        }
        if mint != self.mint {
            return Err(EscrowError::MintMismatch);
        }

        Ok(Payout {
            recipient: self.executor,
            mint: self.mint,
            amount: self.amount,
        })
    }

    /// Releases an eligible Job, returning the terminal state and its payout.
    ///
    /// Same precondition and checks as [`JobV1::check_release`].
    pub fn release(
        &self,
        state: EscrowState,
        journal: &JournalV1,
        recipient: ExecutorId,
        mint: MintId,
    ) -> Result<(EscrowState, Payout), EscrowError> {
        let payout = self.check_release(state, journal, recipient, mint)?;
        Ok((EscrowState::Released, payout))
    }
}

fn is_zero(bytes: &[u8; 32]) -> bool {
    bytes.iter().all(|byte| *byte == 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        evaluate_restricted_artifact, hash_harness_version, hash_restricted_spec,
        RestrictedArtifactV1, DETERMINISTIC_HARNESS_VERSION, RESTRICTED_SPEC_V1,
    };
    use core::fmt::Debug;

    const JOB_ID: JobId = JobId::new([0x11; 32]);
    // Development placeholder only; not the ImageID of a built guest.
    const IMAGE_ID: ImageId = ImageId::new([0x55; 32]);
    const BUYER: BuyerId = BuyerId::new([0xb1; 32]);
    const EXECUTOR: ExecutorId = ExecutorId::new([0xe1; 32]);
    const MINT: MintId = MintId::new([0x4d; 32]);
    const OTHER: [u8; 32] = [0x99; 32];
    const AMOUNT_UNITS: u64 = 1_000_000;

    fn require_ok<T, E: Debug>(result: Result<T, E>) -> T {
        match result {
            Ok(value) => value,
            Err(error) => panic!("expected Ok, got {error:?}"),
        }
    }

    fn amount() -> Amount {
        require_ok(Amount::new(AMOUNT_UNITS))
    }

    fn spec_hash() -> Hash32 {
        require_ok(hash_restricted_spec(&RESTRICTED_SPEC_V1))
    }

    fn harness_hash() -> Hash32 {
        require_ok(hash_harness_version(DETERMINISTIC_HARNESS_VERSION))
    }

    fn job() -> JobV1 {
        require_ok(JobV1::new(
            JOB_ID,
            BUYER,
            EXECUTOR,
            MINT,
            amount(),
            spec_hash(),
            harness_hash(),
            IMAGE_ID,
        ))
    }

    fn journal(input: u32, claimed_output: u32) -> JournalV1 {
        let bytes = require_ok(RestrictedArtifactV1::new(input, claimed_output).encode_candidate());
        require_ok(evaluate_restricted_artifact(JOB_ID, IMAGE_ID, &bytes))
    }

    fn pass_journal() -> JournalV1 {
        journal(7, 14)
    }

    fn fail_journal() -> JournalV1 {
        journal(7, 15)
    }

    fn funded(job: &JobV1) -> EscrowState {
        require_ok(job.fund(EscrowState::Created, BUYER, MINT, amount()))
    }

    fn delivered(job: &JobV1, artifact_hash: Hash32) -> EscrowState {
        require_ok(job.register_delivery(funded(job), EXECUTOR, artifact_hash))
    }

    fn pass_with(
        job_id: JobId,
        spec_hash: Hash32,
        harness_hash: Hash32,
        image_id: ImageId,
    ) -> JournalV1 {
        JournalV1::new(
            job_id,
            spec_hash,
            harness_hash,
            pass_journal().artifact_hash(),
            image_id,
            Verdict::Pass,
        )
    }

    fn release_error(journal: &JournalV1) -> Result<(EscrowState, Payout), EscrowError> {
        let job = job();
        let state = delivered(&job, pass_journal().artifact_hash());
        job.release(state, journal, EXECUTOR, MINT)
    }

    #[test]
    fn eligible_pass_releases_only_to_the_job_executor_and_mint() {
        let job = job();
        let journal = pass_journal();
        assert_eq!(journal.verdict(), Verdict::Pass);
        let state = delivered(&job, journal.artifact_hash());

        let payout = require_ok(job.check_release(state, &journal, EXECUTOR, MINT));
        let (next, released) = require_ok(job.release(state, &journal, EXECUTOR, MINT));

        assert_eq!(next, EscrowState::Released);
        assert!(next.is_terminal());
        assert_eq!(released, payout);
        assert_eq!(payout.recipient(), job.executor());
        assert_eq!(payout.mint(), job.mint());
        assert_eq!(payout.amount().base_units(), AMOUNT_UNITS);
    }

    #[test]
    fn fail_verdict_matching_the_job_is_not_eligible() {
        let job = job();
        let journal = fail_journal();
        let state = delivered(&job, journal.artifact_hash());

        assert_eq!(
            journal.validate_against(&job.expected_commitments(journal.artifact_hash())),
            Ok(Verdict::Fail)
        );
        assert_eq!(
            job.release(state, &journal, EXECUTOR, MINT),
            Err(EscrowError::VerdictNotPass)
        );
    }

    #[test]
    fn rejects_a_journal_for_a_different_job_id() {
        let journal = pass_with(JobId::new(OTHER), spec_hash(), harness_hash(), IMAGE_ID);
        assert_eq!(
            release_error(&journal),
            Err(EscrowError::Journal(JournalValidationError::JobIdMismatch))
        );
    }

    #[test]
    fn rejects_a_journal_with_a_different_spec_hash() {
        let journal = pass_with(JOB_ID, Hash32::new(OTHER), harness_hash(), IMAGE_ID);
        assert_eq!(
            release_error(&journal),
            Err(EscrowError::Journal(JournalValidationError::SpecHashMismatch))
        );
    }

    #[test]
    fn rejects_a_journal_with_a_different_harness_hash() {
        let journal = pass_with(JOB_ID, spec_hash(), Hash32::new(OTHER), IMAGE_ID);
        assert_eq!(
            release_error(&journal),
            Err(EscrowError::Journal(JournalValidationError::HarnessHashMismatch))
        );
    }

    #[test]
    fn rejects_a_passing_journal_for_a_different_artifact_than_registered() {
        let other_pass = journal(8, 16);
        assert_eq!(other_pass.verdict(), Verdict::Pass);
        assert_ne!(other_pass.artifact_hash(), pass_journal().artifact_hash());

        assert_eq!(
            release_error(&other_pass),
            Err(EscrowError::Journal(JournalValidationError::ArtifactHashMismatch))
        );
    }

    #[test]
    fn rejects_a_journal_with_a_different_image_id() {
        let journal = pass_with(JOB_ID, spec_hash(), harness_hash(), ImageId::new(OTHER));
        assert_eq!(
            release_error(&journal),
            Err(EscrowError::Journal(JournalValidationError::ImageIdMismatch))
        );
    }

    #[test]
    fn rejects_a_recipient_other_than_the_job_executor() {
        let job = job();
        let journal = pass_journal();
        let state = delivered(&job, journal.artifact_hash());

        for recipient in [ExecutorId::new(OTHER), ExecutorId::new(*BUYER.as_bytes())] {
            assert_eq!(
                job.release(state, &journal, recipient, MINT),
                Err(EscrowError::RecipientMismatch)
            );
        }
    }

    #[test]
    fn rejects_a_mint_other_than_the_job_mint() {
        let job = job();
        let journal = pass_journal();
        let state = delivered(&job, journal.artifact_hash());

        assert_eq!(
            job.release(state, &journal, EXECUTOR, MintId::new(OTHER)),
            Err(EscrowError::MintMismatch)
        );
    }

    #[test]
    fn rejects_release_of_a_job_that_is_not_funded_or_delivered() {
        let job = job();
        let journal = pass_journal();

        assert_eq!(
            job.release(EscrowState::Created, &journal, EXECUTOR, MINT),
            Err(EscrowError::NotFunded)
        );
        assert_eq!(
            job.release(funded(&job), &journal, EXECUTOR, MINT),
            Err(EscrowError::DeliveryNotRegistered)
        );
    }

    #[test]
    fn rejects_a_duplicate_release() {
        let job = job();
        let journal = pass_journal();
        let state = delivered(&job, journal.artifact_hash());
        let (released, _) = require_ok(job.release(state, &journal, EXECUTOR, MINT));

        assert_eq!(
            job.release(released, &journal, EXECUTOR, MINT),
            Err(EscrowError::AlreadyReleased)
        );
        assert_eq!(
            job.check_release(released, &journal, EXECUTOR, MINT),
            Err(EscrowError::AlreadyReleased)
        );
    }

    #[test]
    fn rejects_transitions_out_of_the_terminal_state() {
        let job = job();

        assert_eq!(
            job.fund(EscrowState::Released, BUYER, MINT, amount()),
            Err(EscrowError::TerminalState)
        );
        assert_eq!(
            job.register_delivery(
                EscrowState::Released,
                EXECUTOR,
                pass_journal().artifact_hash()
            ),
            Err(EscrowError::TerminalState)
        );
    }

    #[test]
    fn no_state_journal_recipient_or_mint_bypasses_the_policy() {
        let job = job();
        let pass = pass_journal();
        let fail = fail_journal();
        let states = [
            EscrowState::Created,
            EscrowState::Funded,
            EscrowState::Delivered {
                artifact_hash: pass.artifact_hash(),
            },
            EscrowState::Delivered {
                artifact_hash: fail.artifact_hash(),
            },
            EscrowState::Released,
        ];
        let recipients = [
            EXECUTOR,
            ExecutorId::new(*BUYER.as_bytes()),
            ExecutorId::new(OTHER),
        ];
        let mints = [MINT, MintId::new(OTHER)];

        let mut released = 0;
        for state in states {
            for journal in [pass, fail] {
                for recipient in recipients {
                    for mint in mints {
                        if let Ok((next, payout)) = job.release(state, &journal, recipient, mint) {
                            released += 1;
                            assert_eq!(
                                state,
                                EscrowState::Delivered {
                                    artifact_hash: pass.artifact_hash()
                                }
                            );
                            assert_eq!(journal, pass);
                            assert_eq!(next, EscrowState::Released);
                            assert_eq!(payout.recipient(), job.executor());
                            assert_eq!(payout.mint(), job.mint());
                            assert_eq!(payout.amount(), job.amount());
                        }
                    }
                }
            }
        }

        assert_eq!(released, 1);
    }

    #[test]
    fn job_construction_rejects_invalid_terms() {
        let build = |buyer: BuyerId, executor: ExecutorId, mint: MintId| {
            JobV1::new(
                JOB_ID,
                buyer,
                executor,
                mint,
                amount(),
                spec_hash(),
                harness_hash(),
                IMAGE_ID,
            )
        };

        assert_eq!(
            build(BuyerId::new([0; 32]), EXECUTOR, MINT),
            Err(JobError::ZeroIdentity(JobParty::Buyer))
        );
        assert_eq!(
            build(BUYER, ExecutorId::new([0; 32]), MINT),
            Err(JobError::ZeroIdentity(JobParty::Executor))
        );
        assert_eq!(
            build(BUYER, EXECUTOR, MintId::new([0; 32])),
            Err(JobError::ZeroIdentity(JobParty::Mint))
        );
        assert_eq!(
            build(BUYER, ExecutorId::new(*BUYER.as_bytes()), MINT),
            Err(JobError::BuyerIsExecutor)
        );
        assert_eq!(Amount::new(0), Err(AmountError::Zero));
    }

    #[test]
    fn funding_requires_the_exact_buyer_mint_and_amount_once() {
        let job = job();
        let created = EscrowState::Created;

        assert_eq!(job.fund(created, BUYER, MINT, amount()), Ok(EscrowState::Funded));
        assert_eq!(
            job.fund(created, BuyerId::new(*EXECUTOR.as_bytes()), MINT, amount()),
            Err(EscrowError::DepositorMismatch)
        );
        assert_eq!(
            job.fund(created, BUYER, MintId::new(OTHER), amount()),
            Err(EscrowError::MintMismatch)
        );
        assert_eq!(
            job.fund(created, BUYER, MINT, require_ok(Amount::new(AMOUNT_UNITS - 1))),
            Err(EscrowError::AmountMismatch)
        );
        assert_eq!(
            job.fund(funded(&job), BUYER, MINT, amount()),
            Err(EscrowError::AlreadyFunded)
        );
        assert_eq!(
            job.fund(
                delivered(&job, pass_journal().artifact_hash()),
                BUYER,
                MINT,
                amount()
            ),
            Err(EscrowError::AlreadyFunded)
        );
    }

    #[test]
    fn delivery_is_registered_once_by_the_job_executor_after_funding() {
        let job = job();
        let pass_hash = pass_journal().artifact_hash();
        let fail_hash = fail_journal().artifact_hash();

        assert_eq!(
            job.register_delivery(EscrowState::Created, EXECUTOR, pass_hash),
            Err(EscrowError::NotFunded)
        );
        assert_eq!(
            job.register_delivery(funded(&job), ExecutorId::new(*BUYER.as_bytes()), pass_hash),
            Err(EscrowError::SubmitterMismatch)
        );
        assert_eq!(
            job.register_delivery(funded(&job), EXECUTOR, pass_hash),
            Ok(EscrowState::Delivered {
                artifact_hash: pass_hash
            })
        );
        assert_eq!(
            job.register_delivery(delivered(&job, fail_hash), EXECUTOR, pass_hash),
            Err(EscrowError::DeliveryAlreadyRegistered)
        );
    }

    #[test]
    fn transitions_do_not_change_the_job_terms() {
        let job = job();
        let before = job;
        let journal = pass_journal();
        let state = delivered(&job, journal.artifact_hash());
        let _ = job.release(state, &journal, ExecutorId::new(OTHER), MINT);
        let _ = require_ok(job.release(state, &journal, EXECUTOR, MINT));

        assert_eq!(job, before);
        assert_eq!(job.job_id(), JOB_ID);
        assert_eq!(job.buyer(), BUYER);
        assert_eq!(job.executor(), EXECUTOR);
        assert_eq!(job.mint(), MINT);
        assert_eq!(job.amount(), amount());
        assert_eq!(job.spec_hash(), spec_hash());
        assert_eq!(job.harness_hash(), harness_hash());
        assert_eq!(job.image_id(), IMAGE_ID);
    }
}
