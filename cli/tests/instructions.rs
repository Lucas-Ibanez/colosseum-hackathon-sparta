//! Offline checks of the devnet client against the program and the suite.
//!
//! Every instruction the client builds must have exactly the bytes (program
//! ID, account metas and data) of the Anchor-based builders that
//! `anchor/tests-local` runs against the real program and verifier. Nothing
//! here uses a network, a cluster or a keypair file.

#[path = "../../anchor/tests-local/tests/common/mod.rs"]
mod common;

use std::{collections::HashMap, fs, path::PathBuf};

use anchor_lang::{AccountSerialize, Discriminator, Space};
use common::*;
use solana_program_test::ProgramTest;
use spl_token::solana_program::program_option::COption;
use vericode_cli::{escrow as cli, keys, receipt::Receipt, tx};

const BUYER: Pubkey = Pubkey::new_from_array([0xb1; 32]);
const EXECUTOR: Pubkey = Pubkey::new_from_array([0xe1; 32]);
const DEVNET_JOB: [u8; 32] = [0x42; 32];
const DEADLINE_SLOT: u64 = 507_900_000;

/// A fixture of `anchor/tests-local/fixtures/groth16`, read like `fixture()`.
fn suite_fixture(name: &str) -> Fixture {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../anchor/tests-local/fixtures/groth16")
        .join(format!("{name}.txt"));
    let fields: HashMap<String, Vec<u8>> = fs::read_to_string(&path)
        .unwrap()
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
        .map(|line| {
            let (key, value) = line.split_once('=').unwrap();
            (key.to_string(), decode_hex(value.trim()))
        })
        .collect();
    Fixture {
        selector: fields["selector"].clone().try_into().unwrap(),
        image_id: fields["image_id"].clone().try_into().unwrap(),
        journal: fields["journal"].clone(),
        journal_digest: fields["journal_digest"].clone().try_into().unwrap(),
        seal: fields["seal"].clone().try_into().unwrap(),
    }
}

fn cli_seal(fixture: &Fixture) -> cli::Groth16Seal {
    cli::Groth16Seal::from_raw(fixture.selector, &fixture.seal)
}

#[test]
fn constants_are_the_program_constants() {
    assert_eq!(cli::PROGRAM_ID, vericode_escrow::ID);
    assert_eq!(cli::VERIFIER_ID, GROTH16_VERIFIER_ID);
    assert_eq!(cli::ADMITTED_MINT, ADMITTED_MINT);
    assert_eq!(cli::ATA_PROGRAM_ID, ATA_PROGRAM_ID);
    assert_eq!(cli::TOKEN_PROGRAM_ID, spl_token::ID);
    assert_eq!(cli::SYSTEM_PROGRAM_ID, system_program::ID);
    assert_eq!(cli::GROTH16_SELECTOR, GROTH16_SELECTOR);
    assert_eq!(cli::ADMITTED_IMAGE_ID_V1, vericode_escrow::ADMITTED_IMAGE_ID_V1);
    assert_eq!(vericode_cli::hex(&cli::ADMITTED_IMAGE_ID_V1), IMAGE_ID_HEX);
    assert_eq!(cli::JOB_SEED, JOB_SEED);
    assert_eq!(cli::VAULT_SEED, VAULT_SEED);
    assert_eq!(cli::JOB_ACCOUNT_LEN, 8 + JobAccount::INIT_SPACE);
    assert_eq!(cli::MINT_DECIMALS, DECIMALS);
    assert_eq!(cli::MIN_DEADLINE_WINDOW_SLOTS, MIN_WINDOW);
    assert_eq!(cli::MAX_DEADLINE_WINDOW_SLOTS, MAX_WINDOW);
    let terms = cli::admitted_terms();
    let suite = admitted();
    assert_eq!(
        (terms.spec_hash, terms.harness_hash, terms.image_id),
        (suite.spec_hash, suite.harness_hash, suite.image_id)
    );
}

