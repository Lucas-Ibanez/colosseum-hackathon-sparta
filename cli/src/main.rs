//! `vericode`: devnet client of the VeriCode escrow. See `cli/README.md`.

use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
    process::ExitCode,
    thread::sleep,
    time::Duration,
};

use serde_json::json;
use solana_instruction::Instruction;
use solana_keypair::Keypair;
use solana_pubkey::Pubkey;
use solana_signer::Signer;
use vericode_core::Verdict;
use vericode_cli::{
    escrow::{self, JobView, Status, ADMITTED_IMAGE_ID_V1, ADMITTED_MINT, LANDING_MARGIN_SLOTS},
    explorer_address, hex, keys,
    receipt::Receipt,
    rpc::{Rpc, DEFAULT_RPC_URL},
    sha256,
    tx::{self, Expect},
    unhex32,
};

const USAGE: &str = "usage: vericode [--rpc-url URL] [--log FILE.jsonl] <command>

commands:
  check
  job create --buyer-keypair FILE --executor PUBKEY [--amount UNITS] [--deadline-offset SLOTS] [--job-file FILE]
  job deliver --executor-keypair FILE --job-id HEX (--artifact INPUT,OUTPUT | --receipt DIR)
  job settle --job-id HEX --receipt DIR (--payer-keypair FILE | --deliver --executor-keypair FILE)
             [--expect-error PROGRAM:CODE [--tamper-seal]]
  job refund-timeout --job-id HEX --payer-keypair FILE [--wait] [--expect-error PROGRAM:CODE]
  job show --job-id HEX

PROGRAM is escrow, verifier, token or system. Devnet only.";

const FLAGS: [&str; 3] = ["--deliver", "--wait", "--tamper-seal"];
const DEFAULT_AMOUNT: u64 = 1_000_000;
const DEFAULT_DEADLINE_OFFSET: u64 = 9_000;
const SIGNATURE_FEE: u64 = 5_000;

type Result<T> = std::result::Result<T, String>;

struct Args {
    positional: Vec<String>,
    values: HashMap<String, String>,
    flags: HashSet<String>,
}

impl Args {
    fn parse(raw: Vec<String>) -> Result<Self> {
        let mut args = Self {
            positional: Vec::new(),
            values: HashMap::new(),
            flags: HashSet::new(),
        };
        let mut iter = raw.into_iter();
        while let Some(arg) = iter.next() {
            if FLAGS.contains(&arg.as_str()) {
                args.flags.insert(arg);
            } else if arg.starts_with("--") {
                let value = iter.next().ok_or_else(|| format!("{arg} needs a value"))?;
                if args.values.insert(arg.clone(), value).is_some() {
                    return Err(format!("{arg} given twice"));
                }
            } else {
                args.positional.push(arg);
            }
        }
        Ok(args)
    }

    fn take(&mut self, key: &str) -> Option<String> {
        self.values.remove(key)
    }

    fn require(&mut self, key: &str) -> Result<String> {
        self.take(key).ok_or_else(|| format!("missing {key}"))
    }

    fn flag(&mut self, key: &str) -> bool {
        self.flags.remove(key)
    }

    fn finish(self) -> Result<()> {
        let mut unknown: Vec<String> = self.values.into_keys().chain(self.flags).collect();
        unknown.sort();
        if unknown.is_empty() {
            Ok(())
        } else {
            Err(format!("unexpected options for this command: {}", unknown.join(", ")))
        }
    }
}

fn parse_pubkey(text: &str, what: &str) -> Result<Pubkey> {
    text.parse().map_err(|_| format!("{what}: not a base58 pubkey: {text:?}"))
}

fn parse_u64(text: &str, what: &str) -> Result<u64> {
    text.parse().map_err(|_| format!("{what}: not an unsigned integer: {text:?}"))
}

fn job_id_arg(args: &mut Args) -> Result<[u8; 32]> {
    let job_id = unhex32(&args.require("--job-id")?)?;
    if job_id == [0x11; 32] {
        return Err("job_id 0x11… belongs to the local test fixtures, never to devnet".into());
    }
    Ok(job_id)
}

