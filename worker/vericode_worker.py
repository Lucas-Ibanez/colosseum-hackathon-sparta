#!/usr/bin/env python3
"""VeriCode local worker (gate D10): a minimal HTTP API on 127.0.0.1 over the
D10a binaries. See `worker/README.md`.

The worker decides nothing economic. Every action is one fixed argv list of
`vericode` (devnet client) or `vericode-prover` (local prover), run without a
shell, in an environment built from scratch, one action at a time. The state
it shows comes from `vericode job show` and from the CLI `--log` JSON Lines,
never from an exit code alone. The devnet keys of the buyer and the executor
stay in files that only the CLI opens, by path; no response, log or page of
the worker carries a key path or key bytes. Standard library only.
"""

import argparse
import dataclasses
import datetime
import hashlib
import hmac
import http.server
import json
import os
import re
import secrets
import signal
import stat
import subprocess
import sys
import threading
import time
from pathlib import Path

# Pinned binaries (C10-1) and shim (C10-2). The real entry point refuses any
# other value; the D9 binaries are refused by name.
PINNED_SHA256 = {
    "cli_sha256": "e6cd4e2914fcfc1233b876cb6f8a3b752e62e770cc4a309c7ce67e456d59acae",
    "prover_sha256": "3f66e1c05a6d587cb64d193be33ceda77c85cb6b0a660922f6b4644e24125232",
    "shim_sha256": "2a8f75b87766b98fe9759d233b929c215d043f1a97c1434f467cba442ba3d866",
}
D9_SHA256_PREFIXES = ("7e7a9260", "79b83528")
# The D10a prover runs `env!("CARGO_MANIFEST_DIR")/docker-shim/docker`.
PINNED_SHIM_PATH = "/home/lucas/src/vericode/prover/docker-shim/docker"
# Mint authority of Test USDC: never one of the worker's keys (C10-9).
DEPLOYER_PUBKEY = "617ogw9Tbem67ZwWDokEAkLV6JMq5avreijFMPrG75nd"

ESCROW_ID = "GZqbL2TbeDVHcNRosngaRfCwzV9YJT6iEbckYr8uwkCH"
VERIFIER_ID = "THq1qFYQoh7zgcjXoMXduDBqiZRCPeg3PvvMbrVQUge"
ADMITTED_IMAGE_ID = "4da06f90da75ec8980c943ce017d69c48370fddbf3aa27689d375d78fac0fb1a"
GROTH16_IMAGE = (
    "risczero/risc0-groth16-prover@sha256:7f173963196570b7a71816ed70565a4579264c5d2e3e0ecb028102538ad0e331"
)

# Preflight lines (C10-1): exact lines, and prefixes of lines that end with a
# slot or a supply.
CHECK_LINES = (
    "check.cluster=devnet genesis=EtWTRABZaYq6iMfeYKouRu166VU2xqa1wcaWoxPkrZBG",
    "check.escrow_program_data bytes=395064 sha256=cdf6967f3abc63d0385e36909fe61b36f639203e2679209be60c4875114d8133",
    f"check.verifier={VERIFIER_ID} program_data=ENdLkqHpzKcrQy4XzFUhkN7H3C3Mz1cugjJBpiNEDxpn "
    "upgrade_authority=none bytes=199256 sha256=34ae6e5c9d63dfe67c48fa04cad04e9752ad9f1cfbc8b4d66e99941df7666cd1",
    "check=ok",
)
CHECK_PREFIXES = (
    f"check.escrow={ESCROW_ID} upgrade_authority=none deployed_slot=",
    "check.mint=9TE2VPFmgrNxT22yS3sEZyRcMxLgJkwzgAoquWRXwV2F decimals=6 freeze_authority=none supply=",
)
PROVER_CHECK_LINES = (
    "guest.sha256=e09ba8cf16f7e36cb92e10656b7574c598f3e88fe089f02bc747a4bb00c078f5",
    f"guest.image_id={ADMITTED_IMAGE_ID}",
    "guest.admitted=true",
    "groth16.selector=73c457ba",
    "prover=LocalProver env=ok",
)
# Key probes: the CLI loads the key, prints its pubkey and then fails on a
# deliberately invalid argument, before any RPC (`cli/src/main.rs`, `job_create`
# and `job_deliver`). The worker itself never opens a key file.
PROBE_ERRORS = {
    "buyer": 'error: --executor: not a base58 pubkey: "1"',
    "executor": 'error: expected 64 hex digits, got "00"',
}

AMOUNT = 1_000_000
MIN_DEADLINE_OFFSET = 1_560
MAX_DEADLINE_OFFSET = 9_000
C10_8_MARGIN_SLOTS = 300
MIN_MEM_AVAILABLE_KB = 2_621_440  # 2.5 GiB (C10-4)
U32_MAX = 2**32 - 1
MAX_BODY = 4096
TIMEOUTS = {"probe": 30, "check": 180, "show": 180, "write": 600, "prove": 900, "compress": 1800}
# Negative runs of CR2 (C10-7): route kind -> (--expect-error program, code).
NEGATIVE_KINDS = {
    "escrow-6021": ("escrow", 6021),
    "verifier-6003": ("verifier", 6003),
    "escrow-6007": ("escrow", 6007),
    "escrow-6014": ("escrow", 6014),
}
PROGRAM_IDS = {"escrow": ESCROW_ID, "verifier": VERIFIER_ID}
WRITE_KINDS = ("create", "settle", "refund-timeout", "negative")
OUTCOMES = ("PASS", "UNEXPECTED", "SIMULATION_UNEXPECTED_NOT_SENT", "NOT_LANDED_BLOCKHASH_EXPIRED")
FIXTURE_JOB_ID = "11" * 32

B58 = "[1-9A-HJ-NP-Za-km-z]"
PK = B58 + "{32,44}"
HEX64_RE = re.compile(r"[0-9a-f]{64}")
OP_ID_RE = re.compile(r"[0-9a-f]{16}")
PUBKEY_RE = re.compile(PK)
SIG_RE = re.compile(B58 + "{64,88}")
LABEL_RE = re.compile(r"[a-z_+]{1,40}")
KEY_ARRAY_RE = re.compile(r"\[\s*(?:\d{1,3}\s*,\s*){63}\d{1,3}\s*\]")
SENT_RE = re.compile(r"\[(?P<label>[a-z_+]+)\] sent signature=(?P<sig>\S+) skip_preflight=(?:true|false)")
ALREADY_RE = re.compile(
    r'\[(?P<label>[a-z_+]+)\] sendTransaction answered "already processed"; resolving signature=(?P<sig>\S+) by its status'
)
CREATE_ID_RE = re.compile(r"create\.job_id=(?P<job_id>[0-9a-f]{64})")
NO_JOB_RE = re.compile(rf"error: no Job {PK} for job_id (?P<job_id>[0-9a-f]{{64}})")
DOCKER_RUN_RE = re.compile(
    r"(?P<ts>\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}[+-]\d{2}:\d{2}) /usr/bin/docker --context default run "
    r"--pull=never --network=none --rm -v (?P<dir>/[^\s:,]+)/groth16-work:/mnt " + re.escape(GROTH16_IMAGE)
)
VERIFIER_UNITS_RE = re.compile(rf"Program {VERIFIER_ID} consumed (\d+) of \d+ compute units")
SHOW_PATTERNS = {
    "job_id": re.compile(r"job\.job_id=([0-9a-f]{64})"),
    "address": re.compile(rf"job\.address=({PK})"),
    "buyer": re.compile(rf"job\.buyer=({PK})"),
    "executor": re.compile(rf"job\.executor=({PK})"),
    "mint": re.compile(rf"job\.mint=({PK}) admitted=(true|false)"),
    "amount": re.compile(r"job\.amount=(\d+)"),
    "deadline_slot": re.compile(r"job\.deadline_slot=(\d+)"),
    "spec_hash": re.compile(r"job\.spec_hash=([0-9a-f]{64})"),
    "harness_hash": re.compile(r"job\.harness_hash=([0-9a-f]{64})"),
    "image_id": re.compile(r"job\.image_id=([0-9a-f]{64})"),
    "terms_v1_admitted": re.compile(r"job\.terms_v1_admitted=(true|false)"),
    "status": re.compile(
        r"job\.status=(Created|Funded|Delivered|Released|RefundedOnFail|RefundedOnTimeout)"
        r"(?: artifact_hash=([0-9a-f]{64}))?"
    ),
    "vault": re.compile(
        rf"vault\.address=({PK}) mint=({PK}) authority=({PK}) authority_is_job_pda=(true|false) "
        r"delegate=(\S+) close_authority=(\S+) amount=(\d+)"
    ),
    "vault_missing": re.compile(rf"vault\.address=({PK}) missing"),
    "buyer_ata": re.compile(rf"buyer\.ata=({PK}) test_usdc=(\d+|missing) sol_lamports=(\d+)"),
    "executor_ata": re.compile(rf"executor\.ata=({PK}) test_usdc=(\d+|missing) sol_lamports=(\d+)"),
    "slot": re.compile(r"slot=(\d+) deadline_slot=(\d+) past_deadline=(true|false)"),
}
SHOW_REQUIRED = ("job_id", "address", "buyer", "executor", "mint", "amount", "deadline_slot", "status", "slot")
SECURITY_HEADERS = (
    ("Cache-Control", "no-store"),
    ("X-Content-Type-Options", "nosniff"),
    ("Content-Security-Policy", "default-src 'none'; frame-ancestors 'none'"),
    ("Referrer-Policy", "no-referrer"),
    ("X-Frame-Options", "DENY"),
)

