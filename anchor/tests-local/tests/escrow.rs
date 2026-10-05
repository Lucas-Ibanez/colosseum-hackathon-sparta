//! In-process tests of the SBF build of `vericode_escrow`: Job creation,
//! funding, delivery, timeout refund and account binding.
//!
//! Every rejected instruction is checked for unchanged Job, vault and token
//! balances. Settlement by verdict through the Groth16 verifier is tested in
//! `tests/settlement.rs`.

mod common;

use common::*;

#[tokio::test]
async fn create_job_rejects_invalid_terms_without_creating_accounts() {
    let mut env = setup().await;
    let buyer = env.buyer.insecure_clone();
    let job = job_pda(&JOB_ID);
    let watched = [buyer.pubkey(), job, vault_pda(&job)];
    let before = snapshot(&mut env.ctx, &watched).await;

    let cases = [
        (Pubkey::new_from_array([0; 32]), AMOUNT, E_ZERO_EXECUTOR),
        (buyer.pubkey(), AMOUNT, E_BUYER_IS_EXECUTOR),
        (env.executor.pubkey(), 0, E_AMOUNT_ZERO),
    ];
    for (executor, amount, error) in cases {
        let ix = create_job_ix(&buyer.pubkey(), &env.mint, JOB_ID, executor, amount, DEADLINE);
        assert_custom(send(&mut env.ctx, &[ix], &[&buyer]).await, error);
        assert_eq!(snapshot(&mut env.ctx, &watched).await, before);
    }
}

#[tokio::test]
async fn create_job_rejects_an_executor_that_is_a_program_account() {
    let mut env = setup().await;
    let buyer = env.buyer.insecure_clone();
    let job = job_pda(&JOB_ID);
    let vault = vault_pda(&job);
    let watched = [buyer.pubkey(), job, vault];
    let before = snapshot(&mut env.ctx, &watched).await;

    for executor in [job, vault] {
        let ix = create_job_ix(&buyer.pubkey(), &env.mint, JOB_ID, executor, AMOUNT, DEADLINE);
        assert_custom(send(&mut env.ctx, &[ix], &[&buyer]).await, E_EXECUTOR_IS_PROGRAM_ACCOUNT);
        assert_eq!(snapshot(&mut env.ctx, &watched).await, before);
    }
}

#[tokio::test]
async fn create_job_rejects_terms_not_admitted_for_v1() {
    let mut env = setup().await;
    let buyer = env.buyer.insecure_clone();
    let job = job_pda(&JOB_ID);
    let watched = [buyer.pubkey(), job, vault_pda(&job)];
    let before = snapshot(&mut env.ctx, &watched).await;
    let admitted = admitted();

    let cases = [
        (Commitments { spec_hash: OTHER, ..admitted }, E_SPEC_NOT_ADMITTED),
        (
            Commitments {
                spec_hash: admitted.harness_hash,
                harness_hash: admitted.spec_hash,
                ..admitted
            },
            E_SPEC_NOT_ADMITTED,
        ),
        (Commitments { harness_hash: OTHER, ..admitted }, E_HARNESS_NOT_ADMITTED),
        // A guest chosen by the buyer, for example one that always fails.
        (Commitments { image_id: OTHER, ..admitted }, E_IMAGE_ID_NOT_ADMITTED),
    ];
    for (commitments, error) in cases {
        let ix = create_job_ix_with(
            &buyer.pubkey(),
            &env.mint,
            JOB_ID,
            env.executor.pubkey(),
            AMOUNT,
            DEADLINE,
            commitments,
        );
        assert_custom(send(&mut env.ctx, &[ix], &[&buyer]).await, error);
        assert_eq!(snapshot(&mut env.ctx, &watched).await, before);
    }
}

#[tokio::test]
async fn create_job_bounds_the_deadline_window() {
    let mut env = setup().await;
    let buyer = env.buyer.insecure_clone();
    let slot = current_slot(&mut env.ctx).await;
    let job = job_pda(&JOB_ID);
    let watched = [buyer.pubkey(), job, vault_pda(&job)];
    let before = snapshot(&mut env.ctx, &watched).await;

    // R-D2 PoC-5 (deadline 0) and PoC-9 (deadline u64::MAX) included.
    for deadline in [slot + MIN_WINDOW - 1, slot + MAX_WINDOW + 1, 0, slot, u64::MAX] {
        let ix = create_job_ix(&buyer.pubkey(), &env.mint, JOB_ID, env.executor.pubkey(), AMOUNT, deadline);
        assert_custom(send(&mut env.ctx, &[ix], &[&buyer]).await, E_DEADLINE_OUT_OF_WINDOW);
        assert_eq!(snapshot(&mut env.ctx, &watched).await, before);
    }

    // Both edges are admitted, against the same Clock slot.
    for (job_id, deadline) in [([0x21; 32], slot + MIN_WINDOW), ([0x22; 32], slot + MAX_WINDOW)] {
        let ix = create_job_ix(&buyer.pubkey(), &env.mint, job_id, env.executor.pubkey(), AMOUNT, deadline);
        send(&mut env.ctx, &[ix], &[&buyer]).await.unwrap();
        assert_eq!(read_job(&mut env.ctx, &job_pda(&job_id)).await.deadline_slot, deadline);
    }
    assert_eq!(current_slot(&mut env.ctx).await, slot, "Clock.slot moved during the test");
}