fn load_key(args: &mut Args, option: &str, role: &str) -> Result<Keypair> {
    let path = PathBuf::from(args.require(option)?);
    let keypair = keys::load(&path, role)?;
    println!("{}", keys::describe(role, &keypair));
    Ok(keypair)
}

/// Pre-send checks. Outside `--expect-error` any failure refuses the
/// operation before anything is signed or sent; with `--expect-error` the
/// failures are reported as the rejections the program must produce.
struct Prechecks {
    negative: bool,
    problems: Vec<String>,
}

impl Prechecks {
    fn new(expect: Expect) -> Self {
        Self {
            negative: expect.is_failure(),
            problems: Vec::new(),
        }
    }

    fn require(&mut self, ok: bool, problem: impl Into<String>) {
        if !ok {
            self.problems.push(problem.into());
        }
    }

    fn finish(self) -> Result<()> {
        if self.problems.is_empty() {
            println!("precheck=ok");
            return Ok(());
        }
        if self.negative {
            for problem in &self.problems {
                println!("precheck.expected_rejection={problem}");
            }
            return Ok(());
        }
        Err(format!("refused before sending: {}", self.problems.join("; ")))
    }
}

/// Cluster, program and mint checks run before every operation.
fn preflight(rpc: &Rpc) -> Result<()> {
    let genesis = rpc.genesis_hash()?;
    if genesis != escrow::DEVNET_GENESIS_HASH {
        return Err(format!("RPC {} is not Solana devnet (genesis {genesis})", rpc.url()));
    }
    println!("check.cluster=devnet genesis={genesis}");
    let program = rpc.account(&escrow::PROGRAM_ID)?.ok_or("escrow program account not found")?;
    if !program.executable || program.owner != escrow::LOADER_V3_ID {
        return Err("escrow program is not an executable upgradeable program".into());
    }
    if escrow::decode_program_account(&program.data)? != escrow::PROGRAM_DATA_ID {
        return Err("escrow ProgramData address differs from B7s9JJVy…".into());
    }
    let header = rpc
        .account_slice(&escrow::PROGRAM_DATA_ID, 0, escrow::PROGRAM_DATA_HEADER_LEN)?
        .ok_or("escrow ProgramData not found")?;
    let (deployed_slot, authority) = escrow::decode_program_data_header(&header.data)?;
    if authority.is_some() {
        return Err("escrow upgrade authority is not none".into());
    }
    println!("check.escrow={} upgrade_authority=none deployed_slot={deployed_slot}", escrow::PROGRAM_ID);
    check_mint(rpc)?;
    let terms = escrow::admitted_terms();
    println!(
        "check.terms_v1 spec_hash={} harness_hash={} image_id={}",
        hex(&terms.spec_hash),
        hex(&terms.harness_hash),
        hex(&terms.image_id)
    );
    Ok(())
}

fn check_mint(rpc: &Rpc) -> Result<()> {
    let account = rpc.account(&ADMITTED_MINT)?.ok_or("admitted Test USDC mint not found")?;
    if account.owner != escrow::TOKEN_PROGRAM_ID {
        return Err(format!("mint owner {} is not the SPL Token program", account.owner));
    }
    let mint = escrow::decode_mint(&account.data)?;
    if !mint.is_initialized || mint.decimals != escrow::MINT_DECIMALS || mint.freeze_authority.is_some() {
        return Err(format!(
            "mint is not the admitted Test USDC: initialized={} decimals={} freeze_authority={:?}",
            mint.is_initialized, mint.decimals, mint.freeze_authority
        ));
    }
    println!(
        "check.mint={ADMITTED_MINT} decimals={} freeze_authority=none supply={}",
        mint.decimals, mint.supply
    );
    Ok(())
}

