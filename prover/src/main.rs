//! `vericode-prover`: see the library documentation and `prover/README.md`.

use std::{env, path::Path};

use vericode_prover::{check, compress, prove, verify, AnyError};

const USAGE: &str = "usage:
  vericode-prover check
  vericode-prover prove <job_id_hex> <input> <claimed_output> <dir>
  vericode-prover compress <dir>
  vericode-prover verify <dir>";

fn main() -> Result<(), AnyError> {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.iter().map(String::as_str).collect::<Vec<_>>().as_slice() {
        ["check"] => check(),
        ["prove", job_id, input, output, dir] => prove(job_id, input.parse()?, output.parse()?, Path::new(dir)),
        ["compress", dir] => compress(Path::new(dir)),
        ["verify", dir] => verify(Path::new(dir)),
        _ => Err(USAGE.into()),
    }
}
