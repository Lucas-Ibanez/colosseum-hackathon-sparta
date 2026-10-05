//! VeriCode prover (gate D7).
//!
//! Proves the admitted guest for one Job with the local RISC Zero prover and
//! compresses the receipt to Groth16 for the devnet escrow. The guest is the
//! preserved deterministic D1c2b build, versioned in
//! `artifacts/vericode-guest.bin` and embedded at compile time; every
//! operation first checks its SHA-256, size and ImageID and aborts on any
//! difference. The guest is never rebuilt here.
//!
//! Receipt directory (input of `vericode job settle`):
//! - `composite.receipt`: bincode `Receipt`, `Composite`;
//! - `groth16.receipt`: bincode `Receipt`, `Groth16`;
//! - `selector` (4 B), `seal` (256 B, raw Groth16 seal, `pi_a` NOT negated),
//!   `image_id` (32 B), `journal` (165 B), `journal_digest` (32 B, SHA-256 of
//!   the journal).

use std::{
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::Instant,
};

use risc0_zkvm::{
    compute_image_id, default_executor, default_prover,
    sha::{Digestible, Sha256},
    Digest, ExecutorEnv, Groth16ReceiptVerifierParameters, InnerReceipt, ProverOpts, Receipt,
};
use vericode_core::{evaluate_restricted_artifact, ImageId, JobId, RestrictedArtifactV1};

pub type AnyError = Box<dyn Error + Send + Sync>;

/// Combined method binary of the deterministic D1c2b guest (D1c2b.3h).
pub const GUEST: &[u8] = include_bytes!("../artifacts/vericode-guest.bin");
pub const GUEST_SIZE: usize = 180_300;
pub const GUEST_SHA256: &str = "e09ba8cf16f7e36cb92e10656b7574c598f3e88fe089f02bc747a4bb00c078f5";
/// `ADMITTED_IMAGE_ID_V1` of the escrow program.
pub const ADMITTED_IMAGE_ID: &str = "4da06f90da75ec8980c943ce017d69c48370fddbf3aa27689d375d78fac0fb1a";
/// Job of the versioned test fixtures; never proved for devnet.
pub const RESERVED_JOB_ID: [u8; 32] = [0x11; 32];
/// Tag that `risc0-groth16 3.0.2` passes to `docker run`, and the local image
/// digest that the shim substitutes for it.
pub const GROTH16_PROVER_TAG: &str = "risczero/risc0-groth16-prover:v2025-04-03.1";
pub const GROTH16_PROVER_IMAGE: &str =
    "risczero/risc0-groth16-prover@sha256:7f173963196570b7a71816ed70565a4579264c5d2e3e0ecb028102538ad0e331";

pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

pub fn unhex32(text: &str) -> Result<[u8; 32], AnyError> {
    if text.len() != 64 || !text.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(format!("expected 64 hex digits, got {:?}", text).into());
    }
    let mut out = [0_u8; 32];
    for (index, byte) in out.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&text[2 * index..2 * index + 2], 16)?;
    }
    Ok(out)
}

pub fn sha256(bytes: &[u8]) -> String {
    hex(risc0_zkvm::sha::Impl::hash_bytes(bytes).as_bytes())
}

fn kind(inner: &InnerReceipt) -> &'static str {
    match inner {
        InnerReceipt::Composite(_) => "Composite",
        InnerReceipt::Succinct(_) => "Succinct",
        InnerReceipt::Groth16(_) => "Groth16",
        InnerReceipt::Fake(_) => "Fake",
        _ => "Unknown",
    }
}

/// Selector of the Groth16 verifier parameters: the first 4 bytes of the
/// digest of `Groth16ReceiptVerifierParameters::default()`.
pub fn selector() -> [u8; 4] {
    let digest = Groth16ReceiptVerifierParameters::default().digest();
    digest.as_bytes()[..4].try_into().expect("4 bytes")
}

