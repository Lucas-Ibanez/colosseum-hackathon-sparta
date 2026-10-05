//! VeriCode escrow program (gates D2c, D2b.1, D2e, D4a).
//!
//! The program persists immutable Job terms, custodies the Job mint in a
//! vault owned by the Job PDA and settles only through the pure policy in
//! `vericode-core`. Economic checks (identities, admitted v1 terms, deadline
//! window, delivery, journal binding, verdict, mint, amount and state) are
//! delegated to the core. Anchor constraints and the program check account
//! structure (PDAs, owners, signers, account types, the Job mint address and
//! the fixed verifier program), the canonical associated token account of the
//! paid party, and three custody preconditions on Solana accounts: the
//! admitted Test USDC mint, a mint without freeze authority and an executor
//! that is not a program account of the Job.
//!
//! `release` and `refund_on_fail` settle only after the RISC Zero Groth16
//! verifier (immutable on devnet) accepted, by CPI in the same instruction, a
//! seal for the exact journal bytes and the Job image ID. There is no
//! Verifier Router in the path (D4a) and no administrative instruction.

use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hash as sha256;
use anchor_lang::solana_program::instruction::{AccountMeta, Instruction};
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::{self, Mint, Token, TokenAccount, TransferChecked};
use vericode_core::escrow::{
    Amount, AmountError, BuyerId, EscrowError, EscrowState, ExecutorId, JobError, JobParty,
    JobV1, MintId, PayoutRecipient, RefundReason, Settlement,
};
use vericode_core::{Hash32, ImageId, JobId, JournalV1, JournalValidationError, Verdict};

declare_id!("GZqbL2TbeDVHcNRosngaRfCwzV9YJT6iEbckYr8uwkCH");

/// Seed prefix of the Job PDA: `["job", job_id]`.
pub const JOB_SEED: &[u8] = b"job";
/// Seed prefix of the vault PDA: `["vault", job]`.
pub const VAULT_SEED: &[u8] = b"vault";
/// Layout version of [`JobAccount`].
pub const JOB_ACCOUNT_VERSION: u8 = 1;
/// ImageID of the deterministic D1c2b guest, the only guest a v1 Job admits:
/// `4da06f90da75ec8980c943ce017d69c48370fddbf3aa27689d375d78fac0fb1a`.
///
/// A rebuild of the guest must be recertified before this value changes.
pub const ADMITTED_IMAGE_ID_V1: [u8; 32] = [
    0x4d, 0xa0, 0x6f, 0x90, 0xda, 0x75, 0xec, 0x89,
    0x80, 0xc9, 0x43, 0xce, 0x01, 0x7d, 0x69, 0xc4,
    0x83, 0x70, 0xfd, 0xdb, 0xf3, 0xaa, 0x27, 0x68,
    0x9d, 0x37, 0x5d, 0x78, 0xfa, 0xc0, 0xfb, 0x1a,
];

/// RISC Zero Groth16 verifier program of `risc0-solana v3.0.0` (commit
/// `ee415935`). Only this program may verify a settlement proof. On devnet it
/// is deployed with upgrade authority `None`, so nobody can change, stop or
/// close it (D4a).
pub const GROTH16_VERIFIER_ID: Pubkey =
    Pubkey::from_str_const("THq1qFYQoh7zgcjXoMXduDBqiZRCPeg3PvvMbrVQUge");
/// Selector of the Groth16 verifier parameters: the first bytes of the digest
/// of `Groth16ReceiptVerifierParameters::default()` in `risc0-zkvm 3.0.3`, the
/// parameters [`GROTH16_VERIFIER_ID`] verifies against.
pub const GROTH16_SELECTOR: [u8; 4] = [0x73, 0xc4, 0x57, 0xba];
/// Anchor discriminator of the verifier `verify` instruction.
pub const VERIFY_DISCRIMINATOR: [u8; 8] = [0x85, 0xa1, 0x8d, 0x30, 0x78, 0xc6, 0x58, 0x96];
/// The only mint a Job admits: the devnet Test USDC of the VeriCode MVP
/// (6 decimals, no freeze authority), R-D2 F-05.
pub const ADMITTED_MINT: Pubkey =
    Pubkey::from_str_const("9TE2VPFmgrNxT22yS3sEZyRcMxLgJkwzgAoquWRXwV2F");