#[tokio::test]
async fn create_job_rejects_token_2022() {
    let mut env = setup().await;
    let buyer = env.buyer.insecure_clone();
    let authority = env.mint_authority.pubkey();
    let token_2022_mint = create_token_2022_mint(&mut env.ctx, &authority).await;
    let job = job_pda(&JOB_ID);
    let watched = [buyer.pubkey(), job, vault_pda(&job)];
    let before = snapshot(&mut env.ctx, &watched).await;

    let executor = env.executor.pubkey();
    let foreign_mint = create_job_ix(&buyer.pubkey(), &token_2022_mint, JOB_ID, executor, AMOUNT, DEADLINE);
    assert_custom(
        send(&mut env.ctx, &[foreign_mint], &[&buyer]).await,
        ANCHOR_ACCOUNT_OWNED_BY_WRONG_PROGRAM,
    );
    let foreign_program = substitute(
        create_job_ix(&buyer.pubkey(), &env.mint, JOB_ID, executor, AMOUNT, DEADLINE),
        &spl_token::ID,
        &spl_token_2022::ID,
    );
    assert_custom(
        send(&mut env.ctx, &[foreign_program], &[&buyer]).await,
        ANCHOR_INVALID_PROGRAM_ID,
    );
    assert_eq!(snapshot(&mut env.ctx, &watched).await, before);
}

#[tokio::test]
async fn create_job_rejects_a_mint_with_freeze_authority() {
    let mut env = setup().await;
    let buyer = env.buyer.insecure_clone();
    let freezer = Keypair::new();
    let freezable = create_mint_with_freeze(
        &mut env.ctx,
        &freezer.pubkey(),
        Some(&freezer.pubkey()),
    )
    .await;
    let job = job_pda(&JOB_ID);

    // The freeze rule is a raw constraint, checked before the admitted mint
    // address (6036) by Anchor 0.31.1.
    let ix = create_job_ix(&buyer.pubkey(), &freezable, JOB_ID, env.executor.pubkey(), AMOUNT, DEADLINE);
    assert_custom(send(&mut env.ctx, &[ix], &[&buyer]).await, E_MINT_HAS_FREEZE_AUTHORITY);
    assert!(env.ctx.banks_client.get_account(job).await.unwrap().is_none());
    assert!(env.ctx.banks_client.get_account(vault_pda(&job)).await.unwrap().is_none());
}

#[tokio::test]
async fn create_job_admits_only_the_test_usdc_mint() {
    let mut env = setup().await;
    let buyer = env.buyer.insecure_clone();
    // A classic SPL mint without freeze authority, controlled by the buyer
    // (R-D2 F-05): a Job in a worthless token could deceive an executor.
    let own_mint = create_mint(&mut env.ctx, &buyer.pubkey()).await;
    let job = job_pda(&JOB_ID);
    let watched = [buyer.pubkey(), job, vault_pda(&job)];
    let before = snapshot(&mut env.ctx, &watched).await;

    let ix = create_job_ix(&buyer.pubkey(), &own_mint, JOB_ID, env.executor.pubkey(), AMOUNT, DEADLINE);
    assert_custom(send(&mut env.ctx, &[ix], &[&buyer]).await, E_MINT_NOT_ADMITTED);
    assert_eq!(snapshot(&mut env.ctx, &watched).await, before);

    let admitted = create_job_ix(&buyer.pubkey(), &ADMITTED_MINT, JOB_ID, env.executor.pubkey(), AMOUNT, DEADLINE);
    send(&mut env.ctx, &[admitted], &[&buyer]).await.unwrap();
    assert_eq!(read_job(&mut env.ctx, &job).await.mint, ADMITTED_MINT);
}