fn read_job(rpc: &Rpc, job_id: &[u8; 32]) -> Result<Option<JobView>> {
    let address = escrow::job_pda(job_id);
    let Some(account) = rpc.account(&address)? else {
        return Ok(None);
    };
    if account.owner != escrow::PROGRAM_ID {
        return Err(format!("account {address} is not owned by the escrow program"));
    }
    let job = escrow::decode_job(&account.data)?;
    if job.job_id != *job_id {
        return Err("Job account holds another job_id".into());
    }
    Ok(Some(job))
}

fn require_job(rpc: &Rpc, job_id: &[u8; 32]) -> Result<JobView> {
    read_job(rpc, job_id)?.ok_or_else(|| format!("no Job {} for job_id {}", escrow::job_pda(job_id), hex(job_id)))
}

/// Problems of a Job account against the admitted v1 terms.
fn term_problems(job: &JobView) -> Vec<String> {
    let terms = escrow::admitted_terms();
    let mut problems = Vec::new();
    if job.version != 1 {
        problems.push(format!("Job account version {} (6023)", job.version));
    }
    if job.mint != ADMITTED_MINT {
        problems.push("Job mint is not the admitted Test USDC".into());
    }
    if job.spec_hash != terms.spec_hash || job.harness_hash != terms.harness_hash || job.image_id != terms.image_id {
        problems.push("Job terms are not the admitted v1 terms".into());
    }
    problems
}

fn token_account(rpc: &Rpc, key: &Pubkey) -> Result<Option<escrow::TokenAccountView>> {
    match rpc.account(key)? {
        None => Ok(None),
        Some(account) if account.owner == escrow::TOKEN_PROGRAM_ID => Ok(Some(escrow::decode_token_account(&account.data)?)),
        Some(account) => Err(format!("{key} is owned by {}, not the SPL Token program", account.owner)),
    }
}

fn token_amount(rpc: &Rpc, key: &Pubkey) -> Result<Option<u64>> {
    Ok(token_account(rpc, key)?.map(|account| account.amount))
}

fn print_job(job: &JobView) {
    let address = escrow::job_pda(&job.job_id);
    let terms_ok = term_problems(job).is_empty();
    println!("job.job_id={}", hex(&job.job_id));
    println!("job.address={address}");
    println!("job.explorer={}", explorer_address(&address));
    println!("job.version={} bump={} vault_bump={}", job.version, job.bump, job.vault_bump);
    println!("job.buyer={}", job.buyer);
    println!("job.executor={}", job.executor);
    println!("job.mint={} admitted={}", job.mint, job.mint == ADMITTED_MINT);
    println!("job.amount={}", job.amount);
    println!("job.deadline_slot={}", job.deadline_slot);
    println!("job.spec_hash={}", hex(&job.spec_hash));
    println!("job.harness_hash={}", hex(&job.harness_hash));
    println!("job.image_id={}", hex(&job.image_id));
    println!("job.terms_v1_admitted={terms_ok}");
    match job.status.artifact_hash() {
        Some(artifact) => println!("job.status={} artifact_hash={}", job.status.name(), hex(&artifact)),
        None => println!("job.status={}", job.status.name()),
    }
}

fn check(rpc: &Rpc) -> Result<()> {
    preflight(rpc)?;
    let data = rpc.account(&escrow::PROGRAM_DATA_ID)?.ok_or("escrow ProgramData not found")?.data;
    let program = &data[escrow::PROGRAM_DATA_HEADER_LEN..];
    let digest = hex(&sha256(program));
    println!("check.escrow_program_data bytes={} sha256={digest}", program.len());
    if program.len() != escrow::PROGRAM_LEN || digest != escrow::PROGRAM_SHA256 {
        return Err(format!("deployed escrow bytes are not {}", escrow::PROGRAM_SHA256));
    }
    let verifier = rpc.account(&escrow::VERIFIER_ID)?.ok_or("Groth16 verifier not found")?;
    if !verifier.executable || verifier.owner != escrow::LOADER_V3_ID {
        return Err("Groth16 verifier is not an executable upgradeable program".into());
    }
    let verifier_data = escrow::decode_program_account(&verifier.data)?;
    let header = rpc
        .account_slice(&verifier_data, 0, escrow::PROGRAM_DATA_HEADER_LEN)?
        .ok_or("verifier ProgramData not found")?;
    let (_, authority) = escrow::decode_program_data_header(&header.data)?;
    if authority.is_some() {
        return Err("Groth16 verifier upgrade authority is not none".into());
    }
    println!(
        "check.verifier={} program_data={verifier_data} upgrade_authority=none",
        escrow::VERIFIER_ID
    );
    rpc.record(json!({"kind": "check", "escrow_sha256": digest, "verifier": escrow::VERIFIER_ID.to_string()}));
    println!("check=ok");
    Ok(())
}