/// Associated Token Account program, which fixes the canonical destination
/// of every settlement.
pub const ATA_PROGRAM_ID: Pubkey =
    Pubkey::from_str_const("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");

/// Groth16 seal in the Borsh layout of the upstream `Seal`: the selector,
/// then the proof points. `pi_a` is already negated, as the verifier expects.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub struct Groth16Seal {
    pub selector: [u8; 4],
    pub pi_a: [u8; 64],
    pub pi_b: [u8; 128],
    pub pi_c: [u8; 64],
}

#[program]
pub mod vericode_escrow {
    use super::*;

    /// Creates the Job PDA with immutable terms and an empty vault owned by it.
    ///
    /// Only the admitted v1 specification, harness and guest image are
    /// accepted, with a deadline inside the creation window of the core.
    pub fn create_job(
        ctx: Context<CreateJob>,
        job_id: [u8; 32],
        executor: Pubkey,
        amount: u64,
        deadline_slot: u64,
        spec_hash: [u8; 32],
        harness_hash: [u8; 32],
        image_id: [u8; 32],
    ) -> Result<()> {
        let amount = Amount::new(amount).map_err(amount_error)?;
        let terms = JobV1::new(
            JobId::new(job_id),
            BuyerId::new(ctx.accounts.buyer.key().to_bytes()),
            ExecutorId::new(executor.to_bytes()),
            MintId::new(ctx.accounts.mint.key().to_bytes()),
            amount,
            deadline_slot,
            Hash32::new(spec_hash),
            Hash32::new(harness_hash),
            ImageId::new(image_id),
        )
        .map_err(job_error)?;
        // The Job PDA can only sign through this program and the vault cannot
        // sign at all: such an executor could never deliver nor be paid.
        require!(
            executor != ctx.accounts.job.key() && executor != ctx.accounts.vault.key(),
            VericodeEscrowError::ExecutorIsProgramAccount
        );
        terms
            .admit(ImageId::new(ADMITTED_IMAGE_ID_V1), Clock::get()?.slot)
            .map_err(job_error)?;

        let job = &mut ctx.accounts.job;
        job.version = JOB_ACCOUNT_VERSION;
        job.bump = ctx.bumps.job;
        job.vault_bump = ctx.bumps.vault;
        job.job_id = job_id;
        job.buyer = ctx.accounts.buyer.key();
        job.executor = executor;
        job.mint = ctx.accounts.mint.key();
        job.amount = terms.amount().base_units();
        job.deadline_slot = terms.deadline_slot();
        job.spec_hash = spec_hash;
        job.harness_hash = harness_hash;
        job.image_id = image_id;
        job.status = EscrowStatus::from_core(EscrowState::Created);
        Ok(())
    }