/// Checks the embedded guest and returns its ImageID, which must be the
/// admitted one.
pub fn admitted_guest() -> Result<Digest, AnyError> {
    if GUEST.len() != GUEST_SIZE {
        return Err(format!("guest has {} bytes, expected {GUEST_SIZE}", GUEST.len()).into());
    }
    let digest = sha256(GUEST);
    if digest != GUEST_SHA256 {
        return Err(format!("guest SHA-256 {digest} is not {GUEST_SHA256}").into());
    }
    let image_id = compute_image_id(GUEST)?;
    if image_id.to_string() != ADMITTED_IMAGE_ID {
        return Err(format!("ImageID {image_id} is not the admitted {ADMITTED_IMAGE_ID}").into());
    }
    Ok(image_id)
}

fn image_id_bytes(image_id: &Digest) -> [u8; 32] {
    image_id.as_bytes().try_into().expect("32 bytes")
}

/// The restricted artifact `(input, claimed_output)` in its 12-byte wire format.
pub fn artifact(input: u32, claimed_output: u32) -> Result<Vec<u8>, AnyError> {
    RestrictedArtifactV1::new(input, claimed_output)
        .encode_candidate()
        .map_err(|error| format!("encode artifact: {error:?}").into())
}

/// Guest input frame of `zkvm/host`: `job_id ‖ artifact (12 B) ‖ image_id`.
pub fn frame(job_id: &[u8; 32], artifact: &[u8], image_id: &[u8; 32]) -> Vec<u8> {
    let mut frame = Vec::with_capacity(32 + artifact.len() + 32);
    frame.extend_from_slice(job_id);
    frame.extend_from_slice(artifact);
    frame.extend_from_slice(image_id);
    frame
}

/// The journal the core produces for this Job and artifact. Invalid
/// artifacts (for example an input above the specification limit) are
/// errors, never a `FAIL` verdict.
pub fn expected_journal(job_id: &[u8; 32], image_id: &[u8; 32], artifact: &[u8]) -> Result<Vec<u8>, AnyError> {
    let journal = evaluate_restricted_artifact(JobId::new(*job_id), ImageId::new(*image_id), artifact)
        .map_err(|error| format!("evaluate artifact: {error:?}"))?;
    journal
        .encode_candidate()
        .map_err(|error| format!("encode journal: {error:?}").into())
}

/// Checks the Job and the artifact before any proving work.
pub fn check_inputs(job_id: &[u8; 32], input: u32, claimed_output: u32) -> Result<Vec<u8>, AnyError> {
    if *job_id == RESERVED_JOB_ID {
        return Err("job_id 0x11… is reserved for the versioned test fixtures".into());
    }
    let artifact = artifact(input, claimed_output)?;
    let image_id = hex_to_image(ADMITTED_IMAGE_ID);
    expected_journal(job_id, &image_id, &artifact)?;
    Ok(artifact)
}

fn hex_to_image(text: &str) -> [u8; 32] {
    unhex32(text).expect("constant ImageID")
}

/// Executes the guest without proving and returns its journal.
pub fn execute(job_id: &[u8; 32], artifact: &[u8]) -> Result<Vec<u8>, AnyError> {
    let image_id = admitted_guest()?;
    let frame = frame(job_id, artifact, &image_id_bytes(&image_id));
    let env = ExecutorEnv::builder().write_slice(&frame).build()?;
    Ok(default_executor().execute(env, GUEST)?.journal.bytes)
}

fn reject_wrong_image(label: &str, receipt: &Receipt, image_id: Digest) -> Result<(), AnyError> {
    let mut wrong = image_id_bytes(&image_id);
    wrong[0] ^= 0xff;
    if receipt.verify(Digest::from(wrong)).is_ok() {
        return Err(format!("{label}: verified against a wrong ImageID").into());
    }
    println!("{label}.negative.wrong_image=rejected");
    Ok(())
}