fn job_create(rpc: &Rpc, mut args: Args) -> Result<()> {
    let buyer = load_key(&mut args, "--buyer-keypair", "buyer")?;
    let executor = parse_pubkey(&args.require("--executor")?, "--executor")?;
    let amount = args.take("--amount").map_or(Ok(DEFAULT_AMOUNT), |text| parse_u64(&text, "--amount"))?;
    let offset = args
        .take("--deadline-offset")
        .map_or(Ok(DEFAULT_DEADLINE_OFFSET), |text| parse_u64(&text, "--deadline-offset"))?;
    let job_file = args.take("--job-file").map(PathBuf::from);
    args.finish()?;
    let min_offset = escrow::MIN_DEADLINE_WINDOW_SLOTS + LANDING_MARGIN_SLOTS;
    if !(min_offset..=escrow::MAX_DEADLINE_WINDOW_SLOTS).contains(&offset) {
        return Err(format!(
            "--deadline-offset {offset} outside [{min_offset}, {}] (creation window plus landing margin)",
            escrow::MAX_DEADLINE_WINDOW_SLOTS
        ));
    }
    if amount == 0 {
        return Err("--amount must be greater than zero".into());
    }
    if executor == buyer.pubkey() {
        return Err("executor and buyer must differ".into());
    }

    preflight(rpc)?;
    let terms = escrow::admitted_terms();
    let mut job_id = [0_u8; 32];
    getrandom::getrandom(&mut job_id).map_err(|error| format!("OS random source: {error}"))?;
    if job_id == [0x11; 32] {
        return Err("random job_id collided with the fixture id; run again".into());
    }
    let job = escrow::job_pda(&job_id);
    let vault = escrow::vault_pda(&job);
    if executor == job || executor == vault {
        return Err("executor is a program account of the Job".into());
    }
    if rpc.account(&job)?.is_some() || rpc.account(&vault)?.is_some() {
        return Err(format!("job_id {} is occupied (Job {job}); nothing sent", hex(&job_id)));
    }
    let buyer_token = escrow::ata(&buyer.pubkey());
    let token = token_account(rpc, &buyer_token)?
        .ok_or_else(|| format!("buyer {} has no Test USDC account {buyer_token}", buyer.pubkey()))?;
    if token.owner != buyer.pubkey() || token.mint != ADMITTED_MINT {
        return Err(format!("{buyer_token} is not the buyer's Test USDC account"));
    }
    if token.amount < amount {
        return Err(format!(
            "buyer {} holds {} Test USDC units in {buyer_token}, needs {amount}",
            buyer.pubkey(),
            token.amount
        ));
    }
    let rent = rpc.rent_exempt_minimum(escrow::JOB_ACCOUNT_LEN)? + rpc.rent_exempt_minimum(escrow::TOKEN_ACCOUNT_LEN)?;
    let needed = rent + SIGNATURE_FEE;
    let balance = rpc.balance(&buyer.pubkey())?;
    if balance < needed {
        return Err(format!(
            "insufficient SOL: buyer {} has {balance} lamports, needs {needed} (rent {rent} + fee)",
            buyer.pubkey()
        ));
    }
    let slot = rpc.slot("confirmed")?;
    let deadline_slot = slot + offset;
    println!("create.job_id={}", hex(&job_id));
    println!("create.job={job}");
    println!("create.vault={vault}");
    println!("create.executor={executor}");
    println!("create.amount={amount}");
    println!("create.slot={slot} deadline_slot={deadline_slot} offset={offset}");
    println!("create.rent_lamports={rent}");

    let instructions = [
        escrow::create_job_ix(&buyer.pubkey(), job_id, &executor, amount, deadline_slot, &terms),
        escrow::fund_ix(&buyer.pubkey(), job_id, amount),
    ];
    let outcome = tx::run(rpc, "create+fund", &instructions, &buyer, &[], Expect::Success, &[])?;
    let created = require_job(rpc, &job_id)?;
    let vault_amount = token_amount(rpc, &vault)?;
    let expected = created.status == Status::Funded
        && created.buyer == buyer.pubkey()
        && created.executor == executor
        && created.amount == amount
        && created.deadline_slot == deadline_slot
        && term_problems(&created).is_empty()
        && vault_amount == Some(amount);
    print_job(&created);
    println!("job.vault={vault} amount={}", vault_amount.unwrap_or(0));
    if !expected {
        return Err("created Job does not hold the requested terms".into());
    }
    if let Some(path) = job_file {
        let record = json!({
            "job_id": hex(&job_id), "job": job.to_string(), "vault": vault.to_string(),
            "buyer": buyer.pubkey().to_string(), "executor": executor.to_string(),
            "mint": ADMITTED_MINT.to_string(), "amount": amount, "deadline_slot": deadline_slot,
            "create_signature": outcome.signature, "create_slot": outcome.slot,
        });
        fs::write(&path, format!("{record:#}\n")).map_err(|error| format!("{}: {error}", path.display()))?;
        println!("create.job_file={}", path.display());
    }
    Ok(())
}

