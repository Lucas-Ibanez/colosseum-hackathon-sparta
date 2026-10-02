use std::{
    env,
    error::Error,
    fmt::Debug,
    fs,
    io::{Error as IoError, ErrorKind},
    path::Path,
};

use risc0_zkvm::{
    compute_image_id, default_executor, default_prover, Digest, ExecutorEnv,
    InnerReceipt, Receipt,
};
use vericode_core::{
    evaluate_restricted_artifact, ImageId, JobId, JournalV1,
    JournalV1Commitments, JournalValidationError, RestrictedArtifactV1,
    Verdict, JOURNAL_V1_CANDIDATE_WIRE_SIZE,
    RESTRICTED_ARTIFACT_V1_WIRE_SIZE,
};
use vericode_methods::{VERICODE_GUEST_ELF, VERICODE_GUEST_ID};

type AnyError = Box<dyn Error + Send + Sync>;

const JOB_ID_BYTES: [u8; 32] = [0x11; 32];
const DIFFERENT_JOB_ID_BYTES: [u8; 32] = [0x22; 32];
const JOB_ID_SIZE: usize = 32;
const IMAGE_ID_SIZE: usize = 32;
const GUEST_INPUT_SIZE: usize =
    JOB_ID_SIZE + RESTRICTED_ARTIFACT_V1_WIRE_SIZE + IMAGE_ID_SIZE;

fn main() -> Result<(), AnyError> {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("execute") => run_execute(),
        Some("prove") => {
            let output_dir = args
                .next()
                .ok_or_else(|| invalid_input("prove requires an output directory"))?;
            if args.next().is_some() {
                return Err(invalid_input("unexpected extra command-line argument"));
            }
            run_prove(Path::new(&output_dir))
        }
        _ => Err(invalid_input(
            "usage: vericode-host execute | vericode-host prove <output-dir>",
        )),
    }
}

fn run_execute() -> Result<(), AnyError> {
    let image_id = effective_image_id()?;
    println!("image_id={image_id}");
    println!("elf_bytes={}", VERICODE_GUEST_ELF.len());

    for scenario in [Scenario::pass(), Scenario::fail()] {
        let journal_bytes = execute_guest(scenario, digest_bytes(&image_id))?;
        validate_expected_journal(&journal_bytes, scenario, digest_bytes(&image_id))?;
        println!(
            "execute.{}.journal_hex={}",
            scenario.name,
            encode_hex(&journal_bytes)
        );
    }

    Ok(())
}

fn run_prove(output_dir: &Path) -> Result<(), AnyError> {
    fs::create_dir_all(output_dir)?;
    let image_id = effective_image_id()?;
    let image_id_bytes = digest_bytes(&image_id);

    println!("image_id={image_id}");
    println!("elf_bytes={}", VERICODE_GUEST_ELF.len());

    let pass = prove_guest(Scenario::pass(), image_id_bytes)?;
    pass.verify(image_id)?;
    validate_expected_journal(
        &pass.journal.bytes,
        Scenario::pass(),
        image_id_bytes,
    )?;
    record_receipt(output_dir, "pass", &pass)?;

    let fail = prove_guest(Scenario::fail(), image_id_bytes)?;
    fail.verify(image_id)?;
    validate_expected_journal(
        &fail.journal.bytes,
        Scenario::fail(),
        image_id_bytes,
    )?;
    record_receipt(output_dir, "fail", &fail)?;

    let mut wrong_verify_id = image_id_bytes;
    wrong_verify_id[0] ^= 0xff;
    if pass.verify(Digest::from(wrong_verify_id)).is_ok() {
        return Err(invalid_data(
            "receipt unexpectedly verified against an incorrect ImageID",
        ));
    }
    println!("negative.wrong_verify_image=rejected");

    let wrong_image_receipt = prove_guest(Scenario::pass(), wrong_verify_id)?;
    wrong_image_receipt.verify(image_id)?;
    let wrong_image_journal =
        decode_exact_journal(&wrong_image_receipt.journal.bytes)?;
    let expected = expected_journal(Scenario::pass(), image_id_bytes)?;
    let commitments = commitments_for(&expected);
    match wrong_image_journal.validate_against(&commitments) {
        Err(JournalValidationError::ImageIdMismatch) => {
            println!("negative.supplied_wrong_journal_image=rejected");
        }
        other => {
            return Err(invalid_data(format!(
                "wrong supplied ImageID produced unexpected validation result: {other:?}"
            )));
        }
    }
    record_receipt(output_dir, "wrong-image", &wrong_image_receipt)?;

    let pass_journal = decode_exact_journal(&pass.journal.bytes)?;
    let wrong_job_commitments = JournalV1Commitments::new(
        JobId::new(DIFFERENT_JOB_ID_BYTES),
        pass_journal.spec_hash(),
        pass_journal.harness_hash(),
        pass_journal.artifact_hash(),
        pass_journal.image_id(),
    );
    match pass_journal.validate_against(&wrong_job_commitments) {
        Err(JournalValidationError::JobIdMismatch) => {
            println!("negative.different_job_id=rejected");
        }
        other => {
            return Err(invalid_data(format!(
                "different JobId produced unexpected validation result: {other:?}"
            )));
        }
    }

    Ok(())
}