# The Hive interface (D11-D12): a fixed table of files under worker/static/, served
# at /ui/ by exact match, GET only, without the token (the page asks for it) and
# with a CSP that allows only this origin. Nothing is listed or globbed.
STATIC_DIR = Path(__file__).resolve().parent / "static"
STATIC_INDEX = "/ui/"
STATIC_FILES = (
    "css/tokens.css",
    "css/app.css",
    "js/api.js",
    "js/components.js",
    "js/dom.js",
    "js/format.js",
    "js/i18n.js",
    "js/icons.js",
    "js/main.js",
    "js/views/job.js",
    "js/views/jobs.js",
    "js/views/new-job.js",
    "fonts/fonts.css",
    "fonts/manrope-wght-latin-ext.woff2",
    "fonts/manrope-wght-latin.woff2",
    "fonts/ibm-plex-mono-400-latin-ext.woff2",
    "fonts/ibm-plex-mono-400-latin.woff2",
    "fonts/ibm-plex-mono-500-latin-ext.woff2",
    "fonts/ibm-plex-mono-500-latin.woff2",
    "fonts/OFL-Manrope.txt",
    "fonts/OFL-IBMPlexMono.txt",
    "brand/hive-horizontal-branco.svg",
    *(f"icons/{name}.svg" for name in (
        "blocks", "check", "chevron-down", "chevron-up", "circle-check", "circle-dashed", "circle-x", "clock", "copy",
        "cpu", "equal", "equal-not", "external-link", "file-check", "file-x", "hourglass", "info", "key-round", "list",
        "loader-circle", "lock", "minus", "package-check", "plus", "refresh-cw", "repeat", "server", "terminal",
        "triangle-alert", "undo-2", "unlink", "x")),
    "icons/LICENSE-lucide.txt",
)
STATIC_ROUTES = {STATIC_INDEX: "index.html", **{STATIC_INDEX + name: name for name in STATIC_FILES}}
CONTENT_TYPES = {
    ".html": "text/html; charset=utf-8",
    ".css": "text/css; charset=utf-8",
    ".js": "text/javascript; charset=utf-8",
    ".svg": "image/svg+xml",
    ".woff2": "font/woff2",
    ".txt": "text/plain; charset=utf-8",
}
PAGE_CSP = ("default-src 'none'; script-src 'self'; style-src 'self'; font-src 'self'; img-src 'self'; "
            "connect-src 'self'; base-uri 'none'; form-action 'none'; frame-ancestors 'none'")
PAGE_HEADERS = (
    ("Cache-Control", "no-store"),
    ("X-Content-Type-Options", "nosniff"),
    ("Content-Security-Policy", PAGE_CSP),
    ("Referrer-Policy", "no-referrer"),
    ("X-Frame-Options", "DENY"),
)

# Read-only views for the interface (D11-D12); they derive from what the worker
# already stores and decide nothing.
VERIFIER_RELEASE = "risc0-solana v3.0.0"  # the pinned verifier bytes 34ae6e5c… (CHECK_LINES; README)
TOKEN_PROGRAM_ID = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
PROGRAM_KEYS = {
    ESCROW_ID: "escrow",
    VERIFIER_ID: "verifier",
    TOKEN_PROGRAM_ID: "token",
    "11111111111111111111111111111111": "system",
    "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL": "ata",
    "ComputeBudget111111111111111111111111111111": "compute_budget",
}
JOURNAL_V1_BYTES = 165
JOURNAL_V1_FIELDS = (("job_id", 4), ("spec_hash", 36), ("harness_hash", 68), ("artifact_hash", 100), ("image_id", 132))
LOCAL_PROVE_KEYS = ("prove.prover", "prove.artifact", "prove.seconds", "prove.receipt_type", "prove.local_verify",
                    "prove.journal_equal_to_core", "prove.verdict")
LOCAL_COMPRESS_KEYS = ("compress.prover", "compress.docker_image", "compress.seconds", "compress.receipt_type",
                       "compress.local_verify", "compress.journal_equal_to_composite", "compress.selector",
                       "compress.image_id", "compress.verdict", "compress.journal_digest")
INVOKE_RE = re.compile(rf"Program ({PK}) invoke \[(\d{{1,2}})\]")
CONSUMED_RE = re.compile(rf"Program ({PK}) consumed (\d+) of \d+ compute units")
SUCCESS_RE = re.compile(rf"Program ({PK}) success")
FAILED_RE = re.compile(rf"Program ({PK}) failed: (.{{1,200}})")
INSTRUCTION_RE = re.compile(r"Program log: Instruction: ([A-Za-z0-9_]{1,64})")
ANCHOR_ERROR_RE = re.compile(r"Program log: AnchorError .{0,200}?Error Code: ([A-Za-z0-9_]{1,64})\. Error Number: (\d{1,10})\.")
MAX_INVOCATIONS = 64
CREATE_LIVE_RES = {
    "job": re.compile(rf"create\.job=({PK})"),
    "vault": re.compile(rf"create\.vault=({PK})"),
    "slot": re.compile(r"create\.slot=(\d+) deadline_slot=(\d+) offset=(\d+)"),
}
FACT_RES = {
    "cluster": re.compile(r"check\.cluster=(devnet) genesis=(" + PK + ")"),
    "escrow": re.compile(rf"check\.escrow=({PK}) upgrade_authority=(none) deployed_slot=(\d+)"),
    "escrow_data": re.compile(r"check\.escrow_program_data bytes=(\d+) sha256=([0-9a-f]{64})"),
    "verifier": re.compile(rf"check\.verifier=({PK}) program_data=({PK}) upgrade_authority=(none) bytes=(\d+) "
                           r"sha256=([0-9a-f]{64})"),
    "mint": re.compile(rf"check\.mint=({PK}) decimals=(\d+) freeze_authority=(none) supply=(\d+)"),
    "terms": re.compile(r"check\.terms_v1 spec_hash=([0-9a-f]{64}) harness_hash=([0-9a-f]{64}) image_id=([0-9a-f]{64})"),
}
MIN_SLOT_CLOCK_SECONDS = 600


class WorkerError(Exception):
    pass


class StartupError(WorkerError):
    pass


class OpFailed(WorkerError):
    pass


class HttpError(Exception):
    def __init__(self, status, code, detail="", extra=None):
        super().__init__(code)
        self.status, self.code, self.detail, self.extra = status, code, detail, extra or {}


def now_iso():
    return datetime.datetime.now().astimezone().isoformat(timespec="seconds")


def sha256_file(path):
    digest = hashlib.sha256()
    with open(path, "rb") as file:
        for chunk in iter(lambda: file.read(1 << 20), b""):
            digest.update(chunk)
    return digest.hexdigest()


def inside_git_work_tree(path):
    real = Path(os.path.realpath(path))
    return any((directory / ".git").exists() for directory in (real, *real.parents))


def explorer_tx(signature):
    return f"https://explorer.solana.com/tx/{signature}?cluster=devnet"


def explorer_address(address):
    return f"https://explorer.solana.com/address/{address}?cluster=devnet"


def read_mem_available_kb():
    with open("/proc/meminfo") as file:
        for line in file:
            if line.startswith("MemAvailable:"):
                return int(line.split()[1])
    raise WorkerError("MemAvailable is missing from /proc/meminfo")


def write_json(path, value):
    tmp = path.with_name(path.name + ".tmp")
    fd = os.open(tmp, os.O_WRONLY | os.O_CREAT | os.O_TRUNC, 0o600)
    with os.fdopen(fd, "w") as file:
        json.dump(value, file, indent=1, sort_keys=True)
        file.write("\n")
    os.replace(tmp, path)


def append_jsonl(path, value):
    fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_APPEND, 0o600)
    with os.fdopen(fd, "a") as file:
        file.write(json.dumps(value, sort_keys=True) + "\n")


def copy(value):
    return json.loads(json.dumps(value))


def line_value(lines, key):
    prefix = key + "="
    return next((line[len(prefix):] for line in lines if line.startswith(prefix)), None)


def custom_code(err):
    try:
        code = err["InstructionError"][1]["Custom"]
    except (TypeError, KeyError, IndexError):
        return None
    return code if type(code) is int else None


def int_or_none(value):
    return value if type(value) is int else None


def bool_or_none(value):
    return value if type(value) is bool else None


# --- configuration -----------------------------------------------------------


@dataclasses.dataclass(frozen=True)
class Config:
    cli: str
    cli_sha256: str
    prover: str
    prover_sha256: str
    shim: str
    shim_sha256: str
    buyer_keypair: str
    buyer_pubkey: str
    executor_keypair: str
    executor_pubkey: str
    data_dir: str
    home_dir: str
    tmp_dir: str
    port: int


FIELDS = tuple(field.name for field in dataclasses.fields(Config))


def check_config(config):
    """Checks shared by the real entry point and the tests. Messages name
    fields, never key paths."""
    for name in FIELDS:
        value = getattr(config, name)
        if name == "port":
            if type(value) is not int or not 0 <= value <= 65535:
                raise StartupError("config: port must be an integer port number")
        elif not isinstance(value, str) or not value:
            raise StartupError(f"config: {name} must be a non-empty string")
    for name in ("cli_sha256", "prover_sha256", "shim_sha256"):
        if not HEX64_RE.fullmatch(getattr(config, name)):
            raise StartupError(f"config: {name} is not a lowercase SHA-256 digest")
    for name in ("buyer_pubkey", "executor_pubkey"):
        if not PUBKEY_RE.fullmatch(getattr(config, name)):
            raise StartupError(f"config: {name} is not a base58 pubkey")
    if config.buyer_pubkey == config.executor_pubkey:
        raise StartupError("config: buyer and executor must be different keys")
    if DEPLOYER_PUBKEY in (config.buyer_pubkey, config.executor_pubkey):
        raise StartupError("config: the deployer key (mint authority) never enters the worker (C10-9)")
    for name in ("cli", "prover", "shim", "buyer_keypair", "executor_keypair", "data_dir", "home_dir", "tmp_dir"):
        if not os.path.isabs(getattr(config, name)):
            raise StartupError(f"config: {name} must be an absolute path")
    for name in ("buyer_keypair", "executor_keypair", "data_dir", "home_dir", "tmp_dir"):
        if inside_git_work_tree(getattr(config, name)):
            raise StartupError(f"config: {name} lies inside a Git work tree")
    for name in ("data_dir", "home_dir", "tmp_dir"):
        try:
            info = os.lstat(getattr(config, name))
        except OSError as error:
            raise StartupError(f"config: {name}: {error.strerror}") from None
        if not stat.S_ISDIR(info.st_mode) or info.st_uid != os.getuid() or stat.S_IMODE(info.st_mode) != 0o700:
            raise StartupError(f"config: {name} must be a directory of this user with mode 0700")


def load_config(path):
    """The real entry point's configuration: outside the clone, mode 0600,
    exact fields, and the pinned D10a binaries and shim."""
    try:
        info = os.lstat(path)
    except OSError as error:
        raise StartupError(f"config: {error.strerror}") from None
    if not stat.S_ISREG(info.st_mode):
        raise StartupError("config: not a regular file")
    if info.st_uid != os.getuid() or info.st_mode & 0o077:
        raise StartupError("config: must belong to this user with mode 0600")
    if inside_git_work_tree(path):
        raise StartupError("config: must live outside any Git work tree")
    try:
        raw = json.loads(Path(path).read_text())
    except (OSError, ValueError):
        raise StartupError("config: not readable JSON") from None
    if not isinstance(raw, dict):
        raise StartupError("config: not a JSON object")
    missing, unknown = sorted(set(FIELDS) - set(raw)), sorted(set(raw) - set(FIELDS))
    if missing or unknown:
        raise StartupError(f"config: missing fields {missing}, unknown fields {unknown}")
    config = Config(**raw)
    check_config(config)
    for name, pinned in PINNED_SHA256.items():
        value = getattr(config, name)
        if value.startswith(D9_SHA256_PREFIXES):
            raise StartupError(f"config: {name} names a D9 binary; only the D10a binaries are used (C10-1)")
        if value != pinned:
            raise StartupError(f"config: {name} is not the pinned D10a value (C10-1/C10-2)")
    if config.shim != PINNED_SHIM_PATH:
        raise StartupError("config: shim must be the repository shim that the D10a prover runs (C10-2)")
    for name in ("data_dir", "home_dir", "tmp_dir"):
        real = os.path.realpath(getattr(config, name))
        if real == "/tmp" or real.startswith("/tmp/"):
            raise StartupError(f"config: {name} must not be under /tmp")
    if config.port < 1024:
        raise StartupError("config: port must be between 1024 and 65535")
    return config