#[test]
fn discriminators_are_the_anchor_discriminators() {
    let cases: [(&str, [u8; 8], &[u8]); 6] = [
        ("create_job", cli::CREATE_JOB_DISCRIMINATOR, instruction::CreateJob::DISCRIMINATOR),
        ("fund", cli::FUND_DISCRIMINATOR, instruction::Fund::DISCRIMINATOR),
        ("deliver", cli::DELIVER_DISCRIMINATOR, instruction::Deliver::DISCRIMINATOR),
        ("release", cli::RELEASE_DISCRIMINATOR, instruction::Release::DISCRIMINATOR),
        ("refund_on_fail", cli::REFUND_ON_FAIL_DISCRIMINATOR, instruction::RefundOnFail::DISCRIMINATOR),
        ("refund_on_timeout", cli::REFUND_ON_TIMEOUT_DISCRIMINATOR, instruction::RefundOnTimeout::DISCRIMINATOR),
    ];
    for (name, client, anchor) in cases {
        assert_eq!(client.as_slice(), anchor, "{name}");
        assert_eq!(client, sha256(format!("global:{name}").as_bytes()).to_bytes()[..8], "{name}");
    }
    assert_eq!(cli::JOB_ACCOUNT_DISCRIMINATOR.as_slice(), JobAccount::DISCRIMINATOR);
    assert_eq!(cli::JOB_ACCOUNT_DISCRIMINATOR, sha256(b"account:JobAccount").to_bytes()[..8]);
}

#[test]
fn addresses_are_the_suite_addresses() {
    for job_id in [DEVNET_JOB, JOB_ID, [0x00; 32], [0xff; 32]] {
        let job = job_pda(&job_id);
        assert_eq!(cli::job_pda(&job_id), job);
        assert_eq!(cli::vault_pda(&job), vault_pda(&job));
    }
    for owner in [BUYER, EXECUTOR, Pubkey::new_unique()] {
        assert_eq!(cli::ata(&owner), ata_address(&owner, &ADMITTED_MINT));
    }
}

#[test]
fn every_instruction_has_the_bytes_of_the_suite_builder() {
    let terms = cli::admitted_terms();
    let buyer_token = ata_address(&BUYER, &ADMITTED_MINT);
    let executor_token = ata_address(&EXECUTOR, &ADMITTED_MINT);
    let pass = suite_fixture("pass");
    let fail = suite_fixture("fail");
    let delivered = pass.artifact_hash();

    assert_eq!(
        cli::create_job_ix(&BUYER, DEVNET_JOB, &EXECUTOR, AMOUNT, DEADLINE_SLOT, &terms),
        create_job_ix(&BUYER, &ADMITTED_MINT, DEVNET_JOB, EXECUTOR, AMOUNT, DEADLINE_SLOT)
    );
    assert_eq!(
        cli::fund_ix(&BUYER, DEVNET_JOB, AMOUNT),
        fund_ix(&BUYER, DEVNET_JOB, &ADMITTED_MINT, &buyer_token, AMOUNT)
    );
    assert_eq!(
        cli::deliver_ix(&EXECUTOR, DEVNET_JOB, delivered),
        deliver_ix(&EXECUTOR, DEVNET_JOB, delivered)
    );
    assert_eq!(
        cli::release_ix(DEVNET_JOB, &executor_token, &pass.journal, &cli_seal(&pass)),
        release_ix(DEVNET_JOB, &ADMITTED_MINT, &executor_token, pass.journal.clone(), groth16_seal(&pass))
    );
    assert_eq!(
        cli::refund_on_fail_ix(DEVNET_JOB, &buyer_token, &fail.journal, &cli_seal(&fail)),
        refund_on_fail_ix(DEVNET_JOB, &ADMITTED_MINT, &buyer_token, fail.journal.clone(), groth16_seal(&fail))
    );
    assert_eq!(
        cli::refund_on_timeout_ix(DEVNET_JOB, &buyer_token),
        refund_ix(DEVNET_JOB, &ADMITTED_MINT, &buyer_token)
    );
}

