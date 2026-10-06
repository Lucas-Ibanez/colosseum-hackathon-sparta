//! The Docker shim (`docker-shim/docker`, gate D10a, finding RD7-02 of the
//! R-D7) accepts exactly the argv lists of its callers and rebuilds the real
//! command from scratch; every other argv is refused before `docker` runs.
//!
//! A fake `docker` (`VERICODE_REAL_DOCKER`) only records its argv: the real
//! Docker is never called, and nothing here uses a network. Cases S1 to S7
//! are the R-D7 PoC (`rd7/poc/shim/shim_poc.sh`).

use std::{
    fs,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::Command,
};

use vericode_prover::{GROTH16_PROVER_IMAGE as DIGEST, GROTH16_PROVER_TAG as TAG};

struct Shim {
    dir: PathBuf,
    /// Canonical existing directory, as `vericode-prover` passes it.
    work: String,
}

struct Run {
    code: Option<i32>,
    /// Argv seen by the fake `docker`, one `[arg]…` line per call.
    record: String,
    stderr: String,
}

fn shim() -> Shim {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("docker-shim");
    let work = dir.join("groth16-work");
    fs::create_dir_all(&work).unwrap();
    let fake = dir.join("fake-docker");
    fs::write(
        &fake,
        "#!/bin/bash\nfor a in \"$@\"; do printf '[%s]' \"$a\"; done >> \"$FAKE_RECORD\"; echo >> \"$FAKE_RECORD\"\n",
    )
    .unwrap();
    fs::set_permissions(&fake, fs::Permissions::from_mode(0o700)).unwrap();
    let work = fs::canonicalize(&work).unwrap().to_str().unwrap().to_string();
    Shim { dir, work }
}

impl Shim {
    fn run(&self, name: &str, args: &[&str], docker_host: Option<&str>) -> Run {
        let record = self.dir.join(format!("{name}.record"));
        let _ = fs::remove_file(&record);
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("docker-shim/docker");
        let mut command = Command::new(path);
        command
            .args(args)
            .env_clear()
            .env("PATH", "/usr/bin:/bin")
            .env("VERICODE_REAL_DOCKER", self.dir.join("fake-docker"))
            .env("VERICODE_DOCKER_SHIM_LOG", self.dir.join("shim.log"))
            .env("FAKE_RECORD", &record);
        if let Some(host) = docker_host {
            command.env("DOCKER_HOST", host);
        }
        let output = command.output().unwrap();
        Run {
            code: output.status.code(),
            record: fs::read_to_string(&record).unwrap_or_default(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        }
    }
}

fn bracketed(argv: &[&str]) -> String {
    argv.iter().map(|arg| format!("[{arg}]")).collect::<String>() + "\n"
}

#[test]
fn the_caller_argv_lists_are_rebuilt_exactly() {
    let shim = shim();
    let volume = format!("{}:/mnt", shim.work);
    let run_rebuilt = [
        "--context", "default", "run", "--pull=never", "--network=none", "--rm", "-v", &volume, DIGEST,
    ];
    let cases: [(&str, Vec<&str>, Option<&str>, Vec<&str>); 4] = [
        ("version", vec!["--version"], None, vec!["--version"]),
        (
            "inspect",
            vec!["image", "inspect", "--format", "{{.Id}}", DIGEST],
            None,
            vec!["--context", "default", "image", "inspect", "--format", "{{.Id}}", DIGEST],
        ),
        ("S1-risc0-groth16-run", vec!["run", "--rm", "-v", &volume, TAG], None, run_rebuilt.to_vec()),
        (
            "S1-local-unix-socket",
            vec!["run", "--rm", "-v", &volume, TAG],
            Some("unix:///var/run/docker.sock"),
            run_rebuilt.to_vec(),
        ),
    ];
    let mut failures = Vec::new();
    for (name, args, host, executed) in cases {
        let run = shim.run(name, &args, host);
        if run.code != Some(0) || run.record != bracketed(&executed) {
            failures.push(format!("{name}: exit {:?}, docker saw {:?}, stderr {:?}", run.code, run.record, run.stderr));
        }
    }
    assert!(failures.is_empty(), "accepted argv lists not rebuilt exactly:\n{}", failures.join("\n"));
    let log = fs::read_to_string(shim.dir.join("shim.log")).unwrap_or_default();
    assert!(
        log.contains(&format!("run --pull=never --network=none --rm -v {volume} {DIGEST}")),
        "the executed run line is logged"
    );
}

#[test]
fn every_other_argv_is_refused_before_docker_runs() {
    let shim = shim();
    let work = shim.work.as_str();
    let volume = format!("{work}:/mnt");
    let dotted = format!("{work}/../groth16-work:/mnt");
    let missing = format!("{work}/missing:/mnt");
    let colon = format!("{work}:/x:/mnt");
    let other_target = format!("{work}:/host");
    let cases: Vec<(&str, Vec<&str>, Option<&str>)> = vec![
        ("S2-later-flags", vec!["run", "--network=host", "--pull=always", "--privileged", "-v", "/:/host", TAG], None),
        ("S3-other-image", vec!["run", "--rm", "alpine"], None),
        ("S4-container-run", vec!["container", "run", "--rm", "-v", &volume, TAG], None),
        ("S5-global-context", vec!["--context", "default", "run", "--rm", "-v", &volume, TAG], None),
        ("S6-pull", vec!["pull", TAG], None),
        ("S7-tag-as-argument", vec!["run", "--rm", "alpine", TAG], None),
        ("extra-flag", vec!["run", "--rm", "-v", &volume, "--privileged", TAG], None),
        ("digest-instead-of-tag", vec!["run", "--rm", "-v", &volume, DIGEST], None),
        ("other-image-in-tag-slot", vec!["run", "--rm", "-v", &volume, "alpine"], None),
        ("relative-work-dir", vec!["run", "--rm", "-v", "groth16-work:/mnt", TAG], None),
        ("root-work-dir", vec!["run", "--rm", "-v", "/:/mnt", TAG], None),
        ("missing-work-dir", vec!["run", "--rm", "-v", &missing, TAG], None),
        ("non-canonical-work-dir", vec!["run", "--rm", "-v", &dotted, TAG], None),
        ("colon-in-work-dir", vec!["run", "--rm", "-v", &colon, TAG], None),
        ("other-mount-target", vec!["run", "--rm", "-v", &other_target, TAG], None),
        ("remote-docker-host", vec!["run", "--rm", "-v", &volume, TAG], Some("tcp://10.0.0.1:2375")),
        ("inspect-other-image", vec!["image", "inspect", "--format", "{{.Id}}", "alpine"], None),
        ("inspect-by-tag", vec!["image", "inspect", "--format", "{{.Id}}", TAG], None),
        ("inspect-extra-flag", vec!["image", "inspect", "--format", "{{.Id}}", DIGEST, "-q"], None),
        ("version-subcommand", vec!["version"], None),
        ("version-extra", vec!["--version", "--format", "x"], None),
        ("empty-argv", vec![], None),
    ];
    let mut failures = Vec::new();
    for (name, args, host) in cases {
        let run = shim.run(name, &args, host);
        if run.code != Some(2) || !run.record.is_empty() || !run.stderr.contains("refusing") {
            failures.push(format!("{name}: exit {:?}, docker saw {:?}", run.code, run.record));
        }
    }
    assert!(failures.is_empty(), "argv lists that reached docker:\n{}", failures.join("\n"));
}
