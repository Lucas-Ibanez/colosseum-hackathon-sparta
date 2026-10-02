#![no_main]
#![no_std]

use risc0_zkvm::guest::env;
use vericode_core::{
    evaluate_restricted_artifact, ImageId, JobId,
    JOURNAL_V1_CANDIDATE_WIRE_SIZE, RESTRICTED_ARTIFACT_V1_WIRE_SIZE,
};

risc0_zkvm::guest::entry!(main);

const JOB_ID_SIZE: usize = 32;
const IMAGE_ID_SIZE: usize = 32;
const GUEST_INPUT_SIZE: usize =
    JOB_ID_SIZE + RESTRICTED_ARTIFACT_V1_WIRE_SIZE + IMAGE_ID_SIZE;

fn main() {
    let mut input = [0_u8; GUEST_INPUT_SIZE];
    env::read_slice(&mut input);

    let mut job_id = [0_u8; JOB_ID_SIZE];
    job_id.copy_from_slice(&input[..JOB_ID_SIZE]);

    let artifact_start = JOB_ID_SIZE;
    let artifact_end = artifact_start + RESTRICTED_ARTIFACT_V1_WIRE_SIZE;
    let artifact = &input[artifact_start..artifact_end];

    let mut image_id = [0_u8; IMAGE_ID_SIZE];
    image_id.copy_from_slice(&input[artifact_end..]);

    let journal = match evaluate_restricted_artifact(
        JobId::new(job_id),
        ImageId::new(image_id),
        artifact,
    ) {
        Ok(journal) => journal,
        Err(_) => env::exit(2),
    };
    let journal_bytes = match journal.encode_candidate() {
        Ok(bytes) => bytes,
        Err(_) => env::exit(3),
    };
    if journal_bytes.len() != JOURNAL_V1_CANDIDATE_WIRE_SIZE {
        env::exit(4);
    }

    env::commit_slice(&journal_bytes);
}