#[test]
fn seal_encoding_is_the_suite_encoding() {
    for name in ["pass", "fail"] {
        let fixture = suite_fixture(name);
        let ours = cli_seal(&fixture);
        let suite = groth16_seal(&fixture);
        assert_eq!(
            (ours.selector, ours.pi_a, ours.pi_b, ours.pi_c),
            (suite.selector, suite.pi_a, suite.pi_b, suite.pi_c),
            "{name}"
        );
        assert_eq!(cli::negate_g1(&fixture.seal[..64]), negate_g1(&fixture.seal[..64]));
        // Negation is an involution on the curve coordinates in the field.
        assert_eq!(cli::negate_g1(&cli::negate_g1(&fixture.seal[..64])), fixture.seal[..64]);
        assert_ne!(ours.pi_a, fixture.seal[..64], "{name}: pi_a must be negated");
    }
}

#[test]
fn ata_create_idempotent_has_the_create_accounts_and_tag_1() {
    let ix = cli::create_ata_idempotent_ix(&BUYER, &EXECUTOR);
    assert_eq!(ix.program_id, ATA_PROGRAM_ID);
    assert_eq!(
        ix.accounts,
        vec![
            AccountMeta::new(BUYER, true),
            AccountMeta::new(ata_address(&EXECUTOR, &ADMITTED_MINT), false),
            AccountMeta::new_readonly(EXECUTOR, false),
            AccountMeta::new_readonly(ADMITTED_MINT, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
        ]
    );
    assert_eq!(ix.data, vec![1]);
}

#[tokio::test]
async fn ata_create_idempotent_creates_the_canonical_account_once() {
    // SPL Token and the Associated Token Account program are embedded in
    // `solana-program-test`; the admitted mint goes in genesis.
    let mut program_test = ProgramTest::default();
    let authority = Keypair::new();
    add_admitted_mint(&mut program_test, &authority.pubkey(), None);
    let mut ctx = program_test.start_with_context().await;
    let owner = Pubkey::new_unique();
    let payer = ctx.payer.pubkey();
    for _ in 0..2 {
        let ix = cli::create_ata_idempotent_ix(&payer, &owner);
        send(&mut ctx, &[ix], &[]).await.unwrap();
    }
    let account = ctx.banks_client.get_account(cli::ata(&owner)).await.unwrap().unwrap();
    let view = cli::decode_token_account(&account.data).unwrap();
    assert_eq!((view.owner, view.mint, view.amount), (owner, ADMITTED_MINT, 0));
}

#[test]
fn transaction_sizes_are_the_devnet_sizes() {
    let buyer = Keypair::new();
    let executor = Keypair::new();
    let third = Keypair::new();
    let terms = cli::admitted_terms();
    let pass = suite_fixture("pass");
    let blockhash = Default::default();
    let executor_token = cli::ata(&executor.pubkey());
    let release = cli::release_ix(DEVNET_JOB, &executor_token, &pass.journal, &cli_seal(&pass));
    let deliver = cli::deliver_ix(&executor.pubkey(), DEVNET_JOB, pass.artifact_hash());
    let cases = [
        (
            "create+fund",
            vec![
                cli::create_job_ix(&buyer.pubkey(), DEVNET_JOB, &executor.pubkey(), AMOUNT, DEADLINE_SLOT, &terms),
                cli::fund_ix(&buyer.pubkey(), DEVNET_JOB, AMOUNT),
            ],
            &buyer,
            577,
        ),
        ("deliver+release", vec![deliver, release.clone()], &executor, 883),
        ("release", vec![release], &third, 838),
        (
            "refund_on_timeout",
            vec![cli::refund_on_timeout_ix(DEVNET_JOB, &cli::ata(&buyer.pubkey()))],
            &third,
            342,
        ),
    ];
    for (label, instructions, payer, size) in cases {
        let transaction = tx::sign(&instructions, payer, &[], blockhash);
        assert_eq!(tx::wire(&transaction).len(), size, "{label}");
        assert!(size <= cli::MAX_TRANSACTION_SIZE);
    }
}

#[test]
fn job_accounts_decode_like_anchor() {
    let artifact_hash = delivered_artifact_hash();
    let statuses = [
        (EscrowStatus::Created, cli::Status::Created),
        (EscrowStatus::Funded, cli::Status::Funded),
        (EscrowStatus::Released { artifact_hash }, cli::Status::Released { artifact_hash }),
        (EscrowStatus::RefundedOnFail { artifact_hash }, cli::Status::RefundedOnFail { artifact_hash }),
        (EscrowStatus::RefundedOnTimeout, cli::Status::RefundedOnTimeout),
        (EscrowStatus::Delivered { artifact_hash }, cli::Status::Delivered { artifact_hash }),
    ];
    let terms = admitted();
    for (anchor_status, expected) in statuses {
        let account = JobAccount {
            version: JOB_ACCOUNT_VERSION,
            bump: 254,
            vault_bump: 253,
            job_id: DEVNET_JOB,
            buyer: BUYER,
            executor: EXECUTOR,
            mint: ADMITTED_MINT,
            amount: AMOUNT,
            deadline_slot: DEADLINE_SLOT,
            spec_hash: terms.spec_hash,
            harness_hash: terms.harness_hash,
            image_id: terms.image_id,
            status: anchor_status,
        };
        let mut data = Vec::new();
        account.try_serialize(&mut data).unwrap();
        data.resize(cli::JOB_ACCOUNT_LEN, 0);
        let view = cli::decode_job(&data).unwrap();
        assert_eq!(view.status, expected);
        assert_eq!(
            (view.version, view.bump, view.vault_bump, view.job_id, view.buyer, view.executor, view.mint),
            (1, 254, 253, DEVNET_JOB, BUYER, EXECUTOR, ADMITTED_MINT)
        );
        assert_eq!((view.amount, view.deadline_slot), (AMOUNT, DEADLINE_SLOT));
        assert_eq!((view.spec_hash, view.harness_hash, view.image_id), (terms.spec_hash, terms.harness_hash, terms.image_id));
        let back = JobAccount::try_deserialize(&mut data.as_slice()).unwrap();
        assert_eq!(back.status, anchor_status);
    }
    let mut wrong = vec![0_u8; cli::JOB_ACCOUNT_LEN];
    assert!(cli::decode_job(&wrong).is_err(), "discriminator");
    wrong.truncate(100);
    assert!(cli::decode_job(&wrong).is_err(), "length");
}

#[test]
fn token_accounts_and_mints_decode_like_spl_token() {
    let delegate = Pubkey::new_unique();
    let close = Pubkey::new_unique();
    let account = spl_token::state::Account {
        mint: ADMITTED_MINT,
        owner: BUYER,
        amount: 999_998_000_000,
        delegate: COption::Some(delegate),
        state: spl_token::state::AccountState::Initialized,
        is_native: COption::None,
        delegated_amount: 7,
        close_authority: COption::Some(close),
    };
    let mut data = vec![0_u8; spl_token::state::Account::LEN];
    spl_token::state::Account::pack(account, &mut data).unwrap();
    let view = cli::decode_token_account(&data).unwrap();
    assert_eq!(
        (view.mint, view.owner, view.amount, view.delegate, view.state, view.is_native, view.delegated_amount, view.close_authority),
        (ADMITTED_MINT, BUYER, 999_998_000_000, Some(delegate), 1, None, 7, Some(close))
    );

    for freeze in [None, Some(Pubkey::new_unique())] {
        let mint = spl_token::state::Mint {
            mint_authority: COption::Some(BUYER),
            supply: 1_000_000_000_000,
            decimals: 6,
            is_initialized: true,
            freeze_authority: freeze.map_or(COption::None, COption::Some),
        };
        let mut data = vec![0_u8; spl_token::state::Mint::LEN];
        spl_token::state::Mint::pack(mint, &mut data).unwrap();
        let view = cli::decode_mint(&data).unwrap();
        assert_eq!(
            (view.mint_authority, view.supply, view.decimals, view.is_initialized, view.freeze_authority),
            (Some(BUYER), 1_000_000_000_000, 6, true, freeze)
        );
    }
}

#[test]
fn program_accounts_decode_like_the_upgradeable_loader() {
    let program_data = Pubkey::new_unique();
    let mut program = 2_u32.to_le_bytes().to_vec();
    program.extend_from_slice(program_data.as_ref());
    assert_eq!(cli::decode_program_account(&program).unwrap(), program_data);

    let mut header = 3_u32.to_le_bytes().to_vec();
    header.extend_from_slice(&507_798_457_u64.to_le_bytes());
    header.push(0);
    header.extend_from_slice(&[0; 32]);
    assert_eq!(cli::decode_program_data_header(&header).unwrap(), (507_798_457, None));
    header[12] = 1;
    header[13..45].copy_from_slice(BUYER.as_ref());
    assert_eq!(cli::decode_program_data_header(&header).unwrap(), (507_798_457, Some(BUYER)));
}

#[test]
fn receipts_are_read_and_checked_like_the_prover_writes_them() {
    let pass = suite_fixture("pass");
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("receipt-pass");
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("journal"), &pass.journal).unwrap();
    fs::write(dir.join("seal"), pass.seal).unwrap();
    fs::write(dir.join("selector"), pass.selector).unwrap();
    fs::write(dir.join("image_id"), pass.image_id).unwrap();
    fs::write(dir.join("journal_digest"), pass.journal_digest).unwrap();
    let receipt = Receipt::load(&dir).unwrap();
    assert_eq!(receipt.job_id(), JOB_ID);
    assert_eq!(receipt.artifact_hash(), pass.artifact_hash());
    assert_eq!(receipt.verdict(), vericode_core::Verdict::Pass);
    assert_eq!(receipt.seal(), cli_seal(&pass));

    fs::write(dir.join("journal_digest"), [0_u8; 32]).unwrap();
    assert!(Receipt::load(&dir).is_err(), "a journal_digest that is not SHA-256(journal)");
    fs::write(dir.join("journal_digest"), pass.journal_digest).unwrap();
    fs::write(dir.join("journal"), &pass.journal[..164]).unwrap();
    assert!(Receipt::load(&dir).is_err(), "a truncated journal");
}