# --- subprocesses --------------------------------------------------------------


@dataclasses.dataclass
class Result:
    exit_code: int
    stdout: list
    stderr: str
    seconds: float
    timed_out: bool
    hook_errors: list


MAX_STDOUT_LINES = 20_000
MAX_STDERR_BYTES = 1 << 20


def run_process(argv, env, cwd, timeout, on_line=None):
    """Runs one fixed argv list (no shell) and returns its output. `on_line`
    sees each stdout line as it is printed. On timeout the whole process
    group is killed."""
    started = time.monotonic()
    proc = subprocess.Popen(
        argv,
        env=env,
        cwd=cwd,
        stdin=subprocess.DEVNULL,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        close_fds=True,
        start_new_session=True,
    )
    lines, hook_errors, err_parts = [], [], []

    def pump_stdout():
        for raw in proc.stdout:
            line = raw.decode("utf-8", "replace").rstrip("\r\n")
            if len(lines) < MAX_STDOUT_LINES:
                lines.append(line)
            if on_line is not None:
                try:
                    on_line(line)
                except Exception as error:  # recorded; the process is never left unread
                    hook_errors.append(f"{type(error).__name__}: {error}")

    def pump_stderr():
        size = 0
        for chunk in iter(lambda: proc.stderr.read(65536), b""):
            if size < MAX_STDERR_BYTES:
                err_parts.append(chunk[: MAX_STDERR_BYTES - size])
            size += len(chunk)

    pumps = [threading.Thread(target=pump_stdout, daemon=True), threading.Thread(target=pump_stderr, daemon=True)]
    for pump in pumps:
        pump.start()
    timed_out = False
    try:
        code = proc.wait(timeout=timeout)
    except subprocess.TimeoutExpired:
        timed_out = True
        try:
            os.killpg(proc.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        code = proc.wait()
    for pump in pumps:
        pump.join(timeout=10)
    proc.stdout.close()
    proc.stderr.close()
    stderr = b"".join(err_parts).decode("utf-8", "replace")
    return Result(code, lines, stderr, time.monotonic() - started, timed_out, hook_errors)


# --- CLI output ----------------------------------------------------------------


def parse_show(lines, job_id):
    """The chain view of `vericode job show`, every field checked by pattern."""
    found = {}
    for line in lines:
        for name, pattern in SHOW_PATTERNS.items():
            match = pattern.fullmatch(line)
            if match and name not in found:
                found[name] = match.groups()
    missing = [name for name in SHOW_REQUIRED if name not in found]
    if missing:
        raise ValueError(f"job show output lacks {missing}")
    if found["job_id"][0] != job_id:
        raise ValueError("job show printed another job_id")

    def ata(name):
        if name not in found:
            return None
        address, amount, lamports = found[name]
        return {"address": address, "test_usdc": None if amount == "missing" else int(amount), "sol_lamports": int(lamports)}

    if "vault" in found:
        address, _, _, is_pda, _, _, amount = found["vault"]
        vault = {"address": address, "amount": int(amount), "authority_is_job_pda": is_pda == "true"}
    elif "vault_missing" in found:
        vault = {"address": found["vault_missing"][0], "missing": True}
    else:
        vault = None
    status, artifact = found["status"]
    slot, _, past = found["slot"]
    address = found["address"][0]
    return {
        "status": status,
        "artifact_hash": artifact,
        "address": address,
        "explorer": explorer_address(address),
        "buyer": found["buyer"][0],
        "executor": found["executor"][0],
        "mint": found["mint"][0],
        "mint_admitted": found["mint"][1] == "true",
        "amount": int(found["amount"][0]),
        "deadline_slot": int(found["deadline_slot"][0]),
        "spec_hash": found.get("spec_hash", (None,))[0],
        "harness_hash": found.get("harness_hash", (None,))[0],
        "image_id": found.get("image_id", (None,))[0],
        "terms_v1_admitted": found.get("terms_v1_admitted", ("false",))[0] == "true",
        "vault": vault,
        "buyer_ata": ata("buyer_ata"),
        "executor_ata": ata("executor_ata"),
        "slot": int(slot),
        "past_deadline": past == "true",
    }


def read_tx_entries(path):
    """Transaction records of a CLI `--log` file."""
    try:
        text = path.read_text()
    except FileNotFoundError:
        return []
    entries = []
    for line in text.splitlines():
        try:
            entry = json.loads(line)
        except ValueError:
            continue
        if isinstance(entry, dict) and entry.get("kind") == "tx":
            entries.append(entry)
    return entries


def public_tx(entry, sent_signature):
    """Public fields of the landed transaction (from the CLI log), or only the
    signature printed at send time when the log has no record."""
    if entry is None:
        if sent_signature is None:
            return None
        return {"signature": sent_signature, "explorer": explorer_tx(sent_signature), "outcome": None}
    signature = entry.get("signature")
    signature = signature if isinstance(signature, str) and SIG_RE.fullmatch(signature) else None
    logs = [line for line in entry.get("logs") or [] if isinstance(line, str)]
    units = [int(match.group(1)) for line in logs if (match := VERIFIER_UNITS_RE.fullmatch(line))]
    snapshot = entry.get("snapshot") if isinstance(entry.get("snapshot"), dict) else {}
    simulation = entry.get("simulation") if isinstance(entry.get("simulation"), dict) else {}
    label = entry.get("label")
    outcome = entry.get("outcome")
    return {
        "label": label if isinstance(label, str) and LABEL_RE.fullmatch(label) else None,
        "signature": signature,
        "explorer": explorer_tx(signature) if signature else None,
        "outcome": outcome if outcome in OUTCOMES else None,
        "slot": int_or_none(entry.get("slot")),
        "err": entry.get("err"),
        "simulation_err": simulation.get("err"),
        "units": int_or_none(entry.get("units")),
        "fee": int_or_none(entry.get("fee")),
        "size": int_or_none(entry.get("size")),
        "verifier_invoked": bool_or_none(entry.get("verifier_invoked")),
        "verifier_units": units[0] if units else None,
        "watched_unchanged": bool_or_none(snapshot.get("unchanged")),
    }


def decode_journal(text):
    """The frozen JournalV1 v1 (docs/manifest-schema.md): exactly 165 bytes,
    schema_version 1, verdict PASS=0 / FAIL=1. Display only; the program checks."""
    if not isinstance(text, str) or not re.fullmatch(r"[0-9a-f]{%d}" % (2 * JOURNAL_V1_BYTES), text):
        raise ValueError(f"journal is not {JOURNAL_V1_BYTES} bytes of lowercase hex")
    raw = bytes.fromhex(text)
    version = int.from_bytes(raw[0:4], "little")
    if version != 1:
        raise ValueError(f"journal schema_version {version} is not 1")
    if raw[164] not in (0, 1):
        raise ValueError(f"journal verdict tag {raw[164]} is neither PASS (0) nor FAIL (1)")
    fields = {name: raw[offset:offset + 32].hex() for name, offset in JOURNAL_V1_FIELDS}
    return {"schema_version": version, **fields, "verdict": "PASS" if raw[164] == 0 else "FAIL", "hex": text}


def invocations_of(logs):
    """Program invocations of a landed transaction, in order, from its public
    logs: program, depth, instruction (when the program logs it), compute
    units, result and Anchor error."""
    calls, stack = [], []
    for line in logs:
        if match := INVOKE_RE.fullmatch(line):
            if len(calls) >= MAX_INVOCATIONS:
                break
            call = {"program_id": match.group(1), "program": PROGRAM_KEYS.get(match.group(1)),
                    "depth": int(match.group(2)), "instruction": None, "units": None, "result": None,
                    "error": None, "anchor_error": None}
            calls.append(call)
            stack.append(call)
            continue
        if not stack:
            continue
        top = stack[-1]
        if match := INSTRUCTION_RE.fullmatch(line):
            if top["instruction"] is None:
                top["instruction"] = match.group(1)
        elif match := ANCHOR_ERROR_RE.match(line):
            top["anchor_error"] = {"name": match.group(1), "code": int(match.group(2))}
        elif (match := CONSUMED_RE.fullmatch(line)) and match.group(1) == top["program_id"]:
            top["units"] = int(match.group(2))
        elif (match := SUCCESS_RE.fullmatch(line)) and match.group(1) == top["program_id"]:
            top["result"] = "success"
            stack.pop()
        elif (match := FAILED_RE.fullmatch(line)) and match.group(1) == top["program_id"]:
            top["result"], top["error"] = "failed", match.group(2)
            stack.pop()
    return calls


def token_amount(balance):
    try:
        amount = balance["uiTokenAmount"]["amount"]
    except (TypeError, KeyError):
        return None
    return int(amount) if isinstance(amount, str) and amount.isdigit() else None


def balances_of(entry, chain):
    """Token balances before and after a landed transaction (its public
    `pre/postTokenBalances`), each with the Job role of its owner."""
    def by_index(name):
        rows = {}
        for item in entry.get(name) or []:
            if isinstance(item, dict) and type(item.get("accountIndex")) is int:
                rows[item["accountIndex"]] = item
        return rows

    pre, post = by_index("pre_token_balances"), by_index("post_token_balances")
    chain = chain if isinstance(chain, dict) and not chain.get("absent") else {}
    roles = {
        chain.get("address"): ("vault", (chain.get("vault") or {}).get("address")),
        chain.get("buyer"): ("buyer", (chain.get("buyer_ata") or {}).get("address")),
        chain.get("executor"): ("executor", (chain.get("executor_ata") or {}).get("address")),
    }
    rows = []
    for index in sorted(set(pre) | set(post)):
        source = post.get(index) or pre.get(index)
        owner, mint = source.get("owner"), source.get("mint")
        owner = owner if isinstance(owner, str) and PUBKEY_RE.fullmatch(owner) else None
        mint = mint if isinstance(mint, str) and PUBKEY_RE.fullmatch(mint) else None
        role, account = roles.get(owner, (None, None)) if owner else (None, None)
        decimals = (source.get("uiTokenAmount") or {}).get("decimals")
        rows.append({"account_index": index, "owner": owner, "role": role, "token_account": account, "mint": mint,
                     "decimals": decimals if type(decimals) is int else None,
                     "before": token_amount(pre.get(index)), "after": token_amount(post.get(index))})
    return rows


def parse_iso(text):
    try:
        return datetime.datetime.fromisoformat(text).timestamp()
    except (TypeError, ValueError):
        return None


def read_app_commit(root):
    """The commit at the worker's start, read from .git (files only, no process)."""
    git = Path(root) / ".git"
    try:
        head = (git / "HEAD").read_text().strip()
        if head.startswith("ref: "):
            ref = head[5:]
            if not re.fullmatch(r"refs/heads/[A-Za-z0-9._/-]{1,100}", ref) or ".." in ref:
                return None
            if (git / ref).is_file():
                head = (git / ref).read_text().strip()
            else:
                packed = (git / "packed-refs").read_text().splitlines()
                head = next((line.split()[0] for line in packed if line.endswith(" " + ref)), "")
    except OSError:
        return None
    return head if re.fullmatch(r"[0-9a-f]{40}", head) else None


def rejection_of(expect, stdout, entry):
    """Which program rejected a negative run (C10-7), only when the CLI output
    shows all of it: the expected error, the simulation's innermost failure of
    that program, the PASS line, the PASS record and `Custom(code)` in the
    landed transaction. Anything else, `UNEXPECTED` included, is no rejection."""
    program, code = expect
    if entry is None or entry.get("outcome") != "PASS":
        return None
    label, signature = entry.get("label"), entry.get("signature")
    if not (isinstance(label, str) and LABEL_RE.fullmatch(label)):
        return None
    if not (isinstance(signature, str) and SIG_RE.fullmatch(signature)):
        return None
    innermost = f"Program {PROGRAM_IDS[program]} failed: custom program error: {code:#x}"
    if f"[{label}] simulation shows `{innermost}` as the innermost failure" not in stdout:
        return None
    if not any(line.startswith(f"[{label}] PASS ") for line in stdout):
        return None
    if custom_code(entry.get("err")) != code:
        return None
    return {
        "program": program,
        "program_id": PROGRAM_IDS[program],
        "code": code,
        "expect_error": f"{program}:{code}",
        "innermost": innermost,
        "signature": signature,
        "explorer": explorer_tx(signature),
    }


# --- worker ----------------------------------------------------------------------


class Worker:
    def __init__(self, config, *, mem_available_kb=read_mem_available_kb, http_log=sys.stderr):
        check_config(config)
        self.config = config
        self.data = Path(config.data_dir)
        self.mem_available_kb = mem_available_kb
        self.http_log = http_log
        self.busy = threading.Lock()
        self.state = threading.RLock()
        self.jobs = {}
        self.ops = {}
        self.current = None
        self.pending = set()
        self.startup_report = None
        self.token = None
        self.port = None
        self.incidents = []
        self.app_commit = None
        replacements = {}
        for role, path in (("buyer", config.buyer_keypair), ("executor", config.executor_keypair)):
            for variant in {path, os.path.realpath(path)}:
                replacements[variant] = f"<{role}-keypair>"
                replacements[os.path.dirname(variant)] = "<keys-dir>"
                replacements[os.path.basename(variant)] = f"<{role}-keypair-file>"
        # Longest first, so a full path is replaced before its directory.
        self.replacements = sorted(replacements.items(), key=lambda item: -len(item[0]))

    # --- hygiene ---

    def scrub_paths(self, text):
        for needle, placeholder in self.replacements:
            text = text.replace(needle, placeholder)
        return text

    def scrub(self, text, where):
        text = self.scrub_paths(text)
        if KEY_ARRAY_RE.search(text):
            text = KEY_ARRAY_RE.sub("<redacted: 64-number array>", text)
            incident = {"at": now_iso(), "where": where, "what": "64-number array in subprocess output (redacted)"}
            with self.state:
                self.incidents.append(incident)
            self.log_event({"event": "incident", **incident})
        return text

    def public_argv(self, argv):
        return [self.scrub_paths(arg) for arg in argv]

    def log_event(self, event):
        event = {"t": now_iso(), **event}
        append_jsonl(self.data / "worker-ops.jsonl", json.loads(self.scrub_paths(json.dumps(event))))

    def child_env(self, prover):
        """The whole environment of a subprocess (C10-3): nothing inherited."""
        env = {"HOME": self.config.home_dir, "PATH": "/usr/bin:/bin", "TMPDIR": self.config.tmp_dir}
        if prover:
            env["RISC0_PROVER"] = "local"
        return env

    def verify_binary(self, path):
        expected = {self.config.cli: self.config.cli_sha256, self.config.prover: self.config.prover_sha256}[path]
        info = os.stat(path)
        if not stat.S_ISREG(info.st_mode) or not os.access(path, os.X_OK):
            raise WorkerError(f"{os.path.basename(path)} is not an executable file")
        digest = sha256_file(path)
        if digest != expected:
            raise WorkerError(f"{os.path.basename(path)} SHA-256 {digest} is not the configured {expected} (C10-1)")
        return digest

    def verify_shim(self):
        """C10-2: the shim the prover runs is the pinned file with mode 0755."""
        path = self.config.shim
        info = os.lstat(path)
        if not stat.S_ISREG(info.st_mode):
            raise WorkerError("docker shim is not a regular file (C10-2)")
        mode = stat.S_IMODE(info.st_mode)
        if mode != 0o755:
            raise WorkerError(f"docker shim mode is {mode:04o}, not 0755 (C10-2)")
        digest = sha256_file(path)
        if digest != self.config.shim_sha256:
            raise WorkerError(f"docker shim SHA-256 {digest} is not {self.config.shim_sha256} (C10-2)")
        return {"path": path, "sha256": digest, "mode": "0755"}

    def shim_status(self):
        try:
            return {"ok": True, **self.verify_shim()}
        except (OSError, WorkerError) as error:
            return {"ok": False, "error": str(error)}

    # --- startup ---

    def startup(self):
        """Refuses to start unless binaries, shim, keys and both preflight
        checks are the expected ones (C10-1, C10-2, C10-9)."""
        config = self.config
        report = {"started_at": now_iso(), "binaries": {}, "probes": {}, "checks": {}}
        try:
            for name in ("cli", "prover"):
                report["binaries"][name] = {"path": getattr(config, name), "sha256": self.verify_binary(getattr(config, name))}
            report["binaries"]["shim"] = self.verify_shim()
            source_dir = os.path.dirname(os.path.dirname(config.shim)).encode()
            if source_dir not in Path(config.prover).read_bytes():
                raise WorkerError("the prover binary does not run the configured shim (C10-2)")
        except (OSError, WorkerError) as error:
            raise StartupError(str(error)) from None
        for role, argv in (
            ("buyer", [config.cli, "job", "create", "--buyer-keypair", config.buyer_keypair, "--executor", "1"]),
            ("executor", [config.cli, "job", "deliver", "--executor-keypair", config.executor_keypair, "--job-id", "00"]),
        ):
            result = run_process(argv, self.child_env(False), config.home_dir, TIMEOUTS["probe"])
            stdout = [self.scrub(line, f"probe {role}") for line in result.stdout]
            stderr = self.scrub(result.stderr, f"probe {role}").strip()
            expected = getattr(config, f"{role}_pubkey")
            printed = [line for line in stdout if line]
            if printed == [f"{role}={DEPLOYER_PUBKEY}"]:
                raise StartupError(f"{role} keypair is the deployer key; it never enters the worker (C10-9)")
            if result.exit_code != 1 or printed != [f"{role}={expected}"] or stderr != PROBE_ERRORS[role]:
                raise StartupError(f"{role} keypair probe failed: stdout {printed}, stderr {stderr!r}")
            report["probes"][role] = expected
        for name, argv, prover, lines, prefixes in (
            ("cli", [config.cli, "check"], False, CHECK_LINES, CHECK_PREFIXES),
            ("prover", [config.prover, "check"], True, PROVER_CHECK_LINES, ()),
        ):
            result = run_process(argv, self.child_env(prover), config.home_dir, TIMEOUTS["check"])
            missing = [line for line in lines if line not in result.stdout]
            missing += [prefix for prefix in prefixes if not any(line.startswith(prefix) for line in result.stdout)]
            if result.exit_code != 0 or result.timed_out or missing:
                raise StartupError(f"{name} check failed (exit {result.exit_code}): missing {missing}")
            report["checks"][name] = [self.scrub(line, f"{name} check") for line in result.stdout]
        self.load_state()
        self.app_commit = read_app_commit(Path(__file__).resolve().parent.parent)
        self.token = secrets.token_urlsafe(32)
        self.startup_report = report
        self.log_event({"event": "startup", "binaries": report["binaries"], "probes": report["probes"], "checks": "ok"})
        return report

    def load_state(self):
        for name in ("jobs", "ops", "receipts"):
            os.makedirs(self.data / name, mode=0o700, exist_ok=True)
        for path in sorted((self.data / "jobs").glob("*.json")):
            record = json.loads(path.read_text())
            if HEX64_RE.fullmatch(path.stem) and record.get("job_id") == path.stem:
                self.jobs[path.stem] = record
                if record.get("reconcile_pending"):
                    self.pending.add(path.stem)
        for path in sorted((self.data / "ops").glob("*/op.json")):
            op = json.loads(path.read_text())
            if not (OP_ID_RE.fullmatch(path.parent.name) and op.get("op_id") == path.parent.name):
                continue
            self.ops[op["op_id"]] = op
            if op.get("status") == "running":
                op.update(status="interrupted", ended_at=now_iso(), error="the worker stopped during this operation")
                job_id = op.get("job_id")
                record = self.jobs.get(job_id)
                if record is not None and op["kind"] in WRITE_KINDS:
                    self.set_pending(job_id, True)
                if record is not None and op["kind"] == "prove":
                    for attempt in record["receipts"]:
                        if attempt.get("op_id") == op["op_id"] and attempt["status"] == "running":
                            attempt.update(status="failed", failure="interrupted")
                    self.persist_job(job_id)
                self.persist_op(op)
                self.log_event({"event": "end", "op_id": op["op_id"], "status": "interrupted"})

    # --- persistence ---

    def persist_job(self, job_id):
        with self.state:
            write_json(self.data / "jobs" / f"{job_id}.json", self.jobs[job_id])

    def persist_op(self, op):
        with self.state:
            write_json(self.data / "ops" / op["op_id"] / "op.json", json.loads(self.scrub_paths(json.dumps(op))))

    def set_pending(self, job_id, pending):
        with self.state:
            record = self.jobs.get(job_id)
            if pending:
                self.pending.add(job_id)
            else:
                self.pending.discard(job_id)
            if record is not None:
                record["reconcile_pending"] = pending
                self.persist_job(job_id)

    def record(self, job_id, origin):
        with self.state:
            if job_id not in self.jobs:
                self.jobs[job_id] = {
                    "job_id": job_id,
                    "origin": origin,
                    "first_seen": now_iso(),
                    "create": None,
                    "chain": None,
                    "receipts": [],
                    "ops": [],
                    "reconcile_pending": False,
                }
                self.persist_job(job_id)
            return self.jobs[job_id]

    # --- views ---

    def op_summary(self, op):
        transaction = op.get("transaction") or {}
        return {
            "op_id": op["op_id"],
            "kind": op["kind"],
            "negative_kind": op.get("negative_kind"),
            "job_id": op.get("job_id"),
            "status": op["status"],
            "outcome": op.get("outcome"),
            "signature": transaction.get("signature") or op.get("signature"),
            "explorer": transaction.get("explorer"),
            "rejection": op.get("rejection"),
            "receipt_job_id": op.get("receipt_job_id"),
            "params": op.get("params"),
            "started_at": op["started_at"],
            "ended_at": op.get("ended_at"),
        }

    def running_op(self):
        return self.ops.get(self.current) if self.current else None

    def state_of(self, record):
        """Displayed state and its scope. Economic truth is `chain`."""
        op = self.running_op()
        if op is not None and op.get("job_id") == record["job_id"]:
            if op["kind"] == "prove":
                return "Proving", "local"
            if op.get("signature"):
                return "Submitted", "worker"
        if record["job_id"] in self.pending:
            return "Submitted", "worker"
        chain = record.get("chain")
        if chain is None:
            return None, None
        if chain.get("absent"):
            return ("Failed", "worker") if record["origin"] == "worker" else (None, None)
        if chain["status"] == "Funded" and record["receipts"] and record["receipts"][-1]["status"] == "failed":
            return "Failed", "worker"
        return chain["status"], "chain"

    def public_receipt(self, attempt):
        return {key: value for key, value in attempt.items() if key != "dir"}

    def receipt_view(self, attempt):
        """The public receipt plus what its proving operation printed: the
        local checks (exact CLI keys) and, for a usable receipt, the journal
        decoded by the frozen offsets (`compress.journal_hex`, which must equal
        `prove.journal_hex`)."""
        view = self.public_receipt(attempt)
        op = self.ops.get(attempt.get("op_id")) or {}
        lines = {step.get("name"): (step.get("stdout") or "").splitlines() for step in op.get("steps") or []}
        prove, compress = lines.get("prove", []), lines.get("compress", [])
        local = {key: line_value(prove, key) for key in LOCAL_PROVE_KEYS}
        local.update({key: line_value(compress, key) for key in LOCAL_COMPRESS_KEYS})
        view["local"] = local
        view["journal"], view["journal_error"] = None, None
        if attempt.get("status") != "usable":
            view["journal_error"] = "no usable receipt"
            return view
        compressed, proved = line_value(compress, "compress.journal_hex"), line_value(prove, "prove.journal_hex")
        if compressed is None or compressed != proved:
            view["journal_error"] = "compress.journal_hex is missing or differs from prove.journal_hex"
            return view
        try:
            view["journal"] = {**decode_journal(compressed), "source": "compress.journal_hex"}
        except ValueError as error:
            view["journal_error"] = str(error)
        return view

    def job_view(self, job_id):
        with self.state:
            record = self.jobs.get(job_id)
            if record is None:
                raise HttpError(404, "unknown_job", "this worker has not seen this job_id; POST …/show reads it")
            state, scope = self.state_of(record)
            op = self.running_op()
            return copy(
                {
                    "job_id": job_id,
                    "origin": record["origin"],
                    "state": state,
                    "state_scope": scope,
                    "chain": record["chain"],
                    "create": record["create"],
                    "receipt": self.receipt_view(record["receipts"][-1]) if record["receipts"] else None,
                    "reconcile_pending": job_id in self.pending,
                    "running_op": self.op_summary(op) if op is not None and op.get("job_id") == job_id else None,
                    "ops": [self.op_summary(self.ops[op_id]) for op_id in record["ops"] if op_id in self.ops],
                }
            )

    def list_jobs(self):
        with self.state:
            jobs = []
            for job_id, record in sorted(self.jobs.items(), key=lambda item: item[1]["first_seen"]):
                state, scope = self.state_of(record)
                chain = record["chain"] or {}
                receipt = record["receipts"][-1] if record["receipts"] else {}
                jobs.append(
                    {
                        "job_id": job_id,
                        "origin": record["origin"],
                        "state": state,
                        "state_scope": scope,
                        "chain_status": chain.get("status"),
                        "reconcile_pending": job_id in self.pending,
                        "first_seen": record["first_seen"],
                        "amount": chain.get("amount"),
                        "deadline_slot": chain.get("deadline_slot"),
                        "read_slot": chain.get("slot"),
                        "read_at": chain.get("read_at"),
                        "past_deadline": chain.get("past_deadline"),
                        "receipt_status": receipt.get("status"),
                        "receipt_verdict": receipt.get("verdict"),
                    }
                )
            return {"jobs": jobs}

    def health(self):
        with self.state:
            op = self.running_op()
            return copy(
                {
                    "worker": "vericode-worker (D10)",
                    "startup": self.startup_report,
                    "shim_now": self.shim_status(),
                    "running_op": self.op_summary(op) if op is not None else None,
                    "reconcile_pending": sorted(self.pending),
                    "incidents": self.incidents,
                    "v1": self.v1_facts(),
                    "slot_clock": self.slot_clock(),
                    "app_commit": self.app_commit,
                    "ui": STATIC_INDEX,
                }
            )

    def v1_facts(self):
        """What the interface shows before a create: the v1 terms and the
        programs, parsed from the startup `check` lines; the pubkeys from the
        key probes; the worker's fixed amount and deadline window."""
        report = self.startup_report or {}
        cli_lines = (report.get("checks") or {}).get("cli") or []
        prover_lines = (report.get("checks") or {}).get("prover") or []
        found = {}
        for line in cli_lines:
            for name, pattern in FACT_RES.items():
                if name not in found and (match := pattern.fullmatch(line)):
                    found[name] = match.groups()

        def part(name, keys, ints=()):
            if name not in found:
                return None
            return {key: int(value) if key in ints else value for key, value in zip(keys, found[name])}

        escrow = part("escrow", ("program_id", "upgrade_authority", "deployed_slot"), ("deployed_slot",))
        if escrow is not None and "escrow_data" in found:
            escrow.update(program_data_bytes=int(found["escrow_data"][0]), program_data_sha256=found["escrow_data"][1])
        verifier = part("verifier", ("program_id", "program_data", "upgrade_authority", "bytes", "sha256"), ("bytes",))
        if verifier is not None:
            verifier["release"] = VERIFIER_RELEASE
        probes = report.get("probes") or {}
        return {
            "cluster": part("cluster", ("name", "genesis")),
            "escrow": escrow,
            "verifier": verifier,
            "mint": part("mint", ("address", "decimals", "freeze_authority", "supply"), ("decimals", "supply")),
            "terms": part("terms", ("spec_hash", "harness_hash", "image_id")),
            "guest": {"image_id": line_value(prover_lines, "guest.image_id"),
                      "admitted": line_value(prover_lines, "guest.admitted"),
                      "selector": line_value(prover_lines, "groth16.selector")},
            "buyer": probes.get("buyer"),
            "executor": probes.get("executor"),
            "amount": AMOUNT,
            "deadline_offset": {"min": MIN_DEADLINE_OFFSET, "max": MAX_DEADLINE_OFFSET},
            "c10_8_margin": C10_8_MARGIN_SLOTS,
        }

    def slot_clock(self):
        """The last slot read from the chain (a `job show`), and seconds per
        slot measured between the oldest and the newest read this worker
        recorded (null under 10 minutes apart). An estimate, never a deadline."""
        reads = []
        for op in self.ops.values():
            chain = op.get("chain") or {}
            if type(chain.get("slot")) is int and (at := parse_iso(chain.get("read_at"))) is not None:
                reads.append((at, chain["slot"], chain["read_at"], op.get("job_id")))
        for job_id, record in self.jobs.items():
            chain = record.get("chain") or {}
            if type(chain.get("slot")) is int and (at := parse_iso(chain.get("read_at"))) is not None:
                reads.append((at, chain["slot"], chain["read_at"], job_id))
        if not reads:
            return {"latest": None, "reference": None, "seconds_per_slot": None}
        first, last = min(reads), max(reads)
        rate = None
        if last[0] - first[0] >= MIN_SLOT_CLOCK_SECONDS and last[1] > first[1]:
            rate = round((last[0] - first[0]) / (last[1] - first[1]), 4)
        return {
            "latest": {"slot": last[1], "read_at": last[2], "job_id": last[3]},
            "reference": {"slot": first[1], "read_at": first[2], "job_id": first[3]},
            "seconds_per_slot": rate,
        }

    def op_view(self, op_id):
        with self.state:
            op = self.ops.get(op_id)
            if op is None:
                raise HttpError(404, "unknown_op")
            view = json.loads(self.scrub_paths(json.dumps(op)))
            if op["kind"] in WRITE_KINDS:
                view["anatomy"] = self.anatomy_of(op)
            return view

    def anatomy_of(self, op):
        """Instructions, invocations and token balances of the operation's
        transaction, from the public fields of the CLI log (`logs`, `programs`,
        `pre/post_token_balances`). None when no transaction was recorded."""
        entries = read_tx_entries(self.data / "ops" / op["op_id"] / "cli-tx.jsonl")
        if not entries:
            return None
        entry = entries[-1]
        logs = [line for line in entry.get("logs") or [] if isinstance(line, str)]
        programs = [item for item in entry.get("programs") or [] if isinstance(item, str) and PUBKEY_RE.fullmatch(item)]
        record = self.jobs.get(op.get("job_id")) or {}
        landed = isinstance(entry.get("slot"), int)
        return {
            "landed": landed,
            "top_level": [{"program_id": item, "program": PROGRAM_KEYS.get(item)} for item in programs],
            "invocations": invocations_of(logs) if landed else [],
            "balances": balances_of(entry, record.get("chain")) if landed else [],
        }

    # --- operations ---

    def acquire(self, allow_pending_show_of=None):
        """One CLI/prover operation at a time (C10-4); nothing but the show of
        a Job awaiting reconciliation while one is pending (C10-5)."""
        if not self.busy.acquire(blocking=False):
            with self.state:
                op = self.running_op()
                extra = {"running_op": self.op_summary(op)} if op is not None else {}
            raise HttpError(409, "busy", "another operation is running", extra)
        with self.state:
            pending = sorted(self.pending)
        if pending and allow_pending_show_of not in pending:
            self.busy.release()
            raise HttpError(409, "reconcile_pending", "POST /api/jobs/<job_id>/show for these Jobs first", {"jobs": pending})

    def new_op(self, kind, job_id=None, negative_kind=None, params=None):
        op_id = secrets.token_hex(8)
        os.makedirs(self.data / "ops" / op_id, mode=0o700)
        op = {
            "op_id": op_id,
            "kind": kind,
            "negative_kind": negative_kind,
            "job_id": job_id,
            "params": params,
            "status": "running",
            "started_at": now_iso(),
            "ended_at": None,
            "steps": [],
            "outcome": None,
            "transaction": None,
            "expect_error": None,
            "rejection": None,
            "signature": None,
            "reconciled": None,
            "error": None,
        }
        if params and "receipt_job_id" in params:
            op["receipt_job_id"] = params["receipt_job_id"]  # escrow-6014: whose receipt it is (CR6)
        with self.state:
            self.ops[op_id] = op
            self.current = op_id
            if job_id is not None and job_id in self.jobs:
                self.jobs[job_id]["ops"].append(op_id)
                self.persist_job(job_id)
        self.persist_op(op)
        self.log_event({"event": "start", "op_id": op_id, "kind": kind, "negative_kind": negative_kind, "job_id": job_id})
        return op

    def finish(self, op, error=None):
        with self.state:
            if error is not None:
                op["status"], op["error"] = "failed", error
            elif op["status"] == "running":
                op["status"] = "ok"
            op["ended_at"] = now_iso()
            op["running_step"] = None
            self.persist_op(op)
            if self.current == op["op_id"]:
                self.current = None
        self.log_event(
            {
                "event": "end",
                "op_id": op["op_id"],
                "status": op["status"],
                "outcome": op.get("outcome"),
                "signature": (op.get("transaction") or {}).get("signature") or op.get("signature"),
                "error": op.get("error"),
            }
        )

    def run_op(self, op, body):
        """Runs `body(op)` and records the outcome; the caller holds the lock."""
        try:
            body(op)
            self.finish(op)
        except (OpFailed, WorkerError, OSError) as error:
            self.finish(op, str(error))
        except Exception as error:  # an internal error must still end the op
            self.finish(op, f"internal error: {type(error).__name__}: {error}")

    def start_async(self, op, body):
        def target():
            try:
                self.run_op(op, body)
            finally:
                self.busy.release()

        threading.Thread(target=target, daemon=True).start()
        return {"op_id": op["op_id"], "kind": op["kind"], "job_id": op.get("job_id"), "status": "running"}

    def run_step(self, op, name, argv, *, prover=False, timeout, on_line=None):
        self.verify_binary(argv[0])
        env = self.child_env(prover)
        started_at = now_iso()
        with self.state:
            op["running_step"] = {"name": name, "started_at": started_at}
            self.persist_op(op)
        result = run_process(argv, env, self.config.home_dir, timeout, on_line)
        step = {
            "name": name,
            "argv": self.public_argv(argv),
            "env": env,
            "started_at": started_at,
            "seconds": round(result.seconds, 1),
            "exit_code": result.exit_code,
            "timed_out": result.timed_out,
            "stdout": self.scrub("\n".join(result.stdout), f"{op['op_id']} {name} stdout"),
            "stderr": self.scrub(result.stderr, f"{op['op_id']} {name} stderr"),
            "hook_errors": result.hook_errors,
        }
        with self.state:
            op["steps"].append(step)
            op["running_step"] = None
            self.persist_op(op)
        self.log_event({"event": "step", "op_id": op["op_id"], "name": name, "exit_code": result.exit_code,
                        "seconds": step["seconds"], "timed_out": result.timed_out})
        return result

    def note_signature(self, op, line):
        match = SENT_RE.fullmatch(line) or ALREADY_RE.fullmatch(line)
        if match and SIG_RE.fullmatch(match.group("sig")):
            with self.state:
                op["signature"] = match.group("sig")
                self.persist_op(op)
            self.log_event({"event": "signature", "op_id": op["op_id"], "signature": match.group("sig")})

    def show_into(self, op, job_id, name):
        """Reads the Job from the chain; False when the read itself failed."""
        argv = [self.config.cli, "job", "show", "--job-id", job_id]
        result = self.run_step(op, name, argv, timeout=TIMEOUTS["show"])
        if result.exit_code == 0 and not result.timed_out:
            try:
                chain = parse_show(result.stdout, job_id)
            except ValueError as error:
                with self.state:
                    op.setdefault("notes", []).append(f"{name}: {error}")
                return False
        elif result.exit_code == 1 and any(
            (match := NO_JOB_RE.fullmatch(line)) and match.group("job_id") == job_id
            for line in result.stderr.splitlines()
        ):
            chain = {"absent": True}
        else:
            return False
        chain["read_at"] = now_iso()
        with self.state:
            if job_id not in self.jobs and chain.get("absent"):
                op["chain"] = chain
                return True
            record = self.record(job_id, "external")
            record["chain"] = chain
            if op["op_id"] not in record["ops"]:
                record["ops"].append(op["op_id"])
            op["chain"] = chain
            self.persist_job(job_id)
        return True

    def reconcile(self, op, job_id):
        """Every write ends with `job show` (C10-5); a failed read leaves the
        Job pending, and the worker refuses everything else until it is read."""
        ok = self.show_into(op, job_id, "show (reconcile)")
        with self.state:
            op["reconciled"] = ok
        self.set_pending(job_id, not ok)
        return ok

    def write_step(self, op, argv, expect=None, on_line=None):
        """One CLI write: the outcome comes from the CLI log and the chain,
        never from the exit code alone. No automatic retry."""
        log = self.data / "ops" / op["op_id"] / "cli-tx.jsonl"
        assert argv[1:3] == ["--log", str(log)]

        def hook(line):
            if on_line is not None:
                on_line(line)
            self.note_signature(op, line)

        result = self.run_step(op, "cli", argv, timeout=TIMEOUTS["write"], on_line=hook)
        entries = read_tx_entries(log)
        entry = entries[-1] if entries else None
        with self.state:
            op["cli_exit_code"] = result.exit_code
            op["transaction"] = public_tx(entry, op.get("signature"))
            if entry is not None and entry.get("outcome") in OUTCOMES:
                op["outcome"] = entry["outcome"]
            else:
                op["outcome"] = "UNKNOWN" if op.get("signature") else "NO_TRANSACTION"
            if expect is not None:
                op["rejection"] = rejection_of(expect, result.stdout, entry)
            if result.exit_code != 0 or result.timed_out:
                lines = [line for line in self.scrub(result.stderr, "cli stderr").splitlines() if line.strip()]
                op["cli_error"] = "timed out" if result.timed_out else (lines[-1] if lines else f"exit {result.exit_code}")
        return result

    def chain_status(self, job_id):
        with self.state:
            chain = self.jobs[job_id].get("chain") or {}
            return chain.get("status")

    # create
    def op_create(self, op, offset):
        config = self.config
        log = self.data / "ops" / op["op_id"] / "cli-tx.jsonl"
        argv = [config.cli, "--log", str(log), "job", "create", "--buyer-keypair", config.buyer_keypair,
                "--executor", config.executor_pubkey, "--amount", str(AMOUNT), "--deadline-offset", str(offset)]

        def on_job_id(line):
            for key, pattern in CREATE_LIVE_RES.items():
                if live := pattern.fullmatch(line):
                    with self.state:
                        value = live.group(1) if key != "slot" else {
                            "slot": int(live.group(1)), "deadline_slot": int(live.group(2)), "offset": int(live.group(3))}
                        op.setdefault("create_live", {})[key] = value
                        self.persist_op(op)
            match = CREATE_ID_RE.fullmatch(line)
            if match and op["job_id"] is None:
                job_id = match.group("job_id")
                with self.state:
                    record = self.record(job_id, "worker")
                    record["create"] = {"op_id": op["op_id"], "deadline_offset": offset, "signature": None}
                    record["ops"].append(op["op_id"])
                    op["job_id"] = job_id
                    self.persist_job(job_id)
                    self.persist_op(op)
                # Registered before the transaction: the CLI prints the
                # job_id before it simulates (C10-5).
                self.log_event({"event": "job_id", "op_id": op["op_id"], "job_id": job_id})

        result = self.write_step(op, argv, on_line=on_job_id)
        job_id = op["job_id"]
        if job_id is None:
            raise OpFailed("create stopped before choosing a job_id; nothing was sent: "
                           + op.get("cli_error", f"exit {result.exit_code}"))
        if not self.reconcile(op, job_id):
            raise OpFailed("job show failed after create; the Job stays pending until POST …/show reads it")
        with self.state:
            record = self.jobs[job_id]
            transaction = op["transaction"] or {}
            if op["outcome"] == "PASS":
                record["create"].update(signature=transaction.get("signature"), slot=transaction.get("slot"),
                                        explorer=transaction.get("explorer"))
                self.persist_job(job_id)
        status = self.chain_status(job_id)
        if op["outcome"] != "PASS" or status != "Funded":
            raise OpFailed(f"create outcome {op['outcome']}, chain {status or 'absent'}; exit {result.exit_code}")

    # prove + compress
    def op_prove(self, op, job_id, value_in, value_out):
        with self.state:
            record = self.jobs[job_id]
            n = len(record["receipts"]) + 1
            directory = self.data / "receipts" / job_id / str(n)
            attempt = {"n": n, "op_id": op["op_id"], "status": "running", "dir": str(directory),
                       "artifact": {"input": value_in, "claimed_output": value_out}}
            record["receipts"].append(attempt)
            self.persist_job(job_id)
        try:
            self.prove_steps(op, job_id, value_in, value_out, directory, attempt)
        except BaseException as error:  # a receipt is usable only when every check passed
            with self.state:
                if attempt["status"] == "running":
                    attempt.update(status="failed", failure=str(error) or type(error).__name__)
                    self.persist_job(job_id)
            raise

    def prove_steps(self, op, job_id, value_in, value_out, directory, attempt):
        config = self.config
        os.makedirs(directory.parent, mode=0o700, exist_ok=True)
        if directory.exists():
            raise OpFailed("receipt directory already exists")

        def fail(reason):
            raise OpFailed(reason)

        result = self.run_step(op, "prove", [config.prover, "prove", job_id, str(value_in), str(value_out), str(directory)],
                               prover=True, timeout=TIMEOUTS["prove"])
        lines = result.stdout
        verdict = line_value(lines, "prove.verdict")
        expected = [f"prove.job_id={job_id}", "prove.receipt_type=Composite", "prove.local_verify=ok",
                    "prove.journal_equal_to_core=true"]
        artifact_ok = (line_value(lines, "prove.artifact") or "").startswith(f"({value_in},{value_out}) ")
        if result.exit_code != 0 or result.timed_out or any(line not in lines for line in expected) \
                or verdict not in ("PASS", "FAIL") or not artifact_ok:
            fail(f"prove failed (exit {result.exit_code}); no receipt")
        # C10-4: compress only with enough memory.
        try:
            available = self.mem_available_kb()
        except (OSError, ValueError, WorkerError) as error:
            fail(f"MemAvailable unreadable ({error}); compress not started (C10-4)")
        with self.state:
            attempt["mem_available_kb_before_compress"] = available
        if available < MIN_MEM_AVAILABLE_KB:
            fail(f"MemAvailable {available} kB < {MIN_MEM_AVAILABLE_KB} kB (2.5 GiB); compress not started (C10-4)")
        # C10-2: the shim, right before compress.
        try:
            shim = self.verify_shim()
        except (OSError, WorkerError) as error:
            fail(f"compress not started: {error}")
        with self.state:
            attempt["shim"] = shim
        started = time.time()
        result = self.run_step(op, "compress", [config.prover, "compress", str(directory)], prover=True,
                               timeout=TIMEOUTS["compress"])
        ended = time.time()
        lines = result.stdout
        if result.exit_code != 0 or result.timed_out:
            fail(f"compress failed (exit {result.exit_code}); receipt not used")
        expected = ["compress.receipt_type=Groth16", "compress.local_verify=ok", "compress.journal_equal_to_composite=true",
                    f"compress.image_id={ADMITTED_IMAGE_ID}", f"compress.docker_shim={os.path.dirname(config.shim)}",
                    f"compress.verdict={verdict}"]
        missing = [line for line in expected if line not in lines]
        if missing:
            fail(f"compress output lacks {missing}; receipt not used")
        runs = [line[len("compress.docker_run="):] for line in lines if line.startswith("compress.docker_run=")]
        if len(runs) != 1:
            fail(f"C10-2: {len(runs)} compress.docker_run lines instead of exactly one; receipt not used")
        match = DOCKER_RUN_RE.fullmatch(runs[0])
        if match is None:
            fail("C10-2: the docker_run line is not the shim's pinned run command; receipt not used")
        if match.group("dir") != os.path.realpath(directory):
            fail("C10-2: the docker_run line names another receipt directory; receipt not used")
        try:
            when = datetime.datetime.fromisoformat(match.group("ts")).timestamp()
        except ValueError:
            fail("C10-2: the docker_run line has no valid time; receipt not used")
        if not started - 1 <= when <= ended + 1:
            fail("C10-2: the docker_run line is not from this compress run; receipt not used")
        try:
            shim_log = (directory / "docker-shim.log").read_text()
        except OSError:
            shim_log = None
        if shim_log != runs[0] + "\n":
            fail("C10-2: docker-shim.log is not exactly the printed run line; receipt not used")
        with self.state:
            attempt.update(
                status="usable",
                verdict=verdict,
                docker_run=runs[0],
                journal_sha256=line_value(lines, "compress.journal_sha256"),
                journal_digest=line_value(lines, "compress.journal_digest"),
                seal_sha256=line_value(lines, "compress.seal_sha256"),
                groth16_receipt_sha256=line_value(lines, "compress.groth16_receipt_sha256"),
                compress_seconds=line_value(lines, "compress.seconds"),
            )
            op["receipt"] = self.public_receipt(attempt)
            self.persist_job(job_id)

    def usable_receipt_dir(self, job_id):
        with self.state:
            record = self.jobs.get(job_id)
            if not record or not record["receipts"] or record["receipts"][-1]["status"] != "usable":
                raise HttpError(409, "no_usable_receipt", "POST …/prove first; a failed proof is never used")
            return record["receipts"][-1]["dir"]

    def receipt_dir_now(self, job_id):
        """The receipt to submit, resolved again under the lock."""
        try:
            return self.usable_receipt_dir(job_id)
        except HttpError as error:
            raise OpFailed(f"no usable receipt for {job_id}: {error.detail}") from None

    # settle (positive)
    def op_settle(self, op, job_id):
        config = self.config
        receipt_dir = self.receipt_dir_now(job_id)
        log = self.data / "ops" / op["op_id"] / "cli-tx.jsonl"
        self.write_step(op, [config.cli, "--log", str(log), "job", "settle", "--job-id", job_id, "--receipt", receipt_dir,
                             "--deliver", "--executor-keypair", config.executor_keypair])
        if not self.reconcile(op, job_id):
            raise OpFailed("job show failed after settle; the Job stays pending until POST …/show reads it")
        status = self.chain_status(job_id)
        transaction = op["transaction"] or {}
        if op["outcome"] != "PASS" or not transaction.get("verifier_invoked") or status not in ("Released", "RefundedOnFail"):
            raise OpFailed(f"settle outcome {op['outcome']}, chain {status}; exit {op['cli_exit_code']}")

    # refund-timeout (positive)
    def op_refund(self, op, job_id):
        config = self.config
        log = self.data / "ops" / op["op_id"] / "cli-tx.jsonl"
        self.write_step(op, [config.cli, "--log", str(log), "job", "refund-timeout", "--job-id", job_id,
                             "--payer-keypair", config.buyer_keypair])
        if not self.reconcile(op, job_id):
            raise OpFailed("job show failed after refund-timeout; the Job stays pending until POST …/show reads it")
        status = self.chain_status(job_id)
        if op["outcome"] != "PASS" or status != "RefundedOnTimeout":
            raise OpFailed(f"refund-timeout outcome {op['outcome']}, chain {status}; exit {op['cli_exit_code']}")

    # negative runs (C10-7, C10-8)
    def op_negative(self, op, job_id, kind, receipt_job_id=None):
        config = self.config
        program, code = NEGATIVE_KINDS[kind]
        if kind != "escrow-6021":
            receipt_dir = self.receipt_dir_now(receipt_job_id if kind == "escrow-6014" else job_id)
        log = self.data / "ops" / op["op_id"] / "cli-tx.jsonl"
        base = [config.cli, "--log", str(log), "job"]
        expect_arg = ["--expect-error", f"{program}:{code}"]
        with self.state:
            op["expect_error"] = f"{program}:{code}"
        if kind == "escrow-6021":
            if not self.show_into(op, job_id, "show (C10-8)"):
                raise OpFailed("job show failed right before the escrow:6021 run; nothing was sent")
            with self.state:
                chain = self.jobs[job_id]["chain"]
            if chain.get("absent"):
                raise OpFailed("the Job is absent; nothing was sent")
            margin = chain["deadline_slot"] - chain["slot"]
            with self.state:
                op["c10_8"] = {"slot": chain["slot"], "deadline_slot": chain["deadline_slot"], "margin": margin,
                               "required": C10_8_MARGIN_SLOTS}
            if margin < C10_8_MARGIN_SLOTS:
                raise OpFailed(f"C10-8: deadline_slot - slot = {margin} < {C10_8_MARGIN_SLOTS}; nothing was sent")
            argv = base + ["refund-timeout", "--job-id", job_id, "--payer-keypair", config.buyer_keypair] + expect_arg
        elif kind == "verifier-6003":
            argv = base + ["settle", "--job-id", job_id, "--receipt", receipt_dir, "--deliver",
                           "--executor-keypair", config.executor_keypair, "--tamper-seal"] + expect_arg
        elif kind == "escrow-6007":
            argv = base + ["settle", "--job-id", job_id, "--receipt", receipt_dir,
                           "--payer-keypair", config.executor_keypair] + expect_arg
        else:  # escrow-6014, with the receipt of another Job of this worker
            argv = base + ["settle", "--job-id", job_id, "--receipt", receipt_dir, "--deliver",
                           "--executor-keypair", config.executor_keypair] + expect_arg
        self.write_step(op, argv, expect=(program, code))
        if not self.reconcile(op, job_id):
            raise OpFailed("job show failed after the negative run; the Job stays pending until POST …/show reads it")
        if op["rejection"] is None:
            raise OpFailed(f"negative {op['expect_error']}: outcome {op['outcome']}, no rejection shown by the CLI")

    def op_check(self, op):
        config = self.config
        results = {}
        for name, argv, prover, lines, prefixes in (
            ("cli", [config.cli, "check"], False, CHECK_LINES, CHECK_PREFIXES),
            ("prover", [config.prover, "check"], True, PROVER_CHECK_LINES, ()),
        ):
            result = self.run_step(op, f"check {name}", argv, prover=prover, timeout=TIMEOUTS["check"])
            missing = [line for line in lines if line not in result.stdout]
            missing += [prefix for prefix in prefixes if not any(line.startswith(prefix) for line in result.stdout)]
            results[name] = {"ok": result.exit_code == 0 and not missing, "missing": missing}
        with self.state:
            op["checks"] = results
        if not all(item["ok"] for item in results.values()):
            raise OpFailed("a preflight check failed")

    # --- HTTP routing ---

    def worker_job(self, job_id):
        with self.state:
            record = self.jobs.get(job_id)
            if record is None:
                raise HttpError(404, "unknown_job", "this worker has not seen this job_id")
            if record["origin"] != "worker":
                raise HttpError(409, "not_a_worker_job", "only Jobs created by this worker are proved or settled here")
            return record

    def route(self, method, path, body):
        if method == "GET":
            if path == "/api/health":
                return 200, self.health()
            if path == "/api/jobs":
                return 200, self.list_jobs()
            if match := re.fullmatch(r"/api/jobs/([^/]+)", path):
                return 200, self.job_view(job_id_param(match.group(1)))
            if match := re.fullmatch(r"/api/ops/([^/]+)", path):
                op_id = match.group(1)
                if not OP_ID_RE.fullmatch(op_id):
                    raise HttpError(400, "bad_op_id")
                return 200, self.op_view(op_id)
            if re.fullmatch(r"/api/(check|jobs/[^/]+/(show|prove|settle|refund-timeout|negative/[^/]+))", path):
                raise HttpError(405, "method_not_allowed")
            raise HttpError(404, "not_found")
        if path == "/api/check":
            expect_keys(body, {})
            return self.sync_check()
        if path == "/api/jobs":
            expect_keys(body, {"deadline_offset": int})
            offset = body["deadline_offset"]
            if not MIN_DEADLINE_OFFSET <= offset <= MAX_DEADLINE_OFFSET:
                raise HttpError(400, "bad_deadline_offset", f"deadline_offset must be in [{MIN_DEADLINE_OFFSET}, {MAX_DEADLINE_OFFSET}]")
            self.acquire()
            return self.launch("create", None, lambda op: self.op_create(op, offset), params={"deadline_offset": offset})
        if match := re.fullmatch(r"/api/jobs/([^/]+)/(show|prove|settle|refund-timeout)", path):
            job_id, action = job_id_param(match.group(1)), match.group(2)
            if action == "show":
                expect_keys(body, {})
                self.acquire(allow_pending_show_of=job_id)
                return self.sync_show(job_id)
            if action == "prove":
                expect_keys(body, {"input": int, "claimed_output": int})
                value_in, value_out = body["input"], body["claimed_output"]
                if not (0 <= value_in <= U32_MAX and 0 <= value_out <= U32_MAX):
                    raise HttpError(400, "bad_u32", "input and claimed_output must be u32")
                record = self.worker_job(job_id)
                chain = record.get("chain") or {}
                if chain.get("status") != "Funded":
                    raise HttpError(409, "job_not_funded", "the last job show must say Funded")
                if chain.get("executor") != self.config.executor_pubkey:
                    raise HttpError(409, "executor_mismatch", "the Job executor is not this worker's executor")
                self.acquire()
                return self.launch("prove", job_id, lambda op: self.op_prove(op, job_id, value_in, value_out),
                                   params={"input": value_in, "claimed_output": value_out})
            if action == "settle":
                expect_keys(body, {})
                self.worker_job(job_id)
                self.usable_receipt_dir(job_id)  # early 409; resolved again under the lock
                self.acquire()
                return self.launch("settle", job_id, lambda op: self.op_settle(op, job_id))
            expect_keys(body, {})  # refund-timeout: any Job, also one of another origin (W1, W2)
            self.acquire()
            return self.launch("refund-timeout", job_id, lambda op: self.op_refund(op, job_id),
                               prepare=lambda: self.record(job_id, "external"))
        if match := re.fullmatch(r"/api/jobs/([^/]+)/negative/([^/]+)", path):
            job_id, kind = job_id_param(match.group(1)), match.group(2)
            if kind not in NEGATIVE_KINDS:
                raise HttpError(400, "bad_kind", f"kind must be one of {sorted(NEGATIVE_KINDS)}")
            self.worker_job(job_id)
            params, other = None, None
            if kind == "escrow-6014":
                expect_keys(body, {"receipt_job_id": str})
                other = job_id_param(body["receipt_job_id"])
                if other == job_id:
                    raise HttpError(400, "same_job", "escrow-6014 needs the receipt of another Job")
                self.worker_job(other)
                self.usable_receipt_dir(other)  # early 409; resolved again under the lock
                params = {"receipt_job_id": other}
            else:
                expect_keys(body, {})
                if kind != "escrow-6021":
                    self.usable_receipt_dir(job_id)
            self.acquire()
            return self.launch("negative", job_id, lambda op: self.op_negative(op, job_id, kind, other),
                               negative_kind=kind, params=params)
        if re.fullmatch(r"/api/(health|jobs/[^/]+|ops/[^/]+)", path):
            raise HttpError(405, "method_not_allowed")
        raise HttpError(404, "not_found")

    def launch(self, kind, job_id, body, negative_kind=None, params=None, prepare=None):
        """Starts an asynchronous operation; the caller already holds the lock,
        which the operation thread releases."""
        try:
            if prepare is not None:
                prepare()
            op = self.new_op(kind, job_id, negative_kind, params)
            return 202, self.start_async(op, body)
        except BaseException:
            self.busy.release()
            raise

    def sync_check(self):
        self.acquire()
        try:
            op = self.new_op("check")
            self.run_op(op, self.op_check)
            return 200, {"op": self.op_view(op["op_id"])}
        finally:
            self.busy.release()

    def sync_show(self, job_id):
        """The caller already holds the lock."""
        try:
            op = self.new_op("show", job_id)

            def body(op):
                if not self.show_into(op, job_id, "show"):
                    raise OpFailed("job show failed; nothing changed")
                if job_id in self.jobs:
                    self.set_pending(job_id, False)

            self.run_op(op, body)
            with self.state:
                view = self.job_view(job_id) if job_id in self.jobs else {"job_id": job_id, "chain": op.get("chain")}
            return 200, {"op": self.op_view(op["op_id"]), "job": view}
        finally:
            self.busy.release()


def job_id_param(text):
    if not HEX64_RE.fullmatch(text) or text == FIXTURE_JOB_ID:
        raise HttpError(400, "bad_job_id", "job_id must be 64 lowercase hex digits (not the fixture id)")
    return text


def expect_keys(body, schema):
    """Exact keys; integers are JSON integers (never booleans or floats)."""
    if set(body) != set(schema):
        raise HttpError(400, "bad_body", f"expected exactly the keys {sorted(schema)}")
    for key, kind in schema.items():
        value = body[key]
        if kind is int and type(value) is not int:
            raise HttpError(400, "bad_body", f"{key} must be a JSON integer")
        if kind is str and not isinstance(value, str):
            raise HttpError(400, "bad_body", f"{key} must be a string")


# --- HTTP server ---------------------------------------------------------------


class Server(http.server.ThreadingHTTPServer):
    daemon_threads = True

    def __init__(self, worker, port):
        self.worker = worker
        super().__init__(("127.0.0.1", port), Handler)  # loopback only, fixed (C10-6)
        worker.port = self.server_address[1]


class Handler(http.server.BaseHTTPRequestHandler):
    server_version = "vericode-worker"
    sys_version = ""
    protocol_version = "HTTP/1.0"
    timeout = 30

    def do_GET(self):
        self.dispatch("GET")

    def do_POST(self):
        self.dispatch("POST")

    def refuse_method(self):
        self.reply(405, {"error": "method_not_allowed"}, allow=True)

    do_HEAD = do_PUT = do_DELETE = do_PATCH = do_OPTIONS = do_TRACE = do_CONNECT = refuse_method

    def send_error(self, code, message=None, explain=None):
        self.reply(code, {"error": f"http_{code}"})

    def log_message(self, format, *args):
        log = self.server.worker.http_log
        if log is not None:
            log.write(f"{now_iso()} {self.address_string()} {format % args}\n")

    def reply(self, status, payload, allow=False):
        text = json.dumps(payload, indent=1, sort_keys=True) + "\n"
        body = self.server.worker.scrub_paths(text).encode("utf-8")
        self.send_response(status)
        for name, value in SECURITY_HEADERS:
            self.send_header(name, value)
        if allow:
            self.send_header("Allow", "GET, POST")
        self.send_header("Content-Type", "application/json; charset=utf-8")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        if self.command != "HEAD":
            self.wfile.write(body)

    def send_static(self, name):
        """One file of the fixed table, with the page headers (no token: the
        page asks the operator for it)."""
        try:
            body = (STATIC_DIR / name).read_bytes()
        except OSError:
            raise HttpError(404, "not_found") from None
        self.send_response(200)
        for header, value in PAGE_HEADERS:
            self.send_header(header, value)
        self.send_header("Content-Type", CONTENT_TYPES[os.path.splitext(name)[1]])
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    def read_body(self):
        if self.headers.get_all("Transfer-Encoding"):
            raise HttpError(400, "chunked_not_allowed")
        lengths = self.headers.get_all("Content-Length") or []
        if len(lengths) > 1 or (lengths and not lengths[0].isdigit()):
            raise HttpError(400, "bad_content_length")
        length = int(lengths[0]) if lengths else 0
        if length > MAX_BODY:
            raise HttpError(413, "body_too_large")
        raw = self.rfile.read(length) if length else b""
        if not raw:
            return {}
        content_type = (self.headers.get("Content-Type") or "").split(";")[0].strip().lower()
        if content_type != "application/json":
            raise HttpError(415, "json_required")
        try:
            body = json.loads(raw.decode("utf-8"))
        except (UnicodeDecodeError, ValueError):
            raise HttpError(400, "bad_json") from None
        if not isinstance(body, dict):
            raise HttpError(400, "bad_json", "a JSON object is required")
        return body

    def dispatch(self, method):
        worker = self.server.worker
        try:
            hosts = self.headers.get_all("Host") or []
            if len(hosts) != 1:
                raise HttpError(400, "bad_host")
            if hosts[0] != f"127.0.0.1:{worker.port}":
                raise HttpError(421, "misdirected_request", "Host must be 127.0.0.1:<port>")
            if "?" in self.path or "#" in self.path:
                raise HttpError(400, "query_not_allowed")
            # http.server collapses a leading "//" into "/"; only the path as sent is routed.
            parts = self.requestline.split(" ")
            if len(parts) != 3 or parts[1] != self.path:
                raise HttpError(400, "bad_path")
            if self.path in STATIC_ROUTES:
                if method != "GET":
                    raise HttpError(405, "method_not_allowed")
                self.send_static(STATIC_ROUTES[self.path])
                return
            if not self.path.startswith("/api/"):
                raise HttpError(404, "not_found")
            tokens = self.headers.get_all("X-VeriCode-Token") or []
            if not tokens:
                raise HttpError(401, "token_required")
            if len(tokens) != 1 or not hmac.compare_digest(tokens[0].encode(), worker.token.encode()):
                raise HttpError(403, "bad_token")
            body = self.read_body() if method == "POST" else None
            status, payload = worker.route(method, self.path, body)
            self.reply(status, payload)
        except HttpError as error:
            self.reply(error.status, {"error": error.code, "detail": error.detail, **error.extra})
        except Exception as error:
            self.reply(500, {"error": "internal", "detail": type(error).__name__})


def main(argv=None):
    parser = argparse.ArgumentParser(description="VeriCode local worker (D10); see worker/README.md")
    parser.add_argument("--config", required=True, help="worker.json outside the clone, mode 0600")
    args = parser.parse_args(argv)
    os.umask(0o077)
    try:
        config = load_config(args.config)
        worker = Worker(config)
        report = worker.startup()
        server = Server(worker, config.port)
    except (WorkerError, OSError) as error:
        print(f"worker.refused={error}", file=sys.stderr)
        return 2
    for name, item in report["binaries"].items():
        print(f"worker.{name}={item['path']} sha256={item['sha256']}")
    for role, pubkey in report["probes"].items():
        print(f"worker.{role}={pubkey}")
    for name, lines in report["checks"].items():
        for line in lines:
            print(f"worker.check.{name}: {line}")
    print(f"worker.url=http://127.0.0.1:{worker.port}")
    print(f"worker.ui=http://127.0.0.1:{worker.port}{STATIC_INDEX}")
    # The token goes only to the operator's terminal, never to a worker log.
    print(f"worker.token={worker.token}", flush=True)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        server.server_close()
    return 0


if __name__ == "__main__":
    sys.exit(main())