fn print_guest(image_id: &Digest) {
    println!("guest.bytes={}", GUEST.len());
    println!("guest.sha256={}", sha256(GUEST));
    println!("guest.image_id={image_id}");
}

/// `check`: the embedded guest is the admitted build.
pub fn check() -> Result<(), AnyError> {
    let image_id = admitted_guest()?;
    print_guest(&image_id);
    println!("guest.admitted=true");
    println!("groth16.selector={}", hex(&selector()));
    Ok(())
}

/// `prove`: Composite receipt of the admitted guest for one Job and artifact.
pub fn prove(job_id_hex: &str, input: u32, claimed_output: u32, dir: &Path) -> Result<(), AnyError> {
    let job_id = unhex32(job_id_hex)?;
    let artifact = check_inputs(&job_id, input, claimed_output)?;
    let image_id = admitted_guest()?;
    print_guest(&image_id);
    let image = image_id_bytes(&image_id);
    let expected = expected_journal(&job_id, &image, &artifact)?;
    let frame = frame(&job_id, &artifact, &image);
    println!("prove.job_id={}", hex(&job_id));
    println!("prove.artifact=({input},{claimed_output}) hex={}", hex(&artifact));
    println!("prove.frame_bytes={}", frame.len());

    let started = Instant::now();
    let env = ExecutorEnv::builder().write_slice(&frame).build()?;
    let receipt = default_prover().prove(env, GUEST)?.receipt;
    println!("prove.seconds={:.1}", started.elapsed().as_secs_f64());
    println!("prove.receipt_type={}", kind(&receipt.inner));
    if !matches!(receipt.inner, InnerReceipt::Composite(_)) {
        return Err(format!("expected a Composite receipt, got {}", kind(&receipt.inner)).into());
    }
    receipt.verify(image_id)?;
    println!("prove.local_verify=ok");
    reject_wrong_image("prove", &receipt, image_id)?;
    if receipt.journal.bytes != expected {
        return Err("journal differs from the core evaluation".into());
    }
    println!("prove.journal_equal_to_core=true");
    println!("prove.verdict={}", verdict_name(&receipt.journal.bytes));
    println!("prove.journal_hex={}", hex(&receipt.journal.bytes));

    let encoded = bincode::serialize(&receipt)?;
    fs::create_dir_all(dir)?;
    fs::write(dir.join("composite.receipt"), &encoded)?;
    println!("prove.composite_receipt_bytes={}", encoded.len());
    println!("prove.composite_receipt_sha256={}", sha256(&encoded));
    Ok(())
}

fn verdict_name(journal: &[u8]) -> &'static str {
    match journal.last() {
        Some(0) => "PASS",
        Some(1) => "FAIL",
        _ => "INVALID",
    }
}