fn parse_artifact(text: &str) -> Result<[u8; 32]> {
    let (input, output) = text.split_once(',').ok_or("--artifact expects INPUT,OUTPUT")?;
    let input = input.trim().parse().map_err(|_| "--artifact INPUT is not a u32")?;
    let output = output.trim().parse().map_err(|_| "--artifact OUTPUT is not a u32")?;
    Ok(escrow::artifact_hash(input, output))
}

fn job_deliver(rpc: &Rpc, mut args: Args) -> Result<()> {
    let executor = load_key(&mut args, "--executor-keypair", "executor")?;
    let job_id = job_id_arg(&mut args)?;
    let artifact_hash = match (args.take("--artifact"), args.take("--receipt")) {
        (Some(text), None) => parse_artifact(&text)?,
        (None, Some(dir)) => Receipt::load(Path::new(&dir))?.artifact_hash(),
        _ => return Err("give exactly one of --artifact or --receipt".into()),
    };
    args.finish()?;
    preflight(rpc)?;
    let job = require_job(rpc, &job_id)?;
    print_job(&job);
    let slot = rpc.slot("confirmed")?;
    let mut prechecks = Prechecks::new(Expect::Success);
    for problem in term_problems(&job) {
        prechecks.require(false, problem);
    }
    prechecks.require(job.executor == executor.pubkey(), "signer is not the Job executor (6027)");
    prechecks.require(job.status == Status::Funded, format!("Job is {}, not Funded", job.status.name()));
    prechecks.require(
        slot + LANDING_MARGIN_SLOTS <= job.deadline_slot,
        format!("slot {slot} is too close to the deadline {}", job.deadline_slot),
    );
    prechecks.finish()?;
    println!("deliver.artifact_hash={}", hex(&artifact_hash));
    let instruction = escrow::deliver_ix(&executor.pubkey(), job_id, artifact_hash);
    tx::run(rpc, "deliver", &[instruction], &executor, &[], Expect::Success, &[])?;
    let delivered = require_job(rpc, &job_id)?;
    print_job(&delivered);
    if delivered.status != (Status::Delivered { artifact_hash }) {
        return Err("Job is not Delivered with the committed artifact".into());
    }
    Ok(())
}