#[derive(Clone, Copy)]
struct Scenario {
    name: &'static str,
    claimed_output: u32,
    expected_verdict: Verdict,
}

impl Scenario {
    const fn pass() -> Self {
        Self {
            name: "pass",
            claimed_output: 14,
            expected_verdict: Verdict::Pass,
        }
    }

    const fn fail() -> Self {
        Self {
            name: "fail",
            claimed_output: 15,
            expected_verdict: Verdict::Fail,
        }
    }
}

fn effective_image_id() -> Result<Digest, AnyError> {
    let computed = compute_image_id(VERICODE_GUEST_ELF)?;
    let generated = Digest::from(VERICODE_GUEST_ID);
    if computed != generated {
        return Err(invalid_data(format!(
            "computed ImageID {computed} differs from generated ImageID {generated}"
        )));
    }
    Ok(computed)
}

fn digest_bytes(digest: &Digest) -> [u8; IMAGE_ID_SIZE] {
    *digest.as_bytes()
}

fn canonical_artifact(scenario: Scenario) -> Result<Vec<u8>, AnyError> {
    map_debug(
        RestrictedArtifactV1::new(7, scenario.claimed_output).encode_candidate(),
        "encode canonical restricted artifact",
    )
}

fn guest_input(
    scenario: Scenario,
    supplied_image_id: [u8; IMAGE_ID_SIZE],
) -> Result<[u8; GUEST_INPUT_SIZE], AnyError> {
    let artifact = canonical_artifact(scenario)?;
    if artifact.len() != RESTRICTED_ARTIFACT_V1_WIRE_SIZE {
        return Err(invalid_data("canonical artifact has an unexpected length"));
    }

    let mut input = [0_u8; GUEST_INPUT_SIZE];
    input[..JOB_ID_SIZE].copy_from_slice(&JOB_ID_BYTES);
    let artifact_end = JOB_ID_SIZE + RESTRICTED_ARTIFACT_V1_WIRE_SIZE;
    input[JOB_ID_SIZE..artifact_end].copy_from_slice(&artifact);
    input[artifact_end..].copy_from_slice(&supplied_image_id);
    Ok(input)
}

fn execute_guest(
    scenario: Scenario,
    supplied_image_id: [u8; IMAGE_ID_SIZE],
) -> Result<Vec<u8>, AnyError> {
    let input = guest_input(scenario, supplied_image_id)?;
    let mut builder = ExecutorEnv::builder();
    builder.write_slice(&input);
    let session =
        default_executor().execute(builder.build()?, VERICODE_GUEST_ELF)?;
    Ok(session.journal.bytes)
}

fn prove_guest(
    scenario: Scenario,
    supplied_image_id: [u8; IMAGE_ID_SIZE],
) -> Result<Receipt, AnyError> {
    let input = guest_input(scenario, supplied_image_id)?;
    let mut builder = ExecutorEnv::builder();
    builder.write_slice(&input);
    let receipt = default_prover()
        .prove(builder.build()?, VERICODE_GUEST_ELF)?
        .receipt;
    Ok(receipt)
}

fn expected_journal(
    scenario: Scenario,
    expected_image_id: [u8; IMAGE_ID_SIZE],
) -> Result<JournalV1, AnyError> {
    let artifact = canonical_artifact(scenario)?;
    map_debug(
        evaluate_restricted_artifact(
            JobId::new(JOB_ID_BYTES),
            ImageId::new(expected_image_id),
            &artifact,
        ),
        "evaluate expected restricted artifact",
    )
}

fn commitments_for(journal: &JournalV1) -> JournalV1Commitments {
    JournalV1Commitments::new(
        journal.job_id(),
        journal.spec_hash(),
        journal.harness_hash(),
        journal.artifact_hash(),
        journal.image_id(),
    )
}