#[tokio::test]
async fn create_job_with_variants_of_the_account_at_the_admitted_mint_address() {
    // R-D4a RD4A-07 (f), PoC i5: what `create_job` does if the account at
    // `ADMITTED_MINT` is not the planned Test USDC. A 9-decimal mint is
    // accepted: the program does not check decimals (RD4A-02); the devnet
    // client checks 6 decimals and no freeze authority before every operation.
    use anchor_spl::token::spl_token::solana_program::program_option::COption;
    const ANCHOR_ACCOUNT_NOT_INITIALIZED: u32 = 3012;
    let authority = Keypair::new().pubkey();
    let cases: [(&str, Pubkey, u8, Option<Pubkey>, Option<u32>); 4] = [
        ("SPL Token, freeze authority", spl_token::ID, DECIMALS, Some(authority), Some(E_MINT_HAS_FREEZE_AUTHORITY)),
        ("Token-2022 owner", spl_token_2022::ID, DECIMALS, None, Some(ANCHOR_ACCOUNT_OWNED_BY_WRONG_PROGRAM)),
        ("SPL Token, 9 decimals", spl_token::ID, 9, None, None),
        ("absent", Pubkey::default(), DECIMALS, None, Some(ANCHOR_ACCOUNT_NOT_INITIALIZED)),
    ];
    for (label, owner, decimals, freeze_authority, expected) in cases {
        let mut program_test = program_test();
        if owner != Pubkey::default() {
            let mint = spl_token::state::Mint {
                mint_authority: COption::Some(authority),
                supply: 0,
                decimals,
                is_initialized: true,
                freeze_authority: freeze_authority.map_or(COption::None, COption::Some),
            };
            let mut data = vec![0_u8; spl_token::state::Mint::LEN];
            spl_token::state::Mint::pack(mint, &mut data).unwrap();
            let lamports = Rent::default().minimum_balance(data.len());
            program_test.add_account_with_base64_data(ADMITTED_MINT, lamports, owner, &base64(&data));
        }
        let mut ctx = program_test.start_with_context().await;
        let buyer = Keypair::new();
        transfer_lamports(&mut ctx, &buyer.pubkey(), 1_000_000_000).await;
        let slot = current_slot(&mut ctx).await;
        let executor = Keypair::new().pubkey();
        let ix = create_job_ix(&buyer.pubkey(), &ADMITTED_MINT, JOB_ID, executor, AMOUNT, slot + 3_000);
        let job = job_pda(&JOB_ID);
        let watched = [buyer.pubkey(), job, vault_pda(&job)];
        match expected {
            Some(code) => {
                let before = snapshot(&mut ctx, &watched).await;
                assert_custom(send(&mut ctx, &[ix], &[&buyer]).await, code);
                assert_eq!(snapshot(&mut ctx, &watched).await, before, "{label}");
            }
            None => {
                send(&mut ctx, &[ix], &[&buyer]).await.unwrap();
                assert_eq!(read_job(&mut ctx, &job).await.mint, ADMITTED_MINT, "{label}");
            }
        }
    }
}

#[tokio::test]
async fn freeze_authority_cannot_be_added_to_the_job_mint() {
    let mut env = setup().await;
    create_default_job(&mut env).await;
    let authority = env.mint_authority.insecure_clone();

    // The accepted mint was created without freeze authority; the SPL Token
    // program executed here (`spl_token-3.5.0.so`) refuses to add one later,
    // so the vault can never be frozen.
    let ix = spl_token::instruction::set_authority(
        &spl_token::ID,
        &env.mint,
        Some(&authority.pubkey()),
        spl_token::instruction::AuthorityType::FreezeAccount,
        &authority.pubkey(),
        &[],
    )
    .unwrap();
    assert_custom(
        send(&mut env.ctx, &[ix], &[&authority]).await,
        spl_token::error::TokenError::MintCannotFreeze as u32,
    );
}

#[tokio::test]
async fn create_job_rejects_a_duplicate_job_id() {
    let mut env = setup().await;
    let job = job_pda(&JOB_ID);

    // R-D2 PoC-9: a squatter takes the Job ID first, with its own amount,
    // inside the creation window. Since D4a only the admitted mint is
    // accepted, so the squatter uses it too.
    let squatter = Keypair::new();
    transfer_lamports(&mut env.ctx, &squatter.pubkey(), 1_000_000_000).await;
    let squat = create_job_ix(&squatter.pubkey(), &env.mint, JOB_ID, env.executor.pubkey(), 1, DEADLINE);
    send(&mut env.ctx, &[squat], &[&squatter]).await.unwrap();

    let buyer = env.buyer.insecure_clone();
    let watched = [buyer.pubkey(), job, vault_pda(&job)];
    let before = snapshot(&mut env.ctx, &watched).await;
    let ix = create_job_ix(&buyer.pubkey(), &env.mint, JOB_ID, env.executor.pubkey(), AMOUNT, DEADLINE);
    assert_custom(send(&mut env.ctx, &[ix], &[&buyer]).await, SYSTEM_ACCOUNT_ALREADY_IN_USE);
    assert_eq!(snapshot(&mut env.ctx, &watched).await, before);
}

