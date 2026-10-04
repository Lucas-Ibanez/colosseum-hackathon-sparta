//! Pure escrow policy for an immutable [`JobV1`].
//!
//! This module only decides whether a requested transition is allowed by the
//! Job terms. It holds no funds, moves no tokens, reads no clock and has no
//! administrative override: the current slot is an explicit input. It does
//! not verify receipts, seals, Groth16 proofs, Router deployments or CPI: a
//! future adapter must have verified the receipt before passing its journal
//! here.
//!
//! Settlement partitions time at `deadline_slot`: up to and including the
//! deadline a valid `Pass` releases to the executor; a valid `Fail` refunds
//! the buyer at any time; after the deadline a timeout refunds the buyer.

use crate::{Hash32, ImageId, JobId, JournalV1, JournalV1Commitments, JournalValidationError, Verdict};

/// 32-byte identity of the buyer that creates, funds and receives refunds.
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
/// Fields are fixed at construction and only readable afterwards. The
/// artifact commitment is not part of the terms: it is recorded from the
/// journal at settlement.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JobV1 {
    job_id: JobId,
    buyer: BuyerId,
    executor: ExecutorId,
    mint: MintId,
    amount: Amount,
    deadline_slot: u64,
    spec_hash: Hash32,
    harness_hash: Hash32,
    image_id: ImageId,
}

/// Why a Job was refunded to its buyer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RefundReason {
    /// A journal bound to the Job carried `Verdict::Fail`.
    Fail {
        /// Artifact commitment recorded from the settling journal.
        artifact_hash: Hash32,
    },
    /// The deadline passed without a verdict settlement.
    Timeout,
}

/// Explicit economic state of the escrow for one Job.
///
/// Worker and UI states such as proving or submitted are not economic states
/// and have no transition here.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EscrowState {
    /// The Job exists but has not been funded.
    Created,
    /// The buyer deposited the exact amount of the Job mint.
    Funded,
    /// The executor was paid; terminal.
    Released {
        /// Artifact commitment recorded from the settling journal.
        artifact_hash: Hash32,
    },
    /// The buyer was refunded; terminal.
    Refunded {
        /// Why the refund happened.
        reason: RefundReason,
    },
}

impl EscrowState {
    /// Returns whether no transition can leave this state.
    #[must_use]
    pub const fn is_terminal(&self) -> bool {
        matches!(self, Self::Released { .. } | Self::Refunded { .. })
    }
}

/// Reason why an escrow transition is rejected.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EscrowError {
    /// The Job has not been funded yet.
    NotFunded,
    /// The Job was already funded.
    AlreadyFunded,
    /// The Job was already released to the executor.
    AlreadyReleased,
    /// The Job was already refunded to the buyer.
    AlreadyRefunded,
    /// The depositor is not the buyer defined in the Job.
    DepositorMismatch,
    /// The recipient is not the party the Job pays in this settlement.
    RecipientMismatch,
    /// The mint is not the mint defined in the Job.
    MintMismatch,
    /// The deposited amount is not the amount defined in the Job.
    AmountMismatch,
    /// The journal is not bound to this Job's commitments.
    Journal(JournalValidationError),
    /// The journal is bound to the Job, but its verdict is not `Pass`.
    VerdictNotPass,
    /// The journal is bound to the Job, but its verdict is not `Fail`.
    VerdictNotFail,
    /// A timeout refund was requested at or before the deadline slot.
    DeadlineNotReached,
    /// A release was requested after the deadline slot.
    DeadlinePassed,
}

/// Party paid by a settlement.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PayoutRecipient {
    /// The executor defined in the Job.
    Executor(ExecutorId),
    /// The buyer defined in the Job.
    Buyer(BuyerId),
}

/// Transfer that a settlement authorizes, derived only from the Job terms.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Payout {
    recipient: PayoutRecipient,
    mint: MintId,
    amount: Amount,
}

impl Payout {
    /// Returns the Job party that receives the tokens.
    #[must_use]
    pub const fn recipient(&self) -> PayoutRecipient {
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

/// Terminal state and payout produced by an accepted settlement.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Settlement {
    state: EscrowState,
    payout: Payout,
}

impl Settlement {
    /// Returns the terminal state that must be persisted with the transfer.
    #[must_use]
    pub const fn state(&self) -> EscrowState {
        self.state
    }