fn validate_expected_journal(
    journal_bytes: &[u8],
    scenario: Scenario,
    expected_image_id: [u8; IMAGE_ID_SIZE],
) -> Result<(), AnyError> {
    let actual = decode_exact_journal(journal_bytes)?;
    let expected = expected_journal(scenario, expected_image_id)?;
    let expected_bytes = map_debug(
        expected.encode_candidate(),
        "encode expected JournalV1 candidate",
    )?;

    if journal_bytes != expected_bytes {
        return Err(invalid_data(format!(
            "{} journal differs byte-for-byte from the local candidate vector",
            scenario.name
        )));
    }
    let verdict = map_debug(
        actual.validate_against(&commitments_for(&expected)),
        "validate JournalV1 commitments",
    )?;
    if verdict != scenario.expected_verdict {
        return Err(invalid_data(format!(
            "{} journal contained verdict {verdict:?}",
            scenario.name
        )));
    }
    Ok(())
}

fn decode_exact_journal(bytes: &[u8]) -> Result<JournalV1, AnyError> {
    if bytes.len() != JOURNAL_V1_CANDIDATE_WIRE_SIZE {
        return Err(invalid_data(format!(
            "journal length is {}, expected {}",
            bytes.len(),
            JOURNAL_V1_CANDIDATE_WIRE_SIZE
        )));
    }
    map_debug(
        JournalV1::decode_candidate(bytes),
        "decode JournalV1 candidate",
    )
}

fn record_receipt(
    output_dir: &Path,
    name: &str,
    receipt: &Receipt,
) -> Result<(), AnyError> {
    let receipt_bytes = bincode::serialize(receipt)?;
    let path = output_dir.join(format!("{name}.receipt"));
    fs::write(&path, &receipt_bytes)?;

    println!("{name}.receipt_type={}", receipt_type(&receipt.inner));
    println!("{name}.receipt_bytes={}", receipt_bytes.len());
    println!("{name}.receipt_path={}", path.display());
    println!("{name}.journal_bytes={}", receipt.journal.bytes.len());
    println!(
        "{name}.journal_hex={}",
        encode_hex(&receipt.journal.bytes)
    );
    Ok(())
}

fn receipt_type(inner: &InnerReceipt) -> &'static str {
    match inner {
        InnerReceipt::Composite(_) => "Composite",
        InnerReceipt::Succinct(_) => "Succinct",
        InnerReceipt::Groth16(_) => "Groth16",
        InnerReceipt::Fake(_) => "Fake",
        _ => "Unknown",
    }
}

fn encode_hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(DIGITS[(byte >> 4) as usize] as char);
        output.push(DIGITS[(byte & 0x0f) as usize] as char);
    }
    output
}

fn map_debug<T, E: Debug>(
    result: Result<T, E>,
    action: &str,
) -> Result<T, AnyError> {
    result.map_err(|error| invalid_data(format!("failed to {action}: {error:?}")))
}

fn invalid_input(message: impl Into<String>) -> AnyError {
    IoError::new(ErrorKind::InvalidInput, message.into()).into()
}

fn invalid_data(message: impl Into<String>) -> AnyError {
    IoError::new(ErrorKind::InvalidData, message.into()).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn require_ok<T, E: Debug>(result: Result<T, E>) -> T {
        match result {
            Ok(value) => value,
            Err(error) => panic!("expected Ok, got {error:?}"),
        }
    }

    #[test]
    fn guest_input_is_the_fixed_public_frame() {
        let image_id = [0x55; IMAGE_ID_SIZE];
        let input = require_ok(guest_input(Scenario::pass(), image_id));

        assert_eq!(input.len(), GUEST_INPUT_SIZE);
        assert_eq!(&input[..JOB_ID_SIZE], &JOB_ID_BYTES);
        assert_eq!(
            &input
                [JOB_ID_SIZE..JOB_ID_SIZE + RESTRICTED_ARTIFACT_V1_WIRE_SIZE],
            &[1, 0, 0, 0, 7, 0, 0, 0, 14, 0, 0, 0]
        );
        assert_eq!(
            &input[JOB_ID_SIZE + RESTRICTED_ARTIFACT_V1_WIRE_SIZE..],
            &image_id
        );
    }

    #[test]
    fn expected_pass_and_fail_are_distinct_exact_journals() {
        let image_id = [0x55; IMAGE_ID_SIZE];
        let pass = require_ok(expected_journal(Scenario::pass(), image_id));
        let fail = require_ok(expected_journal(Scenario::fail(), image_id));
        let pass_bytes = require_ok(pass.encode_candidate());
        let fail_bytes = require_ok(fail.encode_candidate());

        assert_eq!(pass_bytes.len(), JOURNAL_V1_CANDIDATE_WIRE_SIZE);
        assert_eq!(fail_bytes.len(), JOURNAL_V1_CANDIDATE_WIRE_SIZE);
        assert_ne!(pass_bytes, fail_bytes);
        assert_eq!(pass.verdict(), Verdict::Pass);
        assert_eq!(fail.verdict(), Verdict::Fail);
    }
}