#[tokio::test]
async fn create_job_persists_terms_and_an_empty_pda_vault() {
    let mut env = setup().await;
    create_default_job(&mut env).await;

    let job = job_pda(&JOB_ID);
    let account = read_job(&mut env.ctx, &job).await;
    assert_eq!(account.version, JOB_ACCOUNT_VERSION);
    assert_eq!(account.job_id, JOB_ID);
    assert_eq!(account.buyer, env.buyer.pubkey());
    assert_eq!(account.executor, env.executor.pubkey());
    assert_eq!(account.mint, env.mint);
    assert_eq!(account.amount, AMOUNT);
    assert_eq!(account.deadline_slot, DEADLINE);
    assert_eq!(account.spec_hash, admitted().spec_hash);
    assert_eq!(account.harness_hash, admitted().harness_hash);
    assert_eq!(account.image_id, hex32(IMAGE_ID_HEX));
    assert_eq!(account.status, EscrowStatus::Created);

    let vault = vault_pda(&job);
    let vault_account = env.ctx.banks_client.get_account(vault).await.unwrap().unwrap();
    assert_eq!(vault_account.owner, spl_token::ID);
    let vault_state = spl_token::state::Account::unpack(&vault_account.data).unwrap();
    assert_eq!(vault_state.mint, env.mint);
    assert_eq!(vault_state.owner, job);
    assert_eq!(vault_state.amount, 0);
}

#[tokio::test]
async fn fund_rejections_move_no_tokens_and_keep_the_state() {
    let mut env = setup().await;
    create_default_job(&mut env).await;
    let job = job_pda(&JOB_ID);
    let vault = vault_pda(&job);
    let buyer = env.buyer.insecure_clone();

    // Other mint, with a buyer token account of that mint.
    let other_authority = Keypair::new();
    let other_mint = create_mint(&mut env.ctx, &other_authority.pubkey()).await;
    let buyer_other_token = create_token_account(&mut env.ctx, &other_mint, &buyer.pubkey()).await;
    mint_to(&mut env.ctx, &other_mint, &buyer_other_token, &other_authority, AMOUNT).await;

    // Another signer holding the Job mint.
    let intruder = Keypair::new();
    let intruder_token = create_token_account(&mut env.ctx, &env.mint, &intruder.pubkey()).await;
    let authority = env.mint_authority.insecure_clone();
    mint_to(&mut env.ctx, &env.mint, &intruder_token, &authority, AMOUNT).await;

    let watched = [job, vault, env.buyer_token, buyer_other_token, intruder_token];
    let before = snapshot(&mut env.ctx, &watched).await;

    let wrong_mint = fund_ix(&buyer.pubkey(), JOB_ID, &other_mint, &buyer_other_token, AMOUNT);
    assert_custom(send(&mut env.ctx, &[wrong_mint], &[&buyer]).await, E_MINT_MISMATCH);
    // R-D2 F-08: a foreign mint account is a VeriCode error, not an SPL one.
    let foreign_mint_account = fund_ix(&buyer.pubkey(), JOB_ID, &other_mint, &env.buyer_token, AMOUNT);
    assert_custom(
        send(&mut env.ctx, &[foreign_mint_account], &[&buyer]).await,
        E_MINT_MISMATCH,
    );
    let wrong_signer = fund_ix(&intruder.pubkey(), JOB_ID, &env.mint, &intruder_token, AMOUNT);
    assert_custom(
        send(&mut env.ctx, &[wrong_signer], &[&intruder]).await,
        E_DEPOSITOR_MISMATCH,
    );
    let wrong_amount = fund_ix(&buyer.pubkey(), JOB_ID, &env.mint, &env.buyer_token, AMOUNT - 1);
    assert_custom(send(&mut env.ctx, &[wrong_amount], &[&buyer]).await, E_AMOUNT_MISMATCH);
    assert_eq!(snapshot(&mut env.ctx, &watched).await, before);

    fund_default_job(&mut env).await;
    let funded = snapshot(&mut env.ctx, &watched).await;
    let again = fund_ix(&buyer.pubkey(), JOB_ID, &env.mint, &env.buyer_token, AMOUNT);
    assert_custom(send(&mut env.ctx, &[again], &[&buyer]).await, E_ALREADY_FUNDED);
    assert_eq!(snapshot(&mut env.ctx, &watched).await, funded);
}

#[tokio::test]
async fn fund_moves_exactly_the_job_amount_into_the_vault() {
    let mut env = setup().await;
    create_default_job(&mut env).await;
    fund_default_job(&mut env).await;

    let job = job_pda(&JOB_ID);
    assert_eq!(read_job(&mut env.ctx, &job).await.status, EscrowStatus::Funded);
    assert_eq!(token_balance(&mut env.ctx, &vault_pda(&job)).await, AMOUNT);
    assert_eq!(
        token_balance(&mut env.ctx, &env.buyer_token.clone()).await,
        BUYER_START_BALANCE - AMOUNT
    );
}

#[tokio::test]
async fn vault_is_controlled_by_the_job_pda_without_private_key() {
    let mut env = setup().await;
    create_default_job(&mut env).await;
    let job = job_pda(&JOB_ID);
    let vault = vault_pda(&job);

    assert!(!job.is_on_curve(), "Job PDA must have no private key");
    assert!(!vault.is_on_curve(), "vault address must have no private key");
    let data = env.ctx.banks_client.get_account(vault).await.unwrap().unwrap().data;
    let state = spl_token::state::Account::unpack(&data).unwrap();
    assert_eq!(state.owner, job);
    assert!(state.delegate.is_none());
    assert!(state.close_authority.is_none());
}