    /// Moves exactly the Job amount from the buyer into the vault.
    ///
    /// `amount` is the amount the buyer intends to deposit; the core rejects
    /// it unless it equals the Job amount.
    pub fn fund(ctx: Context<Fund>, amount: u64) -> Result<()> {
        let job = &ctx.accounts.job;
        let terms = job.terms()?;
        let deposit = Amount::new(amount).map_err(amount_error)?;
        let next = terms
            .fund(
                job.status.to_core(),
                BuyerId::new(ctx.accounts.buyer.key().to_bytes()),
                MintId::new(ctx.accounts.mint.key().to_bytes()),
                deposit,
            )
            .map_err(escrow_error)?;

        token::transfer_checked(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.buyer_token.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.vault.to_account_info(),
                    authority: ctx.accounts.buyer.to_account_info(),
                },
            ),
            terms.amount().base_units(),
            ctx.accounts.mint.decimals,
        )?;

        ctx.accounts.job.status = EscrowStatus::from_core(next);
        Ok(())
    }

    /// Records, once and up to the deadline, the executor's commitment to the
    /// delivered artifact.
    ///
    /// Only the Job executor may sign it. `artifact_hash` has the semantics
    /// of `vericode_core::hash_restricted_artifact`, the same as the journal
    /// `artifact_hash`; no token moves.
    pub fn deliver(ctx: Context<Deliver>, artifact_hash: [u8; 32]) -> Result<()> {
        let job = &ctx.accounts.job;
        let terms = job.terms()?;
        let next = terms
            .deliver(
                job.status.to_core(),
                ExecutorId::new(ctx.accounts.executor.key().to_bytes()),
                Hash32::new(artifact_hash),
                Clock::get()?.slot,
            )
            .map_err(escrow_error)?;

        ctx.accounts.job.status = EscrowStatus::from_core(next);
        Ok(())
    }

    /// Releases the Job to the executor on a verified `Pass` journal of the
    /// delivered artifact, up to the deadline.
    ///
    /// Permissionless: the destination is the canonical associated token
    /// account of the Job executor, and the proof is verified by CPI to the
    /// fixed Groth16 verifier before any transfer.
    pub fn release(ctx: Context<SettleWithProof>, journal: Vec<u8>, seal: Groth16Seal) -> Result<()> {
        settle_with_proof(ctx, Verdict::Pass, &journal, &seal)
    }

    /// Refunds the buyer on a verified `Fail` journal of the delivered
    /// artifact, at any slot.
    ///
    /// Permissionless, with the same checks as `release`; the destination is
    /// the canonical associated token account of the Job buyer.
    pub fn refund_on_fail(
        ctx: Context<SettleWithProof>,
        journal: Vec<u8>,
        seal: Groth16Seal,
    ) -> Result<()> {
        settle_with_proof(ctx, Verdict::Fail, &journal, &seal)
    }

    /// Refunds the buyer once the current slot is past the deadline, whether
    /// or not the executor delivered.
    ///
    /// Permissionless: any fee payer may crank it. The destination must be
    /// the canonical associated token account of the Job buyer for the Job
    /// mint.
    pub fn refund_on_timeout(ctx: Context<RefundOnTimeout>) -> Result<()> {
        let job = &ctx.accounts.job;
        let terms = job.terms()?;
        let settlement = terms
            .refund_on_timeout(
                job.status.to_core(),
                Clock::get()?.slot,
                BuyerId::new(ctx.accounts.buyer_token.owner.to_bytes()),
                MintId::new(ctx.accounts.buyer_token.mint.to_bytes()),
            )
            .map_err(escrow_error)?;
        require_canonical_destination(&ctx.accounts.buyer_token.key(), &settlement, &job.mint)?;

        transfer_from_vault(
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.vault.to_account_info(),
            &ctx.accounts.mint,
            ctx.accounts.buyer_token.to_account_info(),
            &ctx.accounts.job,
            settlement.payout().amount().base_units(),
        )?;

        ctx.accounts.job.status = EscrowStatus::from_core(settlement.state());
        Ok(())
    }
}

/// Settles by verdict, in this order: journal decoding, core policy (state,
/// deadline, binding to the Job and the delivered artifact, verdict,
/// recipient and mint), canonical destination, seal selector, Groth16
/// verification of `SHA-256(journal)` against the Job image ID, transfer and
/// terminal state. Any failure reverts the whole instruction.
fn settle_with_proof(
    ctx: Context<SettleWithProof>,
    verdict: Verdict,
    journal: &[u8],
    seal: &Groth16Seal,
) -> Result<()> {
    let accounts = &ctx.accounts;
    let job = &accounts.job;
    let terms = job.terms()?;
    let decoded =
        JournalV1::decode_candidate(journal).map_err(|_| error!(VericodeEscrowError::JournalMalformed))?;
    let recipient_mint = MintId::new(accounts.recipient_token.mint.to_bytes());
    let recipient_owner = accounts.recipient_token.owner.to_bytes();
    let settlement = match verdict {
        Verdict::Pass => terms.release(
            job.status.to_core(),
            &decoded,
            Clock::get()?.slot,
            ExecutorId::new(recipient_owner),
            recipient_mint,
        ),
        Verdict::Fail => terms.refund_on_fail(
            job.status.to_core(),
            &decoded,
            BuyerId::new(recipient_owner),
            recipient_mint,
        ),
    }
    .map_err(escrow_error)?;
    require_canonical_destination(&accounts.recipient_token.key(), &settlement, &job.mint)?;
    require!(
        seal.selector == GROTH16_SELECTOR,
        VericodeEscrowError::UnexpectedSelector
    );
    // The digest covers exactly the bytes the core decoded, and the image ID
    // comes from the Job, never from the caller or the journal.
    verify_groth16(accounts, seal, job.image_id, sha256(journal).to_bytes())?;

    transfer_from_vault(
        accounts.token_program.to_account_info(),
        accounts.vault.to_account_info(),
        &accounts.mint,
        accounts.recipient_token.to_account_info(),
        &accounts.job,
        settlement.payout().amount().base_units(),
    )?;

    ctx.accounts.job.status = EscrowStatus::from_core(settlement.state());
    Ok(())
}