#[test]
fn expected_errors_name_the_failing_program() {
    assert_eq!(
        tx::Expect::parse("escrow:6014").unwrap(),
        tx::Expect::Failure { program: vericode_escrow::ID, code: 6014 }
    );
    assert_eq!(
        tx::Expect::parse("verifier:6000").unwrap(),
        tx::Expect::Failure { program: GROTH16_VERIFIER_ID, code: 6000 }
    );
    assert!(tx::Expect::parse("router:1").is_err());
    assert!(tx::Expect::parse("escrow").is_err());
    assert!(!tx::Expect::Success.is_failure());
}

#[test]
fn keypairs_inside_a_git_work_tree_or_shared_are_refused() {
    let inside = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    assert!(keys::inside_git_work_tree(&fs::canonicalize(&inside).unwrap()));
    match keys::load(&inside, "buyer") {
        Ok(_) => panic!("a file inside the clone was accepted as a keypair"),
        Err(error) => assert!(error.contains("Git work tree"), "{error}"),
    }
    assert!(!keys::inside_git_work_tree(std::path::Path::new("/")));
    assert!(keys::check_mode(0o100600, "buyer").is_ok());
    assert!(keys::check_mode(0o100400, "buyer").is_ok());
    assert!(keys::check_mode(0o100644, "buyer").is_err());
    assert!(keys::check_mode(0o100640, "buyer").is_err());
    assert!(keys::check_mode(0o100602, "buyer").is_err());
}