#[tokio::test]
async fn deliver_rejects_other_signers_and_an_unfunded_job() {
    let mut env = setup().await;
    create_default_job(&mut env).await;
    let job = job_pda(&JOB_ID);
    let watched = [job, vault_pda(&job), env.buyer_token];
    let executor = env.executor.insecure_clone();
    let buyer = env.buyer.insecure_clone();
    let intruder = Keypair::new();
    let hash = delivered_artifact_hash();

    let created = snapshot(&mut env.ctx, &watched).await;
    let unfunded = deliver_ix(&executor.pubkey(), JOB_ID, hash);
    assert_custom(send(&mut env.ctx, &[unfunded], &[&executor]).await, E_NOT_FUNDED);
    assert_eq!(snapshot(&mut env.ctx, &watched).await, created);

    fund_default_job(&mut env).await;
    let funded = snapshot(&mut env.ctx, &watched).await;
    for signer in [&buyer, &intruder] {
        let ix = deliver_ix(&signer.pubkey(), JOB_ID, hash);
        assert_custom(send(&mut env.ctx, &[ix], &[signer]).await, E_DELIVERER_MISMATCH);
    }
    // Naming the executor without its signature is not a delivery.
    let forged = unsigned(deliver_ix(&executor.pubkey(), JOB_ID, hash), &executor.pubkey());
    assert_custom(send(&mut env.ctx, &[forged], &[]).await, ANCHOR_ACCOUNT_NOT_SIGNER);
    assert_eq!(snapshot(&mut env.ctx, &watched).await, funded);
    assert_eq!(read_job(&mut env.ctx, &job).await.status, EscrowStatus::Funded);
}

#[tokio::test]
async fn deliver_is_rejected_after_the_deadline_and_accepted_at_it() {
    let mut env = setup().await;
    let late_job_id = [0x12; 32];
    create_default_job(&mut env).await;
    create_job_with_deadline(&mut env, late_job_id, DEADLINE - 1).await;
    fund_default_job(&mut env).await;
    fund_job(&mut env, late_job_id).await;
    let executor = env.executor.insecure_clone();
    let hash = delivered_artifact_hash();

    warp(&mut env.ctx, DEADLINE).await;
    let late_job = job_pda(&late_job_id);
    let watched = [late_job, vault_pda(&late_job), env.buyer_token];
    let before = snapshot(&mut env.ctx, &watched).await;
    // One slot after the deadline of the late Job.
    let late = deliver_ix(&executor.pubkey(), late_job_id, hash);
    assert_custom(send(&mut env.ctx, &[late], &[&executor]).await, E_DEADLINE_PASSED);
    assert_eq!(snapshot(&mut env.ctx, &watched).await, before);

    // At the deadline slot of the default Job.
    let on_time = deliver_ix(&executor.pubkey(), JOB_ID, hash);
    send(&mut env.ctx, &[on_time], &[&executor]).await.unwrap();
    assert_eq!(
        read_job(&mut env.ctx, &job_pda(&JOB_ID)).await.status,
        EscrowStatus::Delivered { artifact_hash: hash }
    );
}

#[tokio::test]
async fn deliver_records_the_executor_commitment_once() {
    let mut env = setup().await;
    create_default_job(&mut env).await;
    fund_default_job(&mut env).await;
    let job = job_pda(&JOB_ID);
    let vault = vault_pda(&job);
    let hash = delivered_artifact_hash();
    // The same artifact as the versioned Groth16 PASS fixture of Job 0x11.
    assert_eq!(hash, fixture("pass").artifact_hash());

    let balances = snapshot(&mut env.ctx, &[vault, env.buyer_token]).await;
    deliver_default_job(&mut env).await;
    assert_eq!(
        read_job(&mut env.ctx, &job).await.status,
        EscrowStatus::Delivered { artifact_hash: hash }
    );
    assert_eq!(snapshot(&mut env.ctx, &[vault, env.buyer_token]).await, balances);

    let watched = [job, vault, env.buyer_token];
    let delivered = snapshot(&mut env.ctx, &watched).await;
    let executor = env.executor.insecure_clone();
    for artifact in [hash, fixture("fail").artifact_hash()] {
        let again = deliver_ix(&executor.pubkey(), JOB_ID, artifact);
        assert_custom(send(&mut env.ctx, &[again], &[&executor]).await, E_ALREADY_DELIVERED);
    }
    let buyer = env.buyer.insecure_clone();
    let fund_after_deliver = fund_ix(&buyer.pubkey(), JOB_ID, &env.mint, &env.buyer_token, AMOUNT);
    assert_custom(
        send(&mut env.ctx, &[fund_after_deliver], &[&buyer]).await,
        E_ALREADY_FUNDED,
    );
    assert_eq!(snapshot(&mut env.ctx, &watched).await, delivered);
}