fn watched(job: &JobView) -> Vec<Pubkey> {
    let address = escrow::job_pda(&job.job_id);
    vec![
        address,
        escrow::vault_pda(&address),
        escrow::ata(&job.buyer),
        escrow::ata(&job.executor),
    ]
}

/// Prepends `CreateIdempotent` when the canonical account of `owner` is
/// missing, and checks it otherwise.
fn destination(
    rpc: &Rpc,
    owner: &Pubkey,
    payer: &Pubkey,
    instructions: &mut Vec<Instruction>,
    prechecks: &mut Prechecks,
) -> Result<Option<u64>> {
    let address = escrow::ata(owner);
    match token_account(rpc, &address)? {
        None => {
            println!("destination={address} missing; prepended CreateIdempotent");
            instructions.push(escrow::create_ata_idempotent_ix(payer, owner));
            Ok(None)
        }
        Some(account) => {
            prechecks.require(
                account.owner == *owner && account.mint == ADMITTED_MINT,
                format!("{address} is not the canonical Test USDC account of {owner}"),
            );
            println!("destination={address} owner={owner} amount={}", account.amount);
            Ok(Some(account.amount))
        }
    }
}

fn job_settle(rpc: &Rpc, mut args: Args) -> Result<()> {
    let job_id = job_id_arg(&mut args)?;
    let receipt_dir = PathBuf::from(args.require("--receipt")?);
    let expect = args.take("--expect-error").map_or(Ok(Expect::Success), |text| Expect::parse(&text))?;
    let tamper = args.flag("--tamper-seal");
    let deliver = args.flag("--deliver");
    if tamper && !expect.is_failure() {
        return Err("--tamper-seal is only for negative runs with --expect-error".into());
    }
    let payer = if deliver {
        load_key(&mut args, "--executor-keypair", "executor")?
    } else {
        load_key(&mut args, "--payer-keypair", "payer")?
    };
    args.finish()?;
    let receipt = Receipt::load(&receipt_dir)?;
    for line in receipt.describe() {
        println!("{line}");
    }
    preflight(rpc)?;
    let job = require_job(rpc, &job_id)?;
    print_job(&job);
    let slot = rpc.slot("confirmed")?;
    println!("settle.slot={slot}");

    let mut prechecks = Prechecks::new(expect);
    for problem in term_problems(&job) {
        prechecks.require(false, problem);
    }
    prechecks.require(receipt.selector == escrow::GROTH16_SELECTOR, "seal selector is not 73c457ba (6033)");
    prechecks.require(
        receipt.job_id() == job_id,
        format!("journal belongs to job_id {} (6014)", hex(&receipt.job_id())),
    );
    prechecks.require(
        receipt.decoded.spec_hash().into_bytes() == job.spec_hash
            && receipt.decoded.harness_hash().into_bytes() == job.harness_hash,
        "journal spec or harness differs from the Job (6015/6016)",
    );
    prechecks.require(
        receipt.image_id() == job.image_id && receipt.image_id() == ADMITTED_IMAGE_ID_V1,
        "journal image_id is not the admitted image of the Job (6018)",
    );
    let verdict = receipt.verdict();
    let party = match verdict {
        Verdict::Pass => job.executor,
        Verdict::Fail => job.buyer,
    };
    let mut instructions = Vec::new();
    if deliver {
        prechecks.require(payer.pubkey() == job.executor, "signer is not the Job executor (6027)");
        prechecks.require(job.status == Status::Funded, format!("Job is {}, not Funded", job.status.name()));
        prechecks.require(
            slot + LANDING_MARGIN_SLOTS <= job.deadline_slot,
            format!("slot {slot} is too close to the deadline {} to deliver", job.deadline_slot),
        );
    } else {
        match job.status {
            Status::Delivered { artifact_hash } => prechecks.require(
                artifact_hash == receipt.artifact_hash(),
                "journal artifact is not the delivered artifact (6017)",
            ),
            Status::Released { .. } => prechecks.require(false, "Job is already Released (6007)"),
            Status::RefundedOnFail { .. } | Status::RefundedOnTimeout => {
                prechecks.require(false, "Job is already refunded (6008)")
            }
            other => prechecks.require(false, format!("Job is {}, not Delivered", other.name())),
        }
    }
    if verdict == Verdict::Pass {
        prechecks.require(
            slot + LANDING_MARGIN_SLOTS <= job.deadline_slot,
            format!("slot {slot} is too close to the deadline {} for a release", job.deadline_slot),
        );
    }
    let before = destination(rpc, &party, &payer.pubkey(), &mut instructions, &mut prechecks)?;
    if deliver {
        instructions.push(escrow::deliver_ix(&payer.pubkey(), job_id, receipt.artifact_hash()));
    }
    let mut seal = receipt.seal();
    if tamper {
        seal.pi_c[10] ^= 0x01;
        println!("settle.tampered=pi_c[10]^=1");
    }
    let recipient = escrow::ata(&party);
    let (label, instruction) = match verdict {
        Verdict::Pass => ("release", escrow::release_ix(job_id, &recipient, &receipt.journal, &seal)),
        Verdict::Fail => ("refund_on_fail", escrow::refund_on_fail_ix(job_id, &recipient, &receipt.journal, &seal)),
    };
    instructions.push(instruction);
    let label = if deliver { format!("deliver+{label}") } else { label.to_string() };
    println!("settle.instruction={label} recipient={recipient}");
    prechecks.finish()?;

    let watch = if expect.is_failure() { watched(&job) } else { Vec::new() };
    let outcome = tx::run(rpc, &label, &instructions, &payer, &[], expect, &watch)?;
    if expect.is_failure() {
        return Ok(());
    }
    if !outcome.verifier_invoked {
        return Err("settlement landed without invoking the Groth16 verifier".into());
    }
    let settled = require_job(rpc, &job_id)?;
    print_job(&settled);
    let artifact_hash = receipt.artifact_hash();
    let expected_status = match verdict {
        Verdict::Pass => Status::Released { artifact_hash },
        Verdict::Fail => Status::RefundedOnFail { artifact_hash },
    };
    let vault = token_amount(rpc, &escrow::vault_pda(&escrow::job_pda(&job_id)))?;
    let after = token_amount(rpc, &recipient)?;
    println!("settle.vault_amount={}", vault.unwrap_or(0));
    println!("settle.recipient_amount_before={} after={}", before.unwrap_or(0), after.unwrap_or(0));
    if settled.status != expected_status || vault != Some(0) || after != Some(before.unwrap_or(0) + job.amount) {
        return Err("settlement state or balances differ from the expected result".into());
    }
    Ok(())
}

