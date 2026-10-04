//! Pure escrow policy for an immutable [`JobV1`].
//!
//! This module only decides whether a requested transition is allowed by the
//! Job terms. It holds no funds, moves no tokens, reads no clock and has no
//! administrative override: the current slot is an explicit input. It does
//! not verify receipts, seals, Groth16 proofs, Router deployments or CPI: a
//! future adapter must have verified the receipt before passing its journal
//! here.
//!
//! Settlement is bound to the executor's delivery: up to and including
//! `deadline_slot` the executor commits the hash of the delivered artifact
//! once, and only a journal for exactly that artifact can settle by verdict.
//! Up to and including the deadline a valid `Pass` releases to the executor;
//! a valid `Fail` refunds the buyer at any time; after the deadline a timeout
//! refunds the buyer, delivered or not.

use crate::{
    hash_harness_version, hash_restricted_spec, Hash32, ImageId, JobId, JournalV1,
    JournalV1Commitments, JournalValidationError, Verdict, DETERMINISTIC_HARNESS_VERSION,
    RESTRICTED_SPEC_V1,
};

/// Fewest slots between Job creation and its deadline (about 10 minutes at
/// 400 ms per slot), so the executor has time to deliver and prove.
pub const MIN_DEADLINE_WINDOW_SLOTS: u64 = 1_500;

/// Most slots between Job creation and its deadline (about 7 days at 400 ms
/// per slot), so a timeout refund is always reachable.
pub const MAX_DEADLINE_WINDOW_SLOTS: u64 = 1_512_000;

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
    /// The specification commitment is not the admitted v1 specification.
    SpecNotAdmitted,
    /// The harness commitment is not the admitted v1 harness.
    HarnessNotAdmitted,
    /// The image identifier is not the admitted guest image.
    ImageIdNotAdmitted,
    /// The deadline is outside the creation window.
    DeadlineOutOfWindow,
}