/// Runs `docker` with `args` and returns whether it succeeded.
fn docker_ok(args: &[&str]) -> bool {
    Command::new("docker")
        .args(args)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Routes the `docker run` of `risc0-groth16` through the versioned shim:
/// local image by digest, `--pull=never`, `--network=none`.
fn use_docker_shim(dir: &Path) -> Result<PathBuf, AnyError> {
    let shim_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("docker-shim");
    if !shim_dir.join("docker").is_file() {
        return Err(format!("missing Docker shim {}", shim_dir.join("docker").display()).into());
    }
    let path = env::var("PATH").unwrap_or_default();
    env::set_var("PATH", format!("{}:{path}", shim_dir.display()));
    let log = dir.join("docker-shim.log");
    env::set_var("VERICODE_DOCKER_SHIM_LOG", &log);
    if !docker_ok(&["image", "inspect", "--format", "{{.Id}}", GROTH16_PROVER_IMAGE]) {
        return Err(format!(
            "local image {GROTH16_PROVER_IMAGE} not found; it is never pulled by the prover"
        )
        .into());
    }
    println!("compress.docker_image={GROTH16_PROVER_IMAGE}");
    println!("compress.docker_shim={}", shim_dir.display());
    Ok(log)
}

/// `compress`: Groth16 receipt of `<dir>/composite.receipt` and the vector
/// files that the CLI submits.
pub fn compress(dir: &Path) -> Result<(), AnyError> {
    let image_id = admitted_guest()?;
    print_guest(&image_id);
    let dir = fs::canonicalize(dir)?;
    let composite: Receipt = bincode::deserialize(&fs::read(dir.join("composite.receipt"))?)?;
    println!("compress.input_receipt_type={}", kind(&composite.inner));
    if !matches!(composite.inner, InnerReceipt::Composite(_)) {
        return Err("input is not a Composite receipt".into());
    }
    composite.verify(image_id)?;
    println!("compress.input_local_verify=ok");

    let shim_log = use_docker_shim(&dir)?;
    if env::var_os("RISC0_WORK_DIR").is_none() {
        let work = dir.join("groth16-work");
        fs::create_dir_all(&work)?;
        env::set_var("RISC0_WORK_DIR", &work);
    }
    println!(
        "compress.work_dir={}",
        env::var("RISC0_WORK_DIR").unwrap_or_default()
    );

    let started = Instant::now();
    let groth16 = default_prover().compress(&ProverOpts::groth16(), &composite)?;
    println!("compress.seconds={:.1}", started.elapsed().as_secs_f64());
    println!("compress.receipt_type={}", kind(&groth16.inner));
    let inner = match &groth16.inner {
        InnerReceipt::Groth16(inner) => inner,
        other => return Err(format!("expected Groth16, got {}", kind(other)).into()),
    };
    groth16.verify(image_id)?;
    println!("compress.local_verify=ok");
    reject_wrong_image("compress", &groth16, image_id)?;
    if groth16.journal.bytes != composite.journal.bytes {
        return Err("Groth16 journal differs from the Composite journal".into());
    }
    println!("compress.journal_equal_to_composite=true");
    let params = Groth16ReceiptVerifierParameters::default().digest();
    if inner.verifier_parameters != params {
        return Err(format!("verifier parameters {} differ from default {params}", inner.verifier_parameters).into());
    }
    if inner.seal.len() != 256 {
        return Err(format!("seal has {} bytes", inner.seal.len()).into());
    }

    let encoded = bincode::serialize(&groth16)?;
    let journal_digest = groth16.journal.digest();
    fs::write(dir.join("groth16.receipt"), &encoded)?;
    fs::write(dir.join("selector"), selector())?;
    fs::write(dir.join("seal"), &inner.seal)?;
    fs::write(dir.join("image_id"), image_id.as_bytes())?;
    fs::write(dir.join("journal"), &groth16.journal.bytes)?;
    fs::write(dir.join("journal_digest"), journal_digest.as_bytes())?;
    println!("compress.groth16_receipt_bytes={}", encoded.len());
    println!("compress.groth16_receipt_sha256={}", sha256(&encoded));
    println!("compress.selector={}", hex(&selector()));
    println!("compress.verifier_parameters={params}");
    println!("compress.image_id={image_id}");
    println!("compress.verdict={}", verdict_name(&groth16.journal.bytes));
    println!("compress.journal_hex={}", hex(&groth16.journal.bytes));
    println!("compress.journal_sha256={}", sha256(&groth16.journal.bytes));
    println!("compress.journal_digest={journal_digest}");
    println!("compress.seal_sha256={}", sha256(&inner.seal));
    if let Ok(line) = fs::read_to_string(&shim_log) {
        for entry in line.lines() {
            println!("compress.docker_run={entry}");
        }
    }
    Ok(())
}

/// `verify`: re-verifies `<dir>/groth16.receipt` against the admitted ImageID
/// and checks that every vector file matches it.
pub fn verify(dir: &Path) -> Result<(), AnyError> {
    let image_id = admitted_guest()?;
    let receipt: Receipt = bincode::deserialize(&fs::read(dir.join("groth16.receipt"))?)?;
    let inner = match &receipt.inner {
        InnerReceipt::Groth16(inner) => inner,
        other => return Err(format!("expected Groth16, got {}", kind(other)).into()),
    };
    receipt.verify(image_id)?;
    println!("verify.receipt_type=Groth16");
    println!("verify.local_verify=ok");
    reject_wrong_image("verify", &receipt, image_id)?;
    let read = |name: &str| fs::read(dir.join(name));
    let checks: [(&str, Vec<u8>); 5] = [
        ("selector", selector().to_vec()),
        ("seal", inner.seal.clone()),
        ("image_id", image_id.as_bytes().to_vec()),
        ("journal", receipt.journal.bytes.clone()),
        ("journal_digest", receipt.journal.digest().as_bytes().to_vec()),
    ];
    for (name, expected) in checks {
        if read(name)? != expected {
            return Err(format!("{name} does not match groth16.receipt").into());
        }
        println!("verify.{name}=matches");
    }
    let journal = &receipt.journal.bytes;
    if journal.len() != 165 {
        return Err(format!("journal has {} bytes", journal.len()).into());
    }
    println!("verify.job_id={}", hex(&journal[4..36]));
    println!("verify.artifact_hash={}", hex(&journal[100..132]));
    println!("verify.verdict={}", verdict_name(journal));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use vericode_core::{hash_restricted_artifact, JournalV1, Verdict};

    const JOB: [u8; 32] = [0x5a; 32];

    #[test]
    fn embedded_guest_is_the_admitted_build() {
        assert_eq!(GUEST.len(), GUEST_SIZE);
        assert_eq!(sha256(GUEST), GUEST_SHA256);
        let image_id = admitted_guest().unwrap();
        assert_eq!(image_id.to_string(), ADMITTED_IMAGE_ID);
        assert_eq!(hex(&selector()), "73c457ba");
    }

    #[test]
    fn frame_is_job_id_artifact_image_id() {
        let image = hex_to_image(ADMITTED_IMAGE_ID);
        let artifact = artifact(7, 14).unwrap();
        assert_eq!(artifact, [1, 0, 0, 0, 7, 0, 0, 0, 14, 0, 0, 0]);
        let frame = frame(&JOB, &artifact, &image);
        assert_eq!(frame.len(), 76);
        assert_eq!(&frame[..32], &JOB);
        assert_eq!(&frame[32..44], artifact.as_slice());
        assert_eq!(&frame[44..], &image);
    }

    #[test]
    fn executed_journal_is_the_core_journal() {
        let image = hex_to_image(ADMITTED_IMAGE_ID);
        for (output, verdict) in [(14, Verdict::Pass), (15, Verdict::Fail)] {
            let artifact = artifact(7, output).unwrap();
            let journal = execute(&JOB, &artifact).unwrap();
            assert_eq!(journal, expected_journal(&JOB, &image, &artifact).unwrap());
            let decoded = JournalV1::decode_candidate(&journal).unwrap();
            assert_eq!(decoded.verdict(), verdict);
            assert_eq!(decoded.job_id(), JobId::new(JOB));
            assert_eq!(
                decoded.artifact_hash(),
                hash_restricted_artifact(&RestrictedArtifactV1::new(7, output)).unwrap()
            );
            assert_eq!(decoded.image_id(), ImageId::new(image));
        }
    }

    #[test]
    fn reserved_job_and_invalid_artifacts_are_rejected_before_proving() {
        assert!(check_inputs(&RESERVED_JOB_ID, 7, 14).is_err());
        assert!(check_inputs(&JOB, 1_000_001, 2_000_002).is_err());
        assert!(check_inputs(&JOB, 21, 42).is_ok());
        assert!(check_inputs(&JOB, 21, 43).is_ok(), "a wrong claim is a normal FAIL");
        assert!(unhex32("11").is_err());
        assert!(unhex32(&"zz".repeat(32)).is_err());
    }
}