#[tokio::test]
async fn timeout_refund_fails_before_and_at_the_deadline() {
    let mut env = setup().await;
    create_default_job(&mut env).await;
    fund_default_job(&mut env).await;
    let job = job_pda(&JOB_ID);
    let watched = [job, vault_pda(&job), env.buyer_token];
    let before = snapshot(&mut env.ctx, &watched).await;

    let early = refund_ix(JOB_ID, &env.mint, &env.buyer_token);
    assert_custom(send(&mut env.ctx, &[early], &[]).await, E_DEADLINE_NOT_REACHED);
    warp(&mut env.ctx, DEADLINE).await;
    let at_deadline = refund_ix(JOB_ID, &env.mint, &env.buyer_token);
    assert_custom(send(&mut env.ctx, &[at_deadline], &[]).await, E_DEADLINE_NOT_REACHED);
    assert_eq!(current_slot(&mut env.ctx).await, DEADLINE);
    assert_eq!(snapshot(&mut env.ctx, &watched).await, before);
}

#[tokio::test]
async fn timeout_refund_rejects_other_recipients_and_mints() {
    let mut env = setup().await;
    create_default_job(&mut env).await;
    fund_default_job(&mut env).await;
    let job = job_pda(&JOB_ID);
    let executor = env.executor.pubkey();
    let executor_token = create_token_account(&mut env.ctx, &env.mint, &executor).await;
    let other_authority = Keypair::new();
    let other_mint = create_mint(&mut env.ctx, &other_authority.pubkey()).await;
    let buyer = env.buyer.pubkey();
    let buyer_other_token = create_token_account(&mut env.ctx, &other_mint, &buyer).await;
    warp(&mut env.ctx, DEADLINE + 1).await;

    let watched = [job, vault_pda(&job), env.buyer_token, executor_token, buyer_other_token];
    let before = snapshot(&mut env.ctx, &watched).await;

    let to_executor = refund_ix(JOB_ID, &env.mint, &executor_token);
    assert_custom(send(&mut env.ctx, &[to_executor], &[]).await, E_RECIPIENT_MISMATCH);
    let other_mint_account = refund_ix(JOB_ID, &env.mint, &buyer_other_token);
    assert_custom(send(&mut env.ctx, &[other_mint_account], &[]).await, E_MINT_MISMATCH);
    // R-D2 PoC-3 / F-08: a foreign mint account used to fail inside SPL Token
    // (`0x3`); it is now bound to the Job mint by the program.
    let foreign_mint_account = refund_ix(JOB_ID, &other_mint, &env.buyer_token);
    assert_custom(send(&mut env.ctx, &[foreign_mint_account], &[]).await, E_MINT_MISMATCH);
    assert_eq!(snapshot(&mut env.ctx, &watched).await, before);
}

#[tokio::test]
async fn timeout_refund_pays_only_the_canonical_buyer_account() {
    let mut env = setup().await;
    create_default_job(&mut env).await;
    fund_default_job(&mut env).await;
    let job = job_pda(&JOB_ID);
    // R-D2 PoC-4: another account of the buyer, with the Job mint and an old
    // delegate that could drain it.
    let buyer = env.buyer.insecure_clone();
    let delegate = Keypair::new();
    let delegated = create_token_account(&mut env.ctx, &env.mint, &buyer.pubkey()).await;
    let approve = spl_token::instruction::approve(
        &spl_token::ID,
        &delegated,
        &delegate.pubkey(),
        &buyer.pubkey(),
        &[],
        AMOUNT,
    )
    .unwrap();
    send(&mut env.ctx, &[approve], &[&buyer]).await.unwrap();
    warp(&mut env.ctx, DEADLINE + 1).await;

    let watched = [job, vault_pda(&job), env.buyer_token, delegated];
    let before = snapshot(&mut env.ctx, &watched).await;
    // A permissionless caller cannot choose the destination (R-D2 F-06).
    let to_delegated = refund_ix(JOB_ID, &env.mint, &delegated);
    assert_custom(send(&mut env.ctx, &[to_delegated], &[]).await, E_DESTINATION_NOT_CANONICAL);
    assert_eq!(snapshot(&mut env.ctx, &watched).await, before);

    let to_canonical = refund_ix(JOB_ID, &env.mint, &env.buyer_token);
    send(&mut env.ctx, &[to_canonical], &[]).await.unwrap();
    assert_eq!(env.buyer_token, ata_address(&buyer.pubkey(), &env.mint));
    assert_eq!(
        token_balance(&mut env.ctx, &env.buyer_token.clone()).await,
        BUYER_START_BALANCE
    );
    assert_eq!(token_balance(&mut env.ctx, &delegated).await, 0);
}