fn job_refund_timeout(rpc: &Rpc, mut args: Args) -> Result<()> {
    let job_id = job_id_arg(&mut args)?;
    let expect = args.take("--expect-error").map_or(Ok(Expect::Success), |text| Expect::parse(&text))?;
    let wait = args.flag("--wait");
    let payer = load_key(&mut args, "--payer-keypair", "payer")?;
    args.finish()?;
    if wait && expect.is_failure() {
        return Err("--wait is for the positive refund only".into());
    }
    preflight(rpc)?;
    let job = require_job(rpc, &job_id)?;
    print_job(&job);
    let mut slot = rpc.slot("confirmed")?;
    while wait && slot <= job.deadline_slot {
        let remaining = job.deadline_slot - slot + 1;
        println!("refund.wait slot={slot} deadline_slot={} slots_to_go={remaining}", job.deadline_slot);
        sleep(Duration::from_secs((remaining * 2 / 5).clamp(2, 30)));
        slot = rpc.slot("confirmed")?;
    }
    println!("refund.slot={slot}");
    let mut prechecks = Prechecks::new(expect);
    for problem in term_problems(&job) {
        prechecks.require(false, problem);
    }
    prechecks.require(
        matches!(job.status, Status::Funded | Status::Delivered { .. }),
        format!("Job is {}, not Funded or Delivered", job.status.name()),
    );
    prechecks.require(
        slot > job.deadline_slot,
        format!("deadline not reached: slot {slot} <= deadline_slot {} (6021)", job.deadline_slot),
    );
    let mut instructions = Vec::new();
    let before = destination(rpc, &job.buyer, &payer.pubkey(), &mut instructions, &mut prechecks)?;
    let buyer_token = escrow::ata(&job.buyer);
    instructions.push(escrow::refund_on_timeout_ix(job_id, &buyer_token));
    prechecks.finish()?;

    let watch = if expect.is_failure() { watched(&job) } else { Vec::new() };
    tx::run(rpc, "refund_on_timeout", &instructions, &payer, &[], expect, &watch)?;
    if expect.is_failure() {
        return Ok(());
    }
    let refunded = require_job(rpc, &job_id)?;
    print_job(&refunded);
    let vault = token_amount(rpc, &escrow::vault_pda(&escrow::job_pda(&job_id)))?;
    let after = token_amount(rpc, &buyer_token)?;
    println!("refund.vault_amount={}", vault.unwrap_or(0));
    println!("refund.buyer_amount_before={} after={}", before.unwrap_or(0), after.unwrap_or(0));
    if refunded.status != Status::RefundedOnTimeout || vault != Some(0) || after != Some(before.unwrap_or(0) + job.amount) {
        return Err("refund state or balances differ from the expected result".into());
    }
    Ok(())
}