/// Requires the canonical associated token account of the party paid by
/// `settlement` for the Job mint (R-D2 F-06), so a caller cannot choose
/// another account of that party.
fn require_canonical_destination(destination: &Pubkey, settlement: &Settlement, mint: &Pubkey) -> Result<()> {
    let party = match settlement.payout().recipient() {
        PayoutRecipient::Executor(executor) => Pubkey::new_from_array(*executor.as_bytes()),
        PayoutRecipient::Buyer(buyer) => Pubkey::new_from_array(*buyer.as_bytes()),
    };
    let (canonical, _) = Pubkey::find_program_address(
        &[party.as_ref(), token::ID.as_ref(), mint.as_ref()],
        &ATA_PROGRAM_ID,
    );
    require_keys_eq!(
        *destination,
        canonical,
        VericodeEscrowError::DestinationNotCanonical
    );
    Ok(())
}

/// Calls the verifier `verify(proof, image_id, journal_digest)` by CPI. The
/// verifier fails, and with it this instruction, unless the Groth16 proof is
/// valid for the claim of `image_id` and `journal_digest`. The selector was
/// checked by the caller and is not part of the verifier arguments.
fn verify_groth16(
    accounts: &SettleWithProof,
    seal: &Groth16Seal,
    image_id: [u8; 32],
    journal_digest: [u8; 32],
) -> Result<()> {
    let mut data = Vec::with_capacity(8 + 64 + 128 + 64 + 32 + 32);
    data.extend_from_slice(&VERIFY_DISCRIMINATOR);
    data.extend_from_slice(&seal.pi_a);
    data.extend_from_slice(&seal.pi_b);
    data.extend_from_slice(&seal.pi_c);
    data.extend_from_slice(&image_id);
    data.extend_from_slice(&journal_digest);
    let instruction = Instruction {
        program_id: GROTH16_VERIFIER_ID,
        accounts: vec![AccountMeta::new_readonly(accounts.system_program.key(), false)],
        data,
    };
    invoke(
        &instruction,
        &[
            accounts.system_program.to_account_info(),
            accounts.verifier_program.to_account_info(),
        ],
    )?;
    Ok(())
}

/// Moves `amount` of the Job mint from the vault to `destination`, signed by
/// the Job PDA.
fn transfer_from_vault<'info>(
    token_program: AccountInfo<'info>,
    vault: AccountInfo<'info>,
    mint: &Account<'info, Mint>,
    destination: AccountInfo<'info>,
    job: &Account<'info, JobAccount>,
    amount: u64,
) -> Result<()> {
    let job_id = job.job_id;
    let bump = [job.bump];
    let signer_seeds: &[&[u8]] = &[JOB_SEED, &job_id, &bump];
    token::transfer_checked(
        CpiContext::new_with_signer(
            token_program,
            TransferChecked {
                from: vault,
                mint: mint.to_account_info(),
                to: destination,
                authority: job.to_account_info(),
            },
            &[signer_seeds],
        ),
        amount,
        mint.decimals,
    )
}