#[tokio::test]
async fn timeout_refund_requires_funding_and_settles_once() {
    let mut env = setup().await;
    create_default_job(&mut env).await;
    let job = job_pda(&JOB_ID);
    warp(&mut env.ctx, DEADLINE + 1).await;
    let watched = [job, vault_pda(&job), env.buyer_token];

    let before = snapshot(&mut env.ctx, &watched).await;
    let unfunded = refund_ix(JOB_ID, &env.mint, &env.buyer_token);
    assert_custom(send(&mut env.ctx, &[unfunded], &[]).await, E_NOT_FUNDED);
    assert_eq!(snapshot(&mut env.ctx, &watched).await, before);

    fund_default_job(&mut env).await;
    let refund = refund_ix(JOB_ID, &env.mint, &env.buyer_token);
    send(&mut env.ctx, &[refund], &[]).await.unwrap();
    let settled = snapshot(&mut env.ctx, &watched).await;

    let again = refund_ix(JOB_ID, &env.mint, &env.buyer_token);
    assert_custom(send(&mut env.ctx, &[again], &[]).await, E_ALREADY_REFUNDED);
    let buyer = env.buyer.insecure_clone();
    let refund_then_fund = fund_ix(&buyer.pubkey(), JOB_ID, &env.mint, &env.buyer_token, AMOUNT);
    assert_custom(
        send(&mut env.ctx, &[refund_then_fund], &[&buyer]).await,
        E_ALREADY_REFUNDED,
    );
    assert_eq!(snapshot(&mut env.ctx, &watched).await, settled);
}

#[tokio::test]
async fn timeout_refund_after_the_deadline_returns_exactly_the_amount_to_the_buyer() {
    let mut env = setup().await;
    create_default_job(&mut env).await;
    fund_default_job(&mut env).await;
    let job = job_pda(&JOB_ID);
    warp(&mut env.ctx, DEADLINE + 1).await;

    // Permissionless: only the fee payer signs; the buyer does not.
    let refund = refund_ix(JOB_ID, &env.mint, &env.buyer_token);
    send(&mut env.ctx, &[refund], &[]).await.unwrap();

    assert_eq!(
        read_job(&mut env.ctx, &job).await.status,
        EscrowStatus::RefundedOnTimeout
    );
    assert_eq!(token_balance(&mut env.ctx, &vault_pda(&job)).await, 0);
    assert_eq!(
        token_balance(&mut env.ctx, &env.buyer_token.clone()).await,
        BUYER_START_BALANCE
    );
}

#[tokio::test]
async fn timeout_refund_returns_a_delivered_job_to_the_buyer() {
    let mut env = setup().await;
    create_default_job(&mut env).await;
    fund_default_job(&mut env).await;
    deliver_default_job(&mut env).await;
    let job = job_pda(&JOB_ID);
    let watched = [job, vault_pda(&job), env.buyer_token];
    let before = snapshot(&mut env.ctx, &watched).await;

    let early = refund_ix(JOB_ID, &env.mint, &env.buyer_token);
    assert_custom(send(&mut env.ctx, &[early], &[]).await, E_DEADLINE_NOT_REACHED);
    warp(&mut env.ctx, DEADLINE).await;
    let at_deadline = refund_ix(JOB_ID, &env.mint, &env.buyer_token);
    assert_custom(send(&mut env.ctx, &[at_deadline], &[]).await, E_DEADLINE_NOT_REACHED);
    assert_eq!(snapshot(&mut env.ctx, &watched).await, before);

    warp(&mut env.ctx, DEADLINE + 1).await;
    let refund = refund_ix(JOB_ID, &env.mint, &env.buyer_token);
    send(&mut env.ctx, &[refund], &[]).await.unwrap();
    assert_eq!(
        read_job(&mut env.ctx, &job).await.status,
        EscrowStatus::RefundedOnTimeout
    );
    assert_eq!(token_balance(&mut env.ctx, &vault_pda(&job)).await, 0);
    assert_eq!(
        token_balance(&mut env.ctx, &env.buyer_token.clone()).await,
        BUYER_START_BALANCE
    );

    let settled = snapshot(&mut env.ctx, &watched).await;
    let executor = env.executor.insecure_clone();
    let deliver_after_refund = deliver_ix(&executor.pubkey(), JOB_ID, delivered_artifact_hash());
    assert_custom(
        send(&mut env.ctx, &[deliver_after_refund], &[&executor]).await,
        E_ALREADY_REFUNDED,
    );
    assert_eq!(snapshot(&mut env.ctx, &watched).await, settled);
}