fn job_show(rpc: &Rpc, mut args: Args) -> Result<()> {
    let job_id = job_id_arg(&mut args)?;
    args.finish()?;
    let genesis = rpc.genesis_hash()?;
    if genesis != escrow::DEVNET_GENESIS_HASH {
        return Err(format!("RPC {} is not Solana devnet (genesis {genesis})", rpc.url()));
    }
    let job = require_job(rpc, &job_id)?;
    print_job(&job);
    let address = escrow::job_pda(&job_id);
    let vault = escrow::vault_pda(&address);
    match token_account(rpc, &vault)? {
        Some(account) => println!(
            "vault.address={vault} mint={} authority={} authority_is_job_pda={} delegate={:?} close_authority={:?} amount={}",
            account.mint,
            account.owner,
            account.owner == address,
            account.delegate,
            account.close_authority,
            account.amount
        ),
        None => println!("vault.address={vault} missing"),
    }
    for (role, owner) in [("buyer", job.buyer), ("executor", job.executor)] {
        let ata = escrow::ata(&owner);
        println!(
            "{role}.ata={ata} test_usdc={} sol_lamports={}",
            token_amount(rpc, &ata)?.map_or("missing".into(), |amount| amount.to_string()),
            rpc.balance(&owner)?
        );
    }
    let slot = rpc.slot("confirmed")?;
    println!(
        "slot={slot} deadline_slot={} past_deadline={}",
        job.deadline_slot,
        slot > job.deadline_slot
    );
    Ok(())
}

fn run(raw: Vec<String>) -> Result<()> {
    let mut args = Args::parse(raw)?;
    let url = args.take("--rpc-url").unwrap_or_else(|| DEFAULT_RPC_URL.to_string());
    let log = args.take("--log").map(PathBuf::from);
    let rpc = Rpc::new(&url, log.as_deref())?;
    let words: Vec<String> = std::mem::take(&mut args.positional);
    match words.iter().map(String::as_str).collect::<Vec<_>>().as_slice() {
        ["check"] => {
            args.finish()?;
            check(&rpc)
        }
        ["job", "create"] => job_create(&rpc, args),
        ["job", "deliver"] => job_deliver(&rpc, args),
        ["job", "settle"] => job_settle(&rpc, args),
        ["job", "refund-timeout"] => job_refund_timeout(&rpc, args),
        ["job", "show"] => job_show(&rpc, args),
        _ => Err(USAGE.into()),
    }
}

fn main() -> ExitCode {
    match run(std::env::args().skip(1).collect()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}