#[derive(Accounts)]
#[instruction(job_id: [u8; 32])]
pub struct CreateJob<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    /// Only the admitted Test USDC mint. A mint without freeze authority can
    /// never gain one (SPL Token `MintCannotFreeze`), so the vault can never
    /// be frozen.
    #[account(
        constraint = mint.freeze_authority.is_none() @ VericodeEscrowError::MintHasFreezeAuthority,
        address = ADMITTED_MINT @ VericodeEscrowError::MintNotAdmitted
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        payer = buyer,
        space = 8 + JobAccount::INIT_SPACE,
        seeds = [JOB_SEED, job_id.as_ref()],
        bump
    )]
    pub job: Account<'info, JobAccount>,
    #[account(
        init,
        payer = buyer,
        seeds = [VAULT_SEED, job.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = job
    )]
    pub vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Fund<'info> {
    pub buyer: Signer<'info>,
    #[account(mut, seeds = [JOB_SEED, job.job_id.as_ref()], bump = job.bump)]
    pub job: Account<'info, JobAccount>,
    #[account(address = job.mint @ VericodeEscrowError::MintMismatch)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub buyer_token: Account<'info, TokenAccount>,
    #[account(mut, seeds = [VAULT_SEED, job.key().as_ref()], bump = job.vault_bump)]
    pub vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Deliver<'info> {
    pub executor: Signer<'info>,
    #[account(mut, seeds = [JOB_SEED, job.job_id.as_ref()], bump = job.bump)]
    pub job: Account<'info, JobAccount>,
}

#[derive(Accounts)]
pub struct SettleWithProof<'info> {
    #[account(mut, seeds = [JOB_SEED, job.job_id.as_ref()], bump = job.bump)]
    pub job: Box<Account<'info, JobAccount>>,
    #[account(address = job.mint @ VericodeEscrowError::MintMismatch)]
    pub mint: Box<Account<'info, Mint>>,
    #[account(mut, seeds = [VAULT_SEED, job.key().as_ref()], bump = job.vault_bump)]
    pub vault: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub recipient_token: Box<Account<'info, TokenAccount>>,
    pub token_program: Program<'info, Token>,
    /// CHECK: fixed Groth16 verifier program, the target of the CPI.
    #[account(address = GROTH16_VERIFIER_ID)]
    pub verifier_program: UncheckedAccount<'info>,
    /// The only account of the verifier `verify` instruction.
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RefundOnTimeout<'info> {
    #[account(mut, seeds = [JOB_SEED, job.job_id.as_ref()], bump = job.bump)]
    pub job: Account<'info, JobAccount>,
    #[account(address = job.mint @ VericodeEscrowError::MintMismatch)]
    pub mint: Account<'info, Mint>,
    #[account(mut, seeds = [VAULT_SEED, job.key().as_ref()], bump = job.vault_bump)]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub buyer_token: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

/// Persisted Job terms and escrow status.
#[account]
#[derive(InitSpace)]
pub struct JobAccount {
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
    pub status: EscrowStatus,
}

impl JobAccount {
    /// Rebuilds the pure core terms from the persisted fields.
    pub fn terms(&self) -> Result<JobV1> {
        require!(
            self.version == JOB_ACCOUNT_VERSION,
            VericodeEscrowError::UnsupportedAccountVersion
        );
        let amount = Amount::new(self.amount).map_err(amount_error)?;
        JobV1::new(
            JobId::new(self.job_id),
            BuyerId::new(self.buyer.to_bytes()),
            ExecutorId::new(self.executor.to_bytes()),
            MintId::new(self.mint.to_bytes()),
            amount,
            self.deadline_slot,
            Hash32::new(self.spec_hash),
            Hash32::new(self.harness_hash),
            ImageId::new(self.image_id),
        )
        .map_err(job_error)
    }
}

/// On-chain encoding of the core [`EscrowState`].
///
/// The Borsh tag of each variant is part of the account layout: new
/// variants are only ever appended.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Eq, PartialEq, InitSpace)]
pub enum EscrowStatus {
    Created,
    Funded,
    Released { artifact_hash: [u8; 32] },
    RefundedOnFail { artifact_hash: [u8; 32] },
    RefundedOnTimeout,
    Delivered { artifact_hash: [u8; 32] },
}