    /// Returns the transfer that must happen atomically with the state change.
    #[must_use]
    pub const fn payout(&self) -> Payout {
        self.payout
    }
}

impl JobV1 {
    /// Creates immutable Job terms.
    ///
    /// All-zero identities and a buyer equal to the executor are rejected.
    /// `deadline_slot` is not otherwise constrained; a past deadline only
    /// means that release is no longer possible.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        job_id: JobId,
        buyer: BuyerId,
        executor: ExecutorId,
        mint: MintId,
        amount: Amount,
        deadline_slot: u64,
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
            deadline_slot,
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

    /// Returns the last slot at which a `Pass` release is accepted.
    #[must_use]
    pub const fn deadline_slot(&self) -> u64 {
        self.deadline_slot
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
            EscrowState::Funded => return Err(EscrowError::AlreadyFunded),
            EscrowState::Released { .. } => return Err(EscrowError::AlreadyReleased),
            EscrowState::Refunded { .. } => return Err(EscrowError::AlreadyRefunded),
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

    /// Releases the Job to its executor on a bound `Pass` journal.
    ///
    /// Precondition: a future adapter has already verified the receipt that
    /// produced `journal`. This function does not verify any receipt, seal,
    /// Groth16 proof, Router or CPI.
    ///
    /// Checks run in this order: state, `current_slot <= deadline_slot`,
    /// journal binding (schema, Job, specification, harness, image),
    /// `Verdict::Pass`, recipient and mint. The journal artifact hash is
    /// recorded, not compared.
    pub fn release(
        &self,
        state: EscrowState,
        journal: &JournalV1,
        current_slot: u64,
        recipient: ExecutorId,
        mint: MintId,
    ) -> Result<Settlement, EscrowError> {
        require_funded(state)?;
        if current_slot > self.deadline_slot {
            return Err(EscrowError::DeadlinePassed);
        }
        if self.bound_verdict(journal)? != Verdict::Pass {
            return Err(EscrowError::VerdictNotPass);
        }
        if recipient != self.executor {
            return Err(EscrowError::RecipientMismatch);
        }
        if mint != self.mint {
            return Err(EscrowError::MintMismatch);
        }

        Ok(self.settle(
            EscrowState::Released {
                artifact_hash: journal.artifact_hash(),
            },
            PayoutRecipient::Executor(self.executor),
        ))
    }

    /// Refunds the Job to its buyer on a bound `Fail` journal, at any slot.
    ///
    /// Same precondition as [`JobV1::release`]. Checks run in this order:
    /// state, journal binding, `Verdict::Fail`, recipient and mint.
    pub fn refund_on_fail(
        &self,
        state: EscrowState,
        journal: &JournalV1,
        recipient: BuyerId,
        mint: MintId,
    ) -> Result<Settlement, EscrowError> {
        require_funded(state)?;
        if self.bound_verdict(journal)? != Verdict::Fail {
            return Err(EscrowError::VerdictNotFail);
        }
        if recipient != self.buyer {
            return Err(EscrowError::RecipientMismatch);
        }
        if mint != self.mint {
            return Err(EscrowError::MintMismatch);
        }

        Ok(self.settle(
            EscrowState::Refunded {
                reason: RefundReason::Fail {
                    artifact_hash: journal.artifact_hash(),
                },
            },
            PayoutRecipient::Buyer(self.buyer),
        ))
    }

    /// Refunds the Job to its buyer once `current_slot > deadline_slot`.
    ///
    /// Checks run in this order: state, deadline, recipient and mint.
    pub fn refund_on_timeout(
        &self,
        state: EscrowState,
        current_slot: u64,
        recipient: BuyerId,
        mint: MintId,
    ) -> Result<Settlement, EscrowError> {
        require_funded(state)?;
        if current_slot <= self.deadline_slot {
            return Err(EscrowError::DeadlineNotReached);
        }
        if recipient != self.buyer {
            return Err(EscrowError::RecipientMismatch);
        }
        if mint != self.mint {
            return Err(EscrowError::MintMismatch);
        }

        Ok(self.settle(
            EscrowState::Refunded {
                reason: RefundReason::Timeout,
            },
            PayoutRecipient::Buyer(self.buyer),
        ))
    }

    /// Returns the verdict of a journal bound to this Job.
    ///
    /// Reuses [`JournalV1::validate_against`] with the journal's own artifact
    /// hash, so schema, Job, specification, harness and image are compared
    /// while the artifact commitment is only recorded.
    fn bound_verdict(&self, journal: &JournalV1) -> Result<Verdict, EscrowError> {
        let expected = JournalV1Commitments::new(
            self.job_id,
            self.spec_hash,
            self.harness_hash,
            journal.artifact_hash(),
            self.image_id,
        );
        journal
            .validate_against(&expected)
            .map_err(EscrowError::Journal)
    }

    fn settle(&self, state: EscrowState, recipient: PayoutRecipient) -> Settlement {
        Settlement {
            state,
            payout: Payout {
                recipient,
                mint: self.mint,
                amount: self.amount,
            },
        }
    }
}

fn require_funded(state: EscrowState) -> Result<(), EscrowError> {
    match state {
        EscrowState::Created => Err(EscrowError::NotFunded),
        EscrowState::Funded => Ok(()),
        EscrowState::Released { .. } => Err(EscrowError::AlreadyReleased),
        EscrowState::Refunded { .. } => Err(EscrowError::AlreadyRefunded),
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
    const DEADLINE: u64 = 1_000;
    const SLOTS: [u64; 3] = [DEADLINE - 1, DEADLINE, DEADLINE + 1];

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
            DEADLINE,
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

    fn divergent(
        verdict: Verdict,
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
            verdict,
        )
    }

    fn divergent_journals(verdict: Verdict) -> [(JournalV1, JournalValidationError); 4] {
        [
            (
                divergent(verdict, JobId::new(OTHER), spec_hash(), harness_hash(), IMAGE_ID),
                JournalValidationError::JobIdMismatch,
            ),
            (
                divergent(verdict, JOB_ID, Hash32::new(OTHER), harness_hash(), IMAGE_ID),
                JournalValidationError::SpecHashMismatch,
            ),
            (
                divergent(verdict, JOB_ID, spec_hash(), Hash32::new(OTHER), IMAGE_ID),
                JournalValidationError::HarnessHashMismatch,
            ),
            (
                divergent(verdict, JOB_ID, spec_hash(), harness_hash(), ImageId::new(OTHER)),
                JournalValidationError::ImageIdMismatch,
            ),
        ]
    }

    fn terminal_states(job: &JobV1) -> [(EscrowState, EscrowError); 3] {
        let released = require_ok(job.release(funded(job), &pass_journal(), DEADLINE, EXECUTOR, MINT));
        let refunded_fail = require_ok(job.refund_on_fail(funded(job), &fail_journal(), BUYER, MINT));
        let refunded_timeout =
            require_ok(job.refund_on_timeout(funded(job), DEADLINE + 1, BUYER, MINT));
        [
            (released.state(), EscrowError::AlreadyReleased),
            (refunded_fail.state(), EscrowError::AlreadyRefunded),
            (refunded_timeout.state(), EscrowError::AlreadyRefunded),
        ]
    }

    fn assert_job_payout(settlement: &Settlement, recipient: PayoutRecipient) {
        assert!(settlement.state().is_terminal());
        assert_eq!(settlement.payout().recipient(), recipient);
        assert_eq!(settlement.payout().mint(), MINT);
        assert_eq!(settlement.payout().amount().base_units(), AMOUNT_UNITS);
    }

    #[test]
    fn pass_releases_to_the_executor_up_to_the_deadline_inclusive() {
        let job = job();
        let journal = pass_journal();

        for slot in [0, DEADLINE - 1, DEADLINE] {
            let settlement = require_ok(job.release(funded(&job), &journal, slot, EXECUTOR, MINT));
            assert_eq!(
                settlement.state(),
                EscrowState::Released {
                    artifact_hash: journal.artifact_hash()
                }
            );
            assert_job_payout(&settlement, PayoutRecipient::Executor(EXECUTOR));
        }
    }

    #[test]
    fn pass_after_the_deadline_is_rejected() {
        let job = job();

        assert_eq!(
            job.release(funded(&job), &pass_journal(), DEADLINE + 1, EXECUTOR, MINT),
            Err(EscrowError::DeadlinePassed)
        );
    }

    #[test]
    fn fail_refunds_the_buyer_before_and_after_the_deadline() {
        let job = job();
        let journal = fail_journal();

        let settlement = require_ok(job.refund_on_fail(funded(&job), &journal, BUYER, MINT));
        assert_eq!(
            settlement.state(),
            EscrowState::Refunded {
                reason: RefundReason::Fail {
                    artifact_hash: journal.artifact_hash()
                }
            }
        );
        assert_job_payout(&settlement, PayoutRecipient::Buyer(BUYER));

        // The refund on `Fail` takes no slot: it is valid on either side of the
        // deadline, and the release path is closed after it.
        assert_eq!(
            job.release(funded(&job), &journal, DEADLINE + 1, EXECUTOR, MINT),
            Err(EscrowError::DeadlinePassed)
        );
    }

    #[test]
    fn fail_never_releases_and_pass_never_refunds_on_fail() {
        let job = job();

        assert_eq!(
            job.release(funded(&job), &fail_journal(), DEADLINE, EXECUTOR, MINT),
            Err(EscrowError::VerdictNotPass)
        );
        assert_eq!(
            job.refund_on_fail(funded(&job), &pass_journal(), BUYER, MINT),
            Err(EscrowError::VerdictNotFail)
        );
    }

    #[test]
    fn timeout_refunds_the_buyer_only_after_the_deadline() {
        let job = job();

        for slot in [0, DEADLINE - 1, DEADLINE] {
            assert_eq!(
                job.refund_on_timeout(funded(&job), slot, BUYER, MINT),
                Err(EscrowError::DeadlineNotReached)
            );
        }

        let settlement = require_ok(job.refund_on_timeout(funded(&job), DEADLINE + 1, BUYER, MINT));
        assert_eq!(
            settlement.state(),
            EscrowState::Refunded {
                reason: RefundReason::Timeout
            }
        );
        assert_job_payout(&settlement, PayoutRecipient::Buyer(BUYER));
    }

    #[test]
    fn release_rejects_each_divergent_commitment() {
        let job = job();

        for (journal, error) in divergent_journals(Verdict::Pass) {
            assert_eq!(
                job.release(funded(&job), &journal, DEADLINE, EXECUTOR, MINT),
                Err(EscrowError::Journal(error))
            );
        }
    }

    #[test]
    fn refund_on_fail_rejects_each_divergent_commitment() {
        let job = job();

        for (journal, error) in divergent_journals(Verdict::Fail) {
            assert_eq!(
                job.refund_on_fail(funded(&job), &journal, BUYER, MINT),
                Err(EscrowError::Journal(error))
            );
        }
    }

    #[test]
    fn artifact_hash_is_recorded_from_the_journal_at_settlement() {
        let job = job();
        let other_pass = journal(8, 16);
        assert_eq!(other_pass.verdict(), Verdict::Pass);
        assert_ne!(other_pass.artifact_hash(), pass_journal().artifact_hash());

        let settlement = require_ok(job.release(funded(&job), &other_pass, DEADLINE, EXECUTOR, MINT));
        assert_eq!(
            settlement.state(),
            EscrowState::Released {
                artifact_hash: other_pass.artifact_hash()
            }
        );
    }

    #[test]
    fn rejects_recipients_other_than_the_paid_job_party() {
        let job = job();

        for recipient in [ExecutorId::new(OTHER), ExecutorId::new(*BUYER.as_bytes())] {
            assert_eq!(
                job.release(funded(&job), &pass_journal(), DEADLINE, recipient, MINT),
                Err(EscrowError::RecipientMismatch)
            );
        }
        for recipient in [BuyerId::new(OTHER), BuyerId::new(*EXECUTOR.as_bytes())] {
            assert_eq!(
                job.refund_on_fail(funded(&job), &fail_journal(), recipient, MINT),
                Err(EscrowError::RecipientMismatch)
            );
            assert_eq!(
                job.refund_on_timeout(funded(&job), DEADLINE + 1, recipient, MINT),
                Err(EscrowError::RecipientMismatch)
            );
        }
    }

    #[test]
    fn rejects_a_mint_other_than_the_job_mint() {
        let job = job();
        let other = MintId::new(OTHER);

        assert_eq!(
            job.release(funded(&job), &pass_journal(), DEADLINE, EXECUTOR, other),
            Err(EscrowError::MintMismatch)
        );
        assert_eq!(
            job.refund_on_fail(funded(&job), &fail_journal(), BUYER, other),
            Err(EscrowError::MintMismatch)
        );
        assert_eq!(
            job.refund_on_timeout(funded(&job), DEADLINE + 1, BUYER, other),
            Err(EscrowError::MintMismatch)
        );
    }

    #[test]
    fn rejects_settlement_of_an_unfunded_job() {
        let job = job();
        let created = EscrowState::Created;

        assert_eq!(
            job.release(created, &pass_journal(), DEADLINE, EXECUTOR, MINT),
            Err(EscrowError::NotFunded)
        );
        assert_eq!(
            job.refund_on_fail(created, &fail_journal(), BUYER, MINT),
            Err(EscrowError::NotFunded)
        );
        assert_eq!(
            job.refund_on_timeout(created, DEADLINE + 1, BUYER, MINT),
            Err(EscrowError::NotFunded)
        );
    }

    #[test]
    fn rejects_double_settlement_and_funding_of_a_terminal_job() {
        let job = job();

        for (state, error) in terminal_states(&job) {
            assert!(state.is_terminal());
            assert_eq!(
                job.release(state, &pass_journal(), DEADLINE, EXECUTOR, MINT),
                Err(error)
            );
            assert_eq!(
                job.refund_on_fail(state, &fail_journal(), BUYER, MINT),
                Err(error)
            );
            assert_eq!(
                job.refund_on_timeout(state, DEADLINE + 1, BUYER, MINT),
                Err(error)
            );
            assert_eq!(job.fund(state, BUYER, MINT, amount()), Err(error));
        }
    }

    #[test]
    fn no_input_combination_bypasses_the_policy() {
        let job = job();
        let pass = pass_journal();
        let fail = fail_journal();
        let foreign_pass = divergent(Verdict::Pass, JobId::new(OTHER), spec_hash(), harness_hash(), IMAGE_ID);
        let [released, refunded_fail, refunded_timeout] = terminal_states(&job);
        let states = [
            EscrowState::Created,
            EscrowState::Funded,
            released.0,
            refunded_fail.0,
            refunded_timeout.0,
        ];
        let journals = [pass, fail, foreign_pass];
        let mints = [MINT, MintId::new(OTHER)];
        let parties = [*EXECUTOR.as_bytes(), *BUYER.as_bytes(), OTHER];

        let mut accepted = 0;
        let mut expected_accepted = 0;
        for state in states {
            for journal in journals {
                for slot in SLOTS {
                    for party in parties {
                        for mint in mints {
                            let funded = state == EscrowState::Funded && mint == MINT;
                            let bound = journal.job_id() == JOB_ID;

                            let release =
                                job.release(state, &journal, slot, ExecutorId::new(party), mint);
                            let release_ok = funded
                                && bound
                                && slot <= DEADLINE
                                && journal.verdict() == Verdict::Pass
                                && party == *EXECUTOR.as_bytes();
                            assert_eq!(release.is_ok(), release_ok);
                            if let Ok(settlement) = release {
                                assert_job_payout(&settlement, PayoutRecipient::Executor(EXECUTOR));
                            }

                            let on_fail =
                                job.refund_on_fail(state, &journal, BuyerId::new(party), mint);
                            let on_fail_ok = funded
                                && bound
                                && journal.verdict() == Verdict::Fail
                                && party == *BUYER.as_bytes();
                            assert_eq!(on_fail.is_ok(), on_fail_ok);
                            if let Ok(settlement) = on_fail {
                                assert_job_payout(&settlement, PayoutRecipient::Buyer(BUYER));
                            }

                            let timeout =
                                job.refund_on_timeout(state, slot, BuyerId::new(party), mint);
                            let timeout_ok =
                                funded && slot > DEADLINE && party == *BUYER.as_bytes();
                            assert_eq!(timeout.is_ok(), timeout_ok);
                            if let Ok(settlement) = timeout {
                                assert_job_payout(&settlement, PayoutRecipient::Buyer(BUYER));
                            }

                            for ok in [release_ok, on_fail_ok, timeout_ok] {
                                expected_accepted += usize::from(ok);
                            }
                            for ok in [release.is_ok(), on_fail.is_ok(), timeout.is_ok()] {
                                accepted += usize::from(ok);
                            }
                        }
                    }
                }
            }
        }

        // From Funded with the Job mint: 2 Pass releases (slots up to the
        // deadline), 3 Fail refunds (any slot) and 3 timeout refunds (one
        // slot after the deadline, for each of the 3 journals).
        assert_eq!(expected_accepted, 8);
        assert_eq!(accepted, expected_accepted);
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
                DEADLINE,
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
    }

    #[test]
    fn transitions_do_not_change_the_job_terms() {
        let job = job();
        let before = job;
        let _ = job.release(funded(&job), &pass_journal(), DEADLINE, ExecutorId::new(OTHER), MINT);
        let _ = require_ok(job.release(funded(&job), &pass_journal(), DEADLINE, EXECUTOR, MINT));
        let _ = require_ok(job.refund_on_fail(funded(&job), &fail_journal(), BUYER, MINT));
        let _ = require_ok(job.refund_on_timeout(funded(&job), DEADLINE + 1, BUYER, MINT));

        assert_eq!(job, before);
        assert_eq!(job.job_id(), JOB_ID);
        assert_eq!(job.buyer(), BUYER);
        assert_eq!(job.executor(), EXECUTOR);
        assert_eq!(job.mint(), MINT);
        assert_eq!(job.amount(), amount());
        assert_eq!(job.deadline_slot(), DEADLINE);
        assert_eq!(job.spec_hash(), spec_hash());
        assert_eq!(job.harness_hash(), harness_hash());
        assert_eq!(job.image_id(), IMAGE_ID);
    }
}