#[tokio::test]
async fn accounts_bound_to_the_job_reject_substitutes() {
    let mut env = setup().await;
    create_default_job(&mut env).await;
    fund_default_job(&mut env).await;
    let job = job_pda(&JOB_ID);
    let vault = vault_pda(&job);
    // A token account of the Job mint owned by the Job PDA, at another address.
    let fake_vault = create_token_account(&mut env.ctx, &env.mint, &job).await;
    // Past the deadline, so that only the substituted account is wrong. The
    // warp comes first: it verifies the bank capitalization, which an
    // injected account would change.
    warp(&mut env.ctx, DEADLINE + 1).await;
    // A byte-for-byte copy of the Job account owned by another program.
    let forged_job = Keypair::new().pubkey();
    let mut forged = env.ctx.banks_client.get_account(job).await.unwrap().unwrap();
    forged.owner = system_program::ID;
    env.ctx.set_account(&forged_job, &forged.into());

    let buyer = env.buyer.insecure_clone();
    let executor = env.executor.insecure_clone();
    let watched = [job, vault, fake_vault, forged_job, env.buyer_token];
    let before = snapshot(&mut env.ctx, &watched).await;

    let fund_fake_vault = substitute(
        fund_ix(&buyer.pubkey(), JOB_ID, &env.mint, &env.buyer_token, AMOUNT),
        &vault,
        &fake_vault,
    );
    assert_custom(send(&mut env.ctx, &[fund_fake_vault], &[&buyer]).await, ANCHOR_CONSTRAINT_SEEDS);
    let refund_fake_vault = substitute(refund_ix(JOB_ID, &env.mint, &env.buyer_token), &vault, &fake_vault);
    assert_custom(send(&mut env.ctx, &[refund_fake_vault], &[]).await, ANCHOR_CONSTRAINT_SEEDS);

    let forged_instructions = [
        (substitute(refund_ix(JOB_ID, &env.mint, &env.buyer_token), &job, &forged_job), None),
        (
            substitute(
                fund_ix(&buyer.pubkey(), JOB_ID, &env.mint, &env.buyer_token, AMOUNT),
                &job,
                &forged_job,
            ),
            Some(&buyer),
        ),
        (
            substitute(deliver_ix(&executor.pubkey(), JOB_ID, delivered_artifact_hash()), &job, &forged_job),
            Some(&executor),
        ),
    ];
    for (ix, signer) in forged_instructions {
        let signers: Vec<&Keypair> = signer.into_iter().collect();
        assert_custom(
            send(&mut env.ctx, &[ix], &signers).await,
            ANCHOR_ACCOUNT_OWNED_BY_WRONG_PROGRAM,
        );
    }
    assert_eq!(snapshot(&mut env.ctx, &watched).await, before);
}

#[tokio::test]
async fn unsupported_account_version_is_rejected() {
    let mut env = setup().await;
    create_default_job(&mut env).await;
    let job = job_pda(&JOB_ID);
    let mut account = env.ctx.banks_client.get_account(job).await.unwrap().unwrap();
    // `JobAccount::version` follows the 8-byte Anchor discriminator.
    assert_eq!(account.data[8], JOB_ACCOUNT_VERSION);
    account.data[8] = JOB_ACCOUNT_VERSION + 1;
    env.ctx.set_account(&job, &account.into());

    let watched = [job, vault_pda(&job), env.buyer_token];
    let before = snapshot(&mut env.ctx, &watched).await;
    let buyer = env.buyer.insecure_clone();
    let fund = fund_ix(&buyer.pubkey(), JOB_ID, &env.mint, &env.buyer_token, AMOUNT);
    assert_custom(send(&mut env.ctx, &[fund], &[&buyer]).await, E_UNSUPPORTED_ACCOUNT_VERSION);
    let executor = env.executor.insecure_clone();
    let deliver = deliver_ix(&executor.pubkey(), JOB_ID, delivered_artifact_hash());
    assert_custom(send(&mut env.ctx, &[deliver], &[&executor]).await, E_UNSUPPORTED_ACCOUNT_VERSION);
    assert_eq!(snapshot(&mut env.ctx, &watched).await, before);
}

#[tokio::test]
async fn create_job_compute_units() {
    let mut env = setup().await;
    // Relative to the Clock, so the same instruction also runs on a program
    // without the creation window (the D2c.1 build, for comparison).
    let deadline = current_slot(&mut env.ctx).await + 10_000;
    let buyer = env.buyer.insecure_clone();
    let ix = create_job_ix(&buyer.pubkey(), &env.mint, JOB_ID, env.executor.pubkey(), AMOUNT, deadline);
    let units = compute_units(&mut env.ctx, &[ix], &[&buyer]).await;
    println!("create_job compute units: {units}");
    assert!(units < 200_000, "create_job uses {units} compute units");
}

#[tokio::test]
async fn deliver_compute_units() {
    let mut env = setup().await;
    create_default_job(&mut env).await;
    fund_default_job(&mut env).await;
    let executor = env.executor.insecure_clone();
    let ix = deliver_ix(&executor.pubkey(), JOB_ID, delivered_artifact_hash());
    let units = compute_units(&mut env.ctx, &[ix], &[&executor]).await;
    println!("deliver compute units: {units}");
    assert!(units < 200_000, "deliver uses {units} compute units");
}