impl EscrowStatus {
    /// Encodes a core state.
    pub fn from_core(state: EscrowState) -> Self {
        match state {
            EscrowState::Created => Self::Created,
            EscrowState::Funded => Self::Funded,
            EscrowState::Delivered { artifact_hash } => Self::Delivered {
                artifact_hash: artifact_hash.into_bytes(),
            },
            EscrowState::Released { artifact_hash } => Self::Released {
                artifact_hash: artifact_hash.into_bytes(),
            },
            EscrowState::Refunded {
                reason: RefundReason::Fail { artifact_hash },
            } => Self::RefundedOnFail {
                artifact_hash: artifact_hash.into_bytes(),
            },
            EscrowState::Refunded {
                reason: RefundReason::Timeout,
            } => Self::RefundedOnTimeout,
        }
    }

    /// Decodes into the core state.
    pub fn to_core(self) -> EscrowState {
        match self {
            Self::Created => EscrowState::Created,
            Self::Funded => EscrowState::Funded,
            Self::Released { artifact_hash } => EscrowState::Released {
                artifact_hash: Hash32::new(artifact_hash),
            },
            Self::RefundedOnFail { artifact_hash } => EscrowState::Refunded {
                reason: RefundReason::Fail {
                    artifact_hash: Hash32::new(artifact_hash),
                },
            },
            Self::RefundedOnTimeout => EscrowState::Refunded {
                reason: RefundReason::Timeout,
            },
            Self::Delivered { artifact_hash } => EscrowState::Delivered {
                artifact_hash: Hash32::new(artifact_hash),
            },
        }
    }
}

/// Program errors; each core rejection maps to exactly one variant.
///
/// Codes are `6000 + index` and part of the public interface: new variants
/// are only ever appended.
#[error_code]
pub enum VericodeEscrowError {
    #[msg("Escrow amount must be greater than zero")]
    AmountZero,
    #[msg("Buyer identity is all zero bytes")]
    ZeroBuyer,
    #[msg("Executor identity is all zero bytes")]
    ZeroExecutor,
    #[msg("Mint identity is all zero bytes")]
    ZeroMint,
    #[msg("Buyer and executor must be different identities")]
    BuyerIsExecutor,
    #[msg("Job is not funded")]
    NotFunded,
    #[msg("Job is already funded")]
    AlreadyFunded,
    #[msg("Job was already released")]
    AlreadyReleased,
    #[msg("Job was already refunded")]
    AlreadyRefunded,
    #[msg("Depositor is not the Job buyer")]
    DepositorMismatch,
    #[msg("Recipient is not the Job party paid by this settlement")]
    RecipientMismatch,
    #[msg("Mint is not the Job mint")]
    MintMismatch,
    #[msg("Amount is not the Job amount")]
    AmountMismatch,
    #[msg("Journal schema version does not match")]
    JournalSchemaVersionMismatch,
    #[msg("Journal belongs to another Job")]
    JournalJobIdMismatch,
    #[msg("Journal specification hash does not match")]
    JournalSpecHashMismatch,
    #[msg("Journal harness hash does not match")]
    JournalHarnessHashMismatch,
    #[msg("Journal artifact hash does not match")]
    JournalArtifactHashMismatch,
    #[msg("Journal image ID does not match")]
    JournalImageIdMismatch,
    #[msg("Journal verdict is not Pass")]
    VerdictNotPass,
    #[msg("Journal verdict is not Fail")]
    VerdictNotFail,
    #[msg("Deadline slot has not passed")]
    DeadlineNotReached,
    #[msg("Deadline slot has passed")]
    DeadlinePassed,
    #[msg("Unsupported Job account version")]
    UnsupportedAccountVersion,
    #[msg("Mint has a freeze authority and could freeze the vault")]
    MintHasFreezeAuthority,
    #[msg("Job has no delivery to settle")]
    NotDelivered,
    #[msg("Executor already delivered")]
    AlreadyDelivered,
    #[msg("Deliverer is not the Job executor")]
    DelivererMismatch,
    #[msg("Specification hash is not the admitted v1 specification")]
    SpecNotAdmitted,
    #[msg("Harness hash is not the admitted v1 harness")]
    HarnessNotAdmitted,
    #[msg("Image ID is not the admitted v1 guest")]
    ImageIdNotAdmitted,
    #[msg("Deadline slot is outside the creation window")]
    DeadlineOutOfWindow,
    #[msg("Executor is a program account of the Job")]
    ExecutorIsProgramAccount,
    #[msg("Seal selector is not the fixed Groth16 selector")]
    UnexpectedSelector,
    #[msg("Journal is not a JournalV1 of the candidate wire format")]
    JournalMalformed,
    #[msg("Destination is not the canonical token account of the paid party")]
    DestinationNotCanonical,
    #[msg("Mint is not the admitted Test USDC mint")]
    MintNotAdmitted,
}