/// Immutable terms of one escrowed VeriCode Job.
///
/// Fields are fixed at construction and only readable afterwards. The
/// artifact commitment is not part of the terms: the executor commits it with
/// [`JobV1::deliver`], and settlement by verdict requires a journal for
/// exactly that commitment.
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
    /// The executor committed the delivered artifact; not terminal.
    Delivered {
        /// Commitment to the delivered artifact, with the semantics of
        /// [`crate::hash_restricted_artifact`].
        artifact_hash: Hash32,
    },
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
    /// A settlement by verdict was requested before any delivery.
    NotDelivered,
    /// The executor already committed a delivery.
    AlreadyDelivered,
    /// The deliverer is not the executor defined in the Job.
    DelivererMismatch,
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
    /// The admitted v1 terms and the deadline window are creation rules
    /// checked separately by [`JobV1::admit`], so that terms persisted at
    /// creation can be rebuilt later at any slot.
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

    /// Checks the rules that admit these terms when the Job is created.
    ///
    /// Checks run in this order: the specification and harness commitments
    /// equal the v1 values computed by this crate, the image identifier equals
    /// `admitted_image_id`, and `current_slot + MIN_DEADLINE_WINDOW_SLOTS <=
    /// deadline_slot <= current_slot + MAX_DEADLINE_WINDOW_SLOTS`. A v1
    /// commitment that cannot be computed admits nothing.
    pub fn admit(&self, admitted_image_id: ImageId, current_slot: u64) -> Result<(), JobError> {
        if hash_restricted_spec(&RESTRICTED_SPEC_V1).ok() != Some(self.spec_hash) {
            return Err(JobError::SpecNotAdmitted);
        }
        if hash_harness_version(DETERMINISTIC_HARNESS_VERSION).ok() != Some(self.harness_hash) {
            return Err(JobError::HarnessNotAdmitted);
        }
        if self.image_id != admitted_image_id {
            return Err(JobError::ImageIdNotAdmitted);
        }
        match self.deadline_slot.checked_sub(current_slot) {
            Some(window)
                if (MIN_DEADLINE_WINDOW_SLOTS..=MAX_DEADLINE_WINDOW_SLOTS).contains(&window) =>
            {
                Ok(())
            }
            _ => Err(JobError::DeadlineOutOfWindow),
        }
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

    /// Records the executor's single commitment to the delivered artifact.
    ///
    /// `artifact_hash` has the semantics of [`crate::hash_restricted_artifact`],
    /// the same as the journal `artifact_hash`. Checks run in this order:
    /// state (`Funded` only), deliverer and `current_slot <= deadline_slot`.
    pub fn deliver(
        &self,
        state: EscrowState,
        deliverer: ExecutorId,
        artifact_hash: Hash32,
        current_slot: u64,
    ) -> Result<EscrowState, EscrowError> {
        match state {
            EscrowState::Created => return Err(EscrowError::NotFunded),
            EscrowState::Funded => {}
            EscrowState::Delivered { .. } => return Err(EscrowError::AlreadyDelivered),
            EscrowState::Released { .. } => return Err(EscrowError::AlreadyReleased),
            EscrowState::Refunded { .. } => return Err(EscrowError::AlreadyRefunded),
        }
        if deliverer != self.executor {
            return Err(EscrowError::DelivererMismatch);
        }
        if current_slot > self.deadline_slot {
            return Err(EscrowError::DeadlinePassed);
        }

        Ok(EscrowState::Delivered { artifact_hash })
    }

    /// Releases the Job to its executor on a `Pass` journal of the delivery.
    ///
    /// Precondition: a future adapter has already verified the receipt that
    /// produced `journal`. This function does not verify any receipt, seal,
    /// Groth16 proof, Router or CPI.
    ///
    /// Checks run in this order: state (`Delivered` only),
    /// `current_slot <= deadline_slot`, journal binding (schema, Job,
    /// specification, harness, delivered artifact, image), `Verdict::Pass`,
    /// recipient and mint.
    pub fn release(
        &self,
        state: EscrowState,
        journal: &JournalV1,
        current_slot: u64,
        recipient: ExecutorId,
        mint: MintId,
    ) -> Result<Settlement, EscrowError> {
        let artifact_hash = require_delivered(state)?;
        if current_slot > self.deadline_slot {
            return Err(EscrowError::DeadlinePassed);
        }
        if self.bound_verdict(journal, artifact_hash)? != Verdict::Pass {
            return Err(EscrowError::VerdictNotPass);
        }
        if recipient != self.executor {
            return Err(EscrowError::RecipientMismatch);
        }
        if mint != self.mint {
            return Err(EscrowError::MintMismatch);
        }

        Ok(self.settle(
            EscrowState::Released { artifact_hash },
            PayoutRecipient::Executor(self.executor),
        ))
    }

    /// Refunds the Job to its buyer on a `Fail` journal of the delivery, at
    /// any slot.
    ///
    /// Same precondition as [`JobV1::release`]. Checks run in this order:
    /// state (`Delivered` only), journal binding, `Verdict::Fail`, recipient
    /// and mint.
    pub fn refund_on_fail(
        &self,
        state: EscrowState,
        journal: &JournalV1,
        recipient: BuyerId,
        mint: MintId,
    ) -> Result<Settlement, EscrowError> {
        let artifact_hash = require_delivered(state)?;
        if self.bound_verdict(journal, artifact_hash)? != Verdict::Fail {
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
                reason: RefundReason::Fail { artifact_hash },
            },
            PayoutRecipient::Buyer(self.buyer),
        ))
    }

    /// Refunds the Job to its buyer once `current_slot > deadline_slot`,
    /// whether or not the executor delivered.
    ///
    /// Checks run in this order: state (`Funded` or `Delivered`), deadline,
    /// recipient and mint.
    pub fn refund_on_timeout(
        &self,
        state: EscrowState,
        current_slot: u64,
        recipient: BuyerId,
        mint: MintId,
    ) -> Result<Settlement, EscrowError> {
        require_open(state)?;
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

    /// Returns the verdict of a journal bound to this Job and delivery.
    ///
    /// Reuses [`JournalV1::validate_against`], so schema, Job, specification,
    /// harness, the delivered artifact and image are all compared.
    fn bound_verdict(
        &self,
        journal: &JournalV1,
        delivered_artifact_hash: Hash32,
    ) -> Result<Verdict, EscrowError> {
        let expected = JournalV1Commitments::new(
            self.job_id,
            self.spec_hash,
            self.harness_hash,
            delivered_artifact_hash,
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

/// Returns the delivered artifact commitment that a verdict must settle.
fn require_delivered(state: EscrowState) -> Result<Hash32, EscrowError> {
    match state {
        EscrowState::Created => Err(EscrowError::NotFunded),
        EscrowState::Funded => Err(EscrowError::NotDelivered),
        EscrowState::Delivered { artifact_hash } => Ok(artifact_hash),
        EscrowState::Released { .. } => Err(EscrowError::AlreadyReleased),
        EscrowState::Refunded { .. } => Err(EscrowError::AlreadyRefunded),
    }
}

/// Accepts the funded, not yet settled states that a timeout may refund.
fn require_open(state: EscrowState) -> Result<(), EscrowError> {
    match state {
        EscrowState::Created => Err(EscrowError::NotFunded),
        EscrowState::Funded | EscrowState::Delivered { .. } => Ok(()),
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
    use crate::{evaluate_restricted_artifact, hash_restricted_artifact, RestrictedArtifactV1};
    use alloc::collections::BTreeSet;
    use alloc::vec::Vec;
    use core::fmt::Debug;

    const JOB_ID: JobId = JobId::new([0x11; 32]);
    // Development placeholder only; not the ImageID of a built guest.
    const IMAGE_ID: ImageId = ImageId::new([0x55; 32]);
    const BUYER: BuyerId = BuyerId::new([0xb1; 32]);
    const EXECUTOR: ExecutorId = ExecutorId::new([0xe1; 32]);
    const MINT: MintId = MintId::new([0x4d; 32]);
    const OTHER: [u8; 32] = [0x99; 32];
    const AMOUNT_UNITS: u64 = 1_000_000;
    // Large enough that every creation slot of the admission window is valid.
    const DEADLINE: u64 = 2_000_000;
    const SLOTS: [u64; 3] = [DEADLINE - 1, DEADLINE, DEADLINE + 1];

    // Operations of the matrix oracle.
    const RELEASE: u8 = 0;
    const REFUND_ON_FAIL: u8 = 1;
    const REFUND_ON_TIMEOUT: u8 = 2;
    const DELIVER: u8 = 3;

    // Destinations of the single-destination property.
    const TO_EXECUTOR: u8 = 0;
    const TO_BUYER: u8 = 1;

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

    fn terms(spec_hash: Hash32, harness_hash: Hash32, image_id: ImageId, deadline_slot: u64) -> JobV1 {
        require_ok(JobV1::new(
            JOB_ID,
            BUYER,
            EXECUTOR,
            MINT,
            amount(),
            deadline_slot,
            spec_hash,
            harness_hash,
            image_id,
        ))
    }

    fn job() -> JobV1 {
        terms(spec_hash(), harness_hash(), IMAGE_ID, DEADLINE)
    }

    fn journal_for(job_id: JobId, input: u32, claimed_output: u32) -> JournalV1 {
        let bytes = require_ok(RestrictedArtifactV1::new(input, claimed_output).encode_candidate());
        require_ok(evaluate_restricted_artifact(job_id, IMAGE_ID, &bytes))
    }

    fn journal(input: u32, claimed_output: u32) -> JournalV1 {
        journal_for(JOB_ID, input, claimed_output)
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

    /// Funded state in which the executor delivered the artifact of `journal`.
    fn delivered(job: &JobV1, journal: &JournalV1) -> EscrowState {
        require_ok(job.deliver(funded(job), EXECUTOR, journal.artifact_hash(), DEADLINE))
    }

    fn divergent(
        verdict: Verdict,
        job_id: JobId,
        spec_hash: Hash32,
        harness_hash: Hash32,
        artifact_hash: Hash32,
        image_id: ImageId,
    ) -> JournalV1 {
        JournalV1::new(job_id, spec_hash, harness_hash, artifact_hash, image_id, verdict)
    }

    /// Journals that differ from the Job and the delivered Pass artifact in
    /// exactly one commitment.
    fn divergent_journals(verdict: Verdict) -> [(JournalV1, JournalValidationError); 5] {
        let artifact = pass_journal().artifact_hash();
        [
            (
                divergent(verdict, JobId::new(OTHER), spec_hash(), harness_hash(), artifact, IMAGE_ID),
                JournalValidationError::JobIdMismatch,
            ),
            (
                divergent(verdict, JOB_ID, Hash32::new(OTHER), harness_hash(), artifact, IMAGE_ID),
                JournalValidationError::SpecHashMismatch,
            ),
            (
                divergent(verdict, JOB_ID, spec_hash(), Hash32::new(OTHER), artifact, IMAGE_ID),
                JournalValidationError::HarnessHashMismatch,
            ),
            (
                divergent(verdict, JOB_ID, spec_hash(), harness_hash(), Hash32::new(OTHER), IMAGE_ID),
                JournalValidationError::ArtifactHashMismatch,
            ),
            (
                divergent(verdict, JOB_ID, spec_hash(), harness_hash(), artifact, ImageId::new(OTHER)),
                JournalValidationError::ImageIdMismatch,
            ),
        ]
    }

    fn terminal_states(job: &JobV1) -> [(EscrowState, EscrowError); 3] {
        let pass = pass_journal();
        let fail = fail_journal();
        let released = require_ok(job.release(delivered(job, &pass), &pass, DEADLINE, EXECUTOR, MINT));
        let refunded_fail = require_ok(job.refund_on_fail(delivered(job, &fail), &fail, BUYER, MINT));
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
    fn deliver_rejects_other_deliverers_states_and_late_slots() {
        let job = job();
        let hash = pass_journal().artifact_hash();

        for deliverer in [ExecutorId::new(*BUYER.as_bytes()), ExecutorId::new(OTHER)] {
            assert_eq!(
                job.deliver(funded(&job), deliverer, hash, DEADLINE),
                Err(EscrowError::DelivererMismatch)
            );
        }
        assert_eq!(
            job.deliver(EscrowState::Created, EXECUTOR, hash, DEADLINE),
            Err(EscrowError::NotFunded)
        );
        // A delivery is committed once, even for another artifact or slot.
        let once = delivered(&job, &pass_journal());
        for (artifact, slot) in [(hash, DEADLINE), (fail_journal().artifact_hash(), 0)] {
            assert_eq!(
                job.deliver(once, EXECUTOR, artifact, slot),
                Err(EscrowError::AlreadyDelivered)
            );
        }
        for slot in [DEADLINE + 1, u64::MAX] {
            assert_eq!(
                job.deliver(funded(&job), EXECUTOR, hash, slot),
                Err(EscrowError::DeadlinePassed)
            );
        }
    }

    #[test]
    fn deliver_records_the_executor_commitment_up_to_the_deadline_inclusive() {
        let job = job();

        for slot in [0, DEADLINE - 1, DEADLINE] {
            for journal in [pass_journal(), fail_journal()] {
                let state =
                    require_ok(job.deliver(funded(&job), EXECUTOR, journal.artifact_hash(), slot));
                assert_eq!(
                    state,
                    EscrowState::Delivered {
                        artifact_hash: journal.artifact_hash()
                    }
                );
                assert!(!state.is_terminal());
            }
        }
    }

    #[test]
    fn verdict_settlement_requires_a_delivery() {
        let job = job();

        for (state, error) in [
            (EscrowState::Created, EscrowError::NotFunded),
            (funded(&job), EscrowError::NotDelivered),
        ] {
            assert_eq!(
                job.release(state, &pass_journal(), DEADLINE, EXECUTOR, MINT),
                Err(error)
            );
            assert_eq!(
                job.refund_on_fail(state, &fail_journal(), BUYER, MINT),
                Err(error)
            );
        }
        assert_eq!(
            job.refund_on_timeout(EscrowState::Created, DEADLINE + 1, BUYER, MINT),
            Err(EscrowError::NotFunded)
        );
    }

    #[test]
    fn refund_on_fail_rejects_a_fail_of_an_undelivered_artifact() {
        // R-D2 PoC-1 inverted: anyone can produce a bound Fail journal for an
        // arbitrary artifact, but it no longer refunds the buyer.
        let job = job();
        let pass = pass_journal();
        let state = delivered(&job, &pass);

        for fail in [journal(7, 15), journal(7, 0), journal(8, 15), journal(1_000_000, 1)] {
            assert_eq!(fail.verdict(), Verdict::Fail);
            assert_ne!(fail.artifact_hash(), pass.artifact_hash());
            // `refund_on_fail` takes no slot: the rejection holds at every slot.
            assert_eq!(
                job.refund_on_fail(state, &fail, BUYER, MINT),
                Err(EscrowError::Journal(JournalValidationError::ArtifactHashMismatch))
            );
        }
        // In the same state, the release of the delivered Pass stays open in
        // every slot up to the deadline.
        for slot in [0, DEADLINE - 1, DEADLINE] {
            require_ok(job.release(state, &pass, slot, EXECUTOR, MINT));
        }
    }

    #[test]
    fn release_rejects_a_pass_of_an_undelivered_artifact() {
        let job = job();
        let fail = fail_journal();
        let state = delivered(&job, &fail);

        for pass in [journal(7, 14), journal(0, 0), journal(8, 16), journal(1_000_000, 2_000_000)] {
            assert_eq!(pass.verdict(), Verdict::Pass);
            for slot in [0, DEADLINE - 1, DEADLINE] {
                assert_eq!(
                    job.release(state, &pass, slot, EXECUTOR, MINT),
                    Err(EscrowError::Journal(JournalValidationError::ArtifactHashMismatch))
                );
            }
            // After the deadline the release is closed before the journal is
            // read (D2b check order).
            assert_eq!(
                job.release(state, &pass, DEADLINE + 1, EXECUTOR, MINT),
                Err(EscrowError::DeadlinePassed)
            );
        }
        // The Fail of the delivered artifact still refunds the buyer.
        require_ok(job.refund_on_fail(state, &fail, BUYER, MINT));
    }

    #[test]
    fn pass_after_the_deadline_is_rejected() {
        let job = job();
        let pass = pass_journal();

        assert_eq!(
            job.release(delivered(&job, &pass), &pass, DEADLINE + 1, EXECUTOR, MINT),
            Err(EscrowError::DeadlinePassed)
        );
    }

    #[test]
    fn fail_never_releases_and_pass_never_refunds_on_fail() {
        let job = job();
        let pass = pass_journal();
        let fail = fail_journal();

        assert_eq!(
            job.release(delivered(&job, &fail), &fail, DEADLINE, EXECUTOR, MINT),
            Err(EscrowError::VerdictNotPass)
        );
        assert_eq!(
            job.refund_on_fail(delivered(&job, &pass), &pass, BUYER, MINT),
            Err(EscrowError::VerdictNotFail)
        );
    }

    #[test]
    fn release_rejects_each_divergent_commitment() {
        let job = job();
        let state = delivered(&job, &pass_journal());

        for (journal, error) in divergent_journals(Verdict::Pass) {
            assert_eq!(
                job.release(state, &journal, DEADLINE, EXECUTOR, MINT),
                Err(EscrowError::Journal(error))
            );
        }
    }

    #[test]
    fn refund_on_fail_rejects_each_divergent_commitment() {
        let job = job();
        let state = delivered(&job, &pass_journal());

        for (journal, error) in divergent_journals(Verdict::Fail) {
            assert_eq!(
                job.refund_on_fail(state, &journal, BUYER, MINT),
                Err(EscrowError::Journal(error))
            );
        }
    }

    #[test]
    fn rejects_recipients_other_than_the_paid_job_party() {
        let job = job();
        let pass = pass_journal();
        let fail = fail_journal();

        for recipient in [ExecutorId::new(OTHER), ExecutorId::new(*BUYER.as_bytes())] {
            assert_eq!(
                job.release(delivered(&job, &pass), &pass, DEADLINE, recipient, MINT),
                Err(EscrowError::RecipientMismatch)
            );
        }
        for recipient in [BuyerId::new(OTHER), BuyerId::new(*EXECUTOR.as_bytes())] {
            assert_eq!(
                job.refund_on_fail(delivered(&job, &fail), &fail, recipient, MINT),
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
        let pass = pass_journal();
        let fail = fail_journal();

        assert_eq!(
            job.release(delivered(&job, &pass), &pass, DEADLINE, EXECUTOR, other),
            Err(EscrowError::MintMismatch)
        );
        assert_eq!(
            job.refund_on_fail(delivered(&job, &fail), &fail, BUYER, other),
            Err(EscrowError::MintMismatch)
        );
        assert_eq!(
            job.refund_on_timeout(funded(&job), DEADLINE + 1, BUYER, other),
            Err(EscrowError::MintMismatch)
        );
    }

    #[test]
    fn rejects_double_settlement_and_funding_of_a_terminal_job() {
        let job = job();
        let pass = pass_journal();

        for (state, error) in terminal_states(&job) {
            assert!(state.is_terminal());
            assert_eq!(job.fund(state, BUYER, MINT, amount()), Err(error));
            assert_eq!(
                job.deliver(state, EXECUTOR, pass.artifact_hash(), DEADLINE),
                Err(error)
            );
            assert_eq!(
                job.release(state, &pass, DEADLINE, EXECUTOR, MINT),
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
        }
    }

    #[test]
    fn timeout_refunds_the_buyer_only_after_the_deadline() {
        let job = job();
        let open_states = [
            funded(&job),
            delivered(&job, &pass_journal()),
            delivered(&job, &fail_journal()),
        ];

        for state in open_states {
            for slot in [0, DEADLINE - 1, DEADLINE] {
                assert_eq!(
                    job.refund_on_timeout(state, slot, BUYER, MINT),
                    Err(EscrowError::DeadlineNotReached)
                );
            }

            let settlement = require_ok(job.refund_on_timeout(state, DEADLINE + 1, BUYER, MINT));
            assert_eq!(
                settlement.state(),
                EscrowState::Refunded {
                    reason: RefundReason::Timeout
                }
            );
            assert_job_payout(&settlement, PayoutRecipient::Buyer(BUYER));
        }
    }

    #[test]
    fn pass_releases_to_the_executor_up_to_the_deadline_inclusive() {
        let job = job();
        let journal = pass_journal();

        for slot in [0, DEADLINE - 1, DEADLINE] {
            let settlement =
                require_ok(job.release(delivered(&job, &journal), &journal, slot, EXECUTOR, MINT));
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
    fn fail_refunds_the_buyer_before_and_after_the_deadline() {
        let job = job();
        let journal = fail_journal();
        let state = delivered(&job, &journal);

        let settlement = require_ok(job.refund_on_fail(state, &journal, BUYER, MINT));
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
            job.release(state, &journal, DEADLINE + 1, EXECUTOR, MINT),
            Err(EscrowError::DeadlinePassed)
        );
    }

    #[test]
    fn settlement_records_the_delivered_commitment() {
        let job = job();
        let pass = journal(8, 16);
        let fail = journal(8, 17);
        // The delivery commitment has the semantics of the journal artifact hash.
        assert_eq!(
            pass.artifact_hash(),
            require_ok(hash_restricted_artifact(&RestrictedArtifactV1::new(8, 16)))
        );

        let released = require_ok(job.release(delivered(&job, &pass), &pass, DEADLINE, EXECUTOR, MINT));
        assert_eq!(
            released.state(),
            EscrowState::Released {
                artifact_hash: pass.artifact_hash()
            }
        );
        let refunded = require_ok(job.refund_on_fail(delivered(&job, &fail), &fail, BUYER, MINT));
        assert_eq!(
            refunded.state(),
            EscrowState::Refunded {
                reason: RefundReason::Fail {
                    artifact_hash: fail.artifact_hash()
                }
            }
        );
    }

    #[test]
    fn funding_requires_the_exact_buyer_mint_and_amount_once() {
        let job = job();
        let created = EscrowState::Created;

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
        for state in [funded(&job), delivered(&job, &pass_journal())] {
            assert_eq!(
                job.fund(state, BUYER, MINT, amount()),
                Err(EscrowError::AlreadyFunded)
            );
        }
        assert_eq!(job.fund(created, BUYER, MINT, amount()), Ok(EscrowState::Funded));
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
    fn admission_accepts_only_the_v1_terms() {
        let creation = DEADLINE - MIN_DEADLINE_WINDOW_SLOTS;
        let cases = [
            (
                terms(Hash32::new(OTHER), harness_hash(), IMAGE_ID, DEADLINE),
                JobError::SpecNotAdmitted,
            ),
            // The v1 commitments are not interchangeable.
            (
                terms(harness_hash(), spec_hash(), IMAGE_ID, DEADLINE),
                JobError::SpecNotAdmitted,
            ),
            (
                terms(spec_hash(), Hash32::new(OTHER), IMAGE_ID, DEADLINE),
                JobError::HarnessNotAdmitted,
            ),
            (
                terms(spec_hash(), harness_hash(), ImageId::new(OTHER), DEADLINE),
                JobError::ImageIdNotAdmitted,
            ),
            // Terms are checked before the deadline window.
            (
                terms(Hash32::new(OTHER), harness_hash(), IMAGE_ID, 0),
                JobError::SpecNotAdmitted,
            ),
        ];
        for (job, error) in cases {
            assert_eq!(job.admit(IMAGE_ID, creation), Err(error));
        }
        assert_eq!(
            job().admit(ImageId::new(OTHER), creation),
            Err(JobError::ImageIdNotAdmitted)
        );

        assert_eq!(job().admit(IMAGE_ID, creation), Ok(()));
    }

    #[test]
    fn admission_bounds_the_deadline_window() {
        let job = job();
        let out = Err(JobError::DeadlineOutOfWindow);

        // Window edges: creation slot = deadline - window.
        assert_eq!(job.admit(IMAGE_ID, DEADLINE - (MIN_DEADLINE_WINDOW_SLOTS - 1)), out);
        assert_eq!(job.admit(IMAGE_ID, DEADLINE - (MAX_DEADLINE_WINDOW_SLOTS + 1)), out);
        // A deadline at or before the creation slot.
        for slot in [DEADLINE, DEADLINE + 1, u64::MAX] {
            assert_eq!(job.admit(IMAGE_ID, slot), out);
        }
        // R-D2 PoC-5 (deadline 0) and PoC-9 (deadline u64::MAX).
        let zero = terms(spec_hash(), harness_hash(), IMAGE_ID, 0);
        let max = terms(spec_hash(), harness_hash(), IMAGE_ID, u64::MAX);
        for slot in [0, 1, 1_000] {
            assert_eq!(zero.admit(IMAGE_ID, slot), out);
            assert_eq!(max.admit(IMAGE_ID, slot), out);
        }
        // Creation slots near u64::MAX neither overflow nor admit.
        assert_eq!(max.admit(IMAGE_ID, u64::MAX - MIN_DEADLINE_WINDOW_SLOTS + 1), out);
        assert_eq!(max.admit(IMAGE_ID, u64::MAX), out);

        assert_eq!(job.admit(IMAGE_ID, DEADLINE - MIN_DEADLINE_WINDOW_SLOTS), Ok(()));
        assert_eq!(job.admit(IMAGE_ID, DEADLINE - MAX_DEADLINE_WINDOW_SLOTS), Ok(()));
    }

    #[test]
    fn no_input_combination_bypasses_the_policy() {
        let job = job();
        // 0: Pass A, delivered in state 2; 1: Fail B, delivered in state 3;
        // 2: Pass C and 3: Fail D, never delivered; 4: Pass A of another Job.
        let journals = [
            journal(7, 14),
            journal(7, 15),
            journal(8, 16),
            journal(8, 17),
            journal_for(JobId::new(OTHER), 7, 14),
        ];
        let [released, refunded_fail, refunded_timeout] = terminal_states(&job);
        // 0 Created, 1 Funded, 2 Delivered A, 3 Delivered B, 4 Released,
        // 5 Refunded on Fail, 6 Refunded on timeout.
        let states = [
            EscrowState::Created,
            funded(&job),
            delivered(&job, &journals[0]),
            delivered(&job, &journals[1]),
            released.0,
            refunded_fail.0,
            refunded_timeout.0,
        ];
        // 0 executor, 1 buyer, 2 other.
        let parties = [*EXECUTOR.as_bytes(), *BUYER.as_bytes(), OTHER];
        // 0 Job mint, 1 other.
        let mints = [MINT, MintId::new(OTHER)];

        let mut accepted = BTreeSet::new();
        for (s, state) in states.into_iter().enumerate() {
            for (j, journal) in journals.iter().enumerate() {
                for slot in SLOTS {
                    for (p, party) in parties.into_iter().enumerate() {
                        let deliver =
                            job.deliver(state, ExecutorId::new(party), journal.artifact_hash(), slot);
                        if deliver.is_ok() {
                            accepted.insert((DELIVER, s, j, slot, p, 0));
                        }
                        for (m, mint) in mints.into_iter().enumerate() {
                            let release = job.release(state, journal, slot, ExecutorId::new(party), mint);
                            if let Ok(settlement) = release {
                                assert_job_payout(&settlement, PayoutRecipient::Executor(EXECUTOR));
                                accepted.insert((RELEASE, s, j, slot, p, m));
                            }
                            let on_fail = job.refund_on_fail(state, journal, BuyerId::new(party), mint);
                            if let Ok(settlement) = on_fail {
                                assert_job_payout(&settlement, PayoutRecipient::Buyer(BUYER));
                                accepted.insert((REFUND_ON_FAIL, s, j, slot, p, m));
                            }
                            let timeout = job.refund_on_timeout(state, slot, BuyerId::new(party), mint);
                            if let Ok(settlement) = timeout {
                                assert_job_payout(&settlement, PayoutRecipient::Buyer(BUYER));
                                accepted.insert((REFUND_ON_TIMEOUT, s, j, slot, p, m));
                            }
                        }
                    }
                }
            }
        }

        // Independent oracle: every accepted call, listed from the policy
        // rather than recomputed from the checks.
        let (before, at, after) = (DEADLINE - 1, DEADLINE, DEADLINE + 1);
        let mut expected = BTreeSet::new();
        // The Pass of delivered artifact A pays the executor up to the deadline.
        for slot in [before, at] {
            expected.insert((RELEASE, 2, 0, slot, 0, 0));
        }
        // The Fail of delivered artifact B refunds the buyer at any slot.
        for slot in [before, at, after] {
            expected.insert((REFUND_ON_FAIL, 3, 1, slot, 1, 0));
        }
        // After the deadline, a timeout refunds the buyer from Funded or
        // Delivered, whatever journal is at hand.
        for state in [1, 2, 3] {
            for journal in 0..journals.len() {
                expected.insert((REFUND_ON_TIMEOUT, state, journal, after, 1, 0));
            }
        }
        // From Funded, the executor commits any artifact up to the deadline.
        for journal in 0..journals.len() {
            for slot in [before, at] {
                expected.insert((DELIVER, 1, journal, slot, 0, 0));
            }
        }

        assert_eq!(expected.len(), 30);
        assert_eq!(accepted, expected);
    }

    #[test]
    fn at_most_one_destination_per_state_and_slot() {
        let job = job();
        // Journals produced by the deterministic harness stand in for receipts
        // verified against the admitted guest; this core verifies none.
        let mut journals = Vec::new();
        for input in 0..4 {
            for claimed_output in 0..10 {
                journals.push(journal(input, claimed_output));
            }
        }
        journals.push(pass_journal());
        journals.push(fail_journal());
        journals.push(journal_for(JobId::new(OTHER), 7, 14));
        journals.push(journal_for(JobId::new(OTHER), 7, 15));

        let mut states = Vec::new();
        states.push(EscrowState::Created);
        states.push(funded(&job));
        for journal in journals.iter().filter(|journal| journal.job_id() == JOB_ID) {
            states.push(delivered(&job, journal));
        }
        for (state, _) in terminal_states(&job) {
            states.push(state);
        }
        let parties = [*EXECUTOR.as_bytes(), *BUYER.as_bytes(), OTHER];
        let mints = [MINT, MintId::new(OTHER)];

        let destinations = |state: EscrowState, slot: u64| {
            let mut found = BTreeSet::new();
            for journal in &journals {
                for party in parties {
                    for mint in mints {
                        let settlements = [
                            job.release(state, journal, slot, ExecutorId::new(party), mint),
                            job.refund_on_fail(state, journal, BuyerId::new(party), mint),
                            job.refund_on_timeout(state, slot, BuyerId::new(party), mint),
                        ];
                        for settlement in settlements.into_iter().flatten() {
                            found.insert(match settlement.payout().recipient() {
                                PayoutRecipient::Executor(executor) => {
                                    assert_eq!(executor, EXECUTOR);
                                    TO_EXECUTOR
                                }
                                PayoutRecipient::Buyer(buyer) => {
                                    assert_eq!(buyer, BUYER);
                                    TO_BUYER
                                }
                            });
                        }
                    }
                }
            }
            found
        };

        for state in states.iter().copied() {
            for slot in [0, DEADLINE - 1, DEADLINE, DEADLINE + 1, u64::MAX] {
                let found = destinations(state, slot);
                assert!(found.len() <= 1, "{state:?} at slot {slot} pays {found:?}");
            }
        }

        let pass = delivered(&job, &pass_journal());
        let fail = delivered(&job, &fail_journal());
        let only = |destination: u8| BTreeSet::from([destination]);
        assert_eq!(destinations(pass, DEADLINE), only(TO_EXECUTOR));
        assert_eq!(destinations(fail, DEADLINE), only(TO_BUYER));
        assert_eq!(destinations(pass, DEADLINE + 1), only(TO_BUYER));
        assert_eq!(destinations(funded(&job), DEADLINE), BTreeSet::new());
        assert_eq!(destinations(funded(&job), DEADLINE + 1), only(TO_BUYER));
    }
}