fn amount_error(error: AmountError) -> Error {
    match error {
        AmountError::Zero => VericodeEscrowError::AmountZero.into(),
    }
}

fn job_error(error: JobError) -> Error {
    match error {
        JobError::ZeroIdentity(JobParty::Buyer) => VericodeEscrowError::ZeroBuyer.into(),
        JobError::ZeroIdentity(JobParty::Executor) => VericodeEscrowError::ZeroExecutor.into(),
        JobError::ZeroIdentity(JobParty::Mint) => VericodeEscrowError::ZeroMint.into(),
        JobError::BuyerIsExecutor => VericodeEscrowError::BuyerIsExecutor.into(),
        JobError::SpecNotAdmitted => VericodeEscrowError::SpecNotAdmitted.into(),
        JobError::HarnessNotAdmitted => VericodeEscrowError::HarnessNotAdmitted.into(),
        JobError::ImageIdNotAdmitted => VericodeEscrowError::ImageIdNotAdmitted.into(),
        JobError::DeadlineOutOfWindow => VericodeEscrowError::DeadlineOutOfWindow.into(),
    }
}

fn escrow_error(error: EscrowError) -> Error {
    let code = match error {
        EscrowError::NotFunded => VericodeEscrowError::NotFunded,
        EscrowError::AlreadyFunded => VericodeEscrowError::AlreadyFunded,
        EscrowError::AlreadyReleased => VericodeEscrowError::AlreadyReleased,
        EscrowError::AlreadyRefunded => VericodeEscrowError::AlreadyRefunded,
        EscrowError::DepositorMismatch => VericodeEscrowError::DepositorMismatch,
        EscrowError::RecipientMismatch => VericodeEscrowError::RecipientMismatch,
        EscrowError::MintMismatch => VericodeEscrowError::MintMismatch,
        EscrowError::AmountMismatch => VericodeEscrowError::AmountMismatch,
        EscrowError::Journal(journal) => match journal {
            JournalValidationError::SchemaVersionMismatch => {
                VericodeEscrowError::JournalSchemaVersionMismatch
            }
            JournalValidationError::JobIdMismatch => VericodeEscrowError::JournalJobIdMismatch,
            JournalValidationError::SpecHashMismatch => {
                VericodeEscrowError::JournalSpecHashMismatch
            }
            JournalValidationError::HarnessHashMismatch => {
                VericodeEscrowError::JournalHarnessHashMismatch
            }
            JournalValidationError::ArtifactHashMismatch => {
                VericodeEscrowError::JournalArtifactHashMismatch
            }
            JournalValidationError::ImageIdMismatch => VericodeEscrowError::JournalImageIdMismatch,
        },
        EscrowError::VerdictNotPass => VericodeEscrowError::VerdictNotPass,
        EscrowError::VerdictNotFail => VericodeEscrowError::VerdictNotFail,
        EscrowError::DeadlineNotReached => VericodeEscrowError::DeadlineNotReached,
        EscrowError::DeadlinePassed => VericodeEscrowError::DeadlinePassed,
        EscrowError::NotDelivered => VericodeEscrowError::NotDelivered,
        EscrowError::AlreadyDelivered => VericodeEscrowError::AlreadyDelivered,
        EscrowError::DelivererMismatch => VericodeEscrowError::DelivererMismatch,
    };
    code.into()
}
