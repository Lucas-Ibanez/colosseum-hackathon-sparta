"""Offline tests of the D10 worker (unittest, standard library only).

The binaries are the TEST FAKES of `tests/fakes/`, copied to a temporary root
with their own hashes in the test configuration. Nothing here opens a network
connection, reads a real key or runs Docker. Run from the repository root,
with TMPDIR outside /tmp for this project:

    TMPDIR=<dir> python3 -B -m unittest discover -s worker/tests -v
"""

import hashlib
import http.client
import json
import os
import shutil
import sys
import tempfile
import threading
import time
import unittest
from pathlib import Path
from unittest import mock

sys.dont_write_bytecode = True
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import vericode_worker as vw  # noqa: E402

FAKES = Path(__file__).resolve().parent / "fakes"
BUYER = "EZgGUg4JhEAhzMPd4jBuKWFXdDNpFATvCjkj7mLAdxU6"
EXECUTOR = "EdB25bVhdj6rm5b7FbVHe5A3zx2hAhPpK4YAzrLaDs6U"
JOB = hashlib.sha256(b"vericode worker test job").hexdigest()
OTHER = hashlib.sha256(b"vericode worker test job, another one").hexdigest()
EXTERNAL = hashlib.sha256(b"vericode worker test job of another origin").hexdigest()
ALPHABET = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"
HOSTILE = {
    "BONSAI_API_KEY": "d10-test-not-a-key",
    "BONSAI_API_URL": "http://127.0.0.1:9",
    "RISC0_PROVER": "bonsai",
    "RISC0_DEV_MODE": "1",
    "RISC0_EXECUTOR": "ipc",
    "RISC0_WORK_DIR": "/etc",
    "VERICODE_REAL_DOCKER": "/bin/false",
    "VERICODE_DOCKER_SHIM_LOG": "/dev/null",
    "DOCKER_HOST": "tcp://127.0.0.1:2375",
    "DOCKER_CONTEXT": "elsewhere",
    "DOCKER_CONFIG": "/etc",
    "HTTP_PROXY": "http://127.0.0.1:9",
    "HTTPS_PROXY": "http://127.0.0.1:9",
    "ALL_PROXY": "socks5://127.0.0.1:9",
    "http_proxy": "http://127.0.0.1:9",
    "https_proxy": "http://127.0.0.1:9",
    "LD_PRELOAD": "/nonexistent/vericode-test.so",
    "PYTHONPATH": "/nonexistent",
}


def b58(data):
    number, out = int.from_bytes(data, "big"), ""
    while number:
        number, rest = divmod(number, 58)
        out = ALPHABET[rest] + out
    return "1" * (len(data) - len(data.lstrip(b"\0"))) + out


def fake_sig(name):
    return b58(hashlib.sha512(name.encode()).digest())


def fake_pubkey(name):
    return b58(hashlib.sha256(name.encode()).digest())


def sha256(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def custom(index, code):
    return {"InstructionError": [index, {"Custom": code}]}


def sim_line(label, program_id, code):
    return f"[{label}] simulation shows `Program {program_id} failed: custom program error: {code:#x}` as the innermost failure"


CHECK_OUTPUT = [
    vw.CHECK_LINES[0],
    vw.CHECK_PREFIXES[0] + "507798457",
    vw.CHECK_PREFIXES[1] + "1000000000000",
    "check.terms_v1 spec_hash=" + "af" * 32,
    vw.CHECK_LINES[1],
    vw.CHECK_LINES[2],
    vw.CHECK_LINES[3],
]
PROVER_CHECK_OUTPUT = ["guest.bytes=180300", *vw.PROVER_CHECK_LINES]
PROVE_OUTPUT = [
    "guest.bytes=180300",
    vw.PROVER_CHECK_LINES[0],
    vw.PROVER_CHECK_LINES[1],
    "prove.job_id={job_id}",
    "prove.artifact=({input},{output}) hex=01000000150000002a000000",
    "prove.frame_bytes=76",
    "prove.prover=LocalProver",
    "prove.seconds=7.9",
    "prove.receipt_type=Composite",
    "prove.local_verify=ok",
    "prove.negative.wrong_image=rejected",
    "prove.journal_equal_to_core=true",
    "prove.verdict=PASS",
    "prove.composite_receipt_bytes=221540",
    "prove.composite_receipt_sha256=" + "ec" * 32,
]
RUN_LINE = "{now} /usr/bin/docker --context default run --pull=never --network=none --rm -v {dir}/groth16-work:/mnt " + vw.GROTH16_IMAGE


def show_lines(status="Funded", slot=1_000, deadline=10_000, artifact=None, executor=EXECUTOR, vault=1_000_000):
    pda, vault_address = fake_pubkey("job pda"), fake_pubkey("vault pda")
    mint = "9TE2VPFmgrNxT22yS3sEZyRcMxLgJkwzgAoquWRXwV2F"
    return [
        "job.job_id={job_id}",
        f"job.address={pda}",
        f"job.explorer=https://explorer.solana.com/address/{pda}?cluster=devnet",
        "job.version=1 bump=255 vault_bump=254",
        f"job.buyer={BUYER}",
        f"job.executor={executor}",
        f"job.mint={mint} admitted=true",
        "job.amount=1000000",
        f"job.deadline_slot={deadline}",
        "job.spec_hash=" + "af" * 32,
        "job.harness_hash=" + "01" * 32,
        f"job.image_id={vw.ADMITTED_IMAGE_ID}",
        "job.terms_v1_admitted=true",
        f"job.status={status}" + (f" artifact_hash={artifact}" if artifact else ""),
        f"vault.address={vault_address} mint={mint} authority={pda} authority_is_job_pda=true delegate=None "
        f"close_authority=None amount={vault}",
        f"buyer.ata={fake_pubkey('buyer ata')} test_usdc=999992000000 sol_lamports=103351800",
        f"executor.ata={fake_pubkey('executor ata')} test_usdc=6000000 sol_lamports=29925000",
        f"slot={slot} deadline_slot={deadline} past_deadline={'true' if slot > deadline else 'false'}",
    ]


def show(**kwargs):
    return {"stdout": show_lines(**kwargs)}


RELEASED = show(status="Released", artifact="22" * 32, vault=0)


def tx(label, name, *, err=None, outcome="PASS", verifier=False, expect="Success", sim=None, exit=0, stderr="",
       before=(), pause=None, log=True):
    signature = fake_sig(name)
    logs = [f"Program {vw.ESCROW_ID} invoke [1]"]
    if verifier:
        logs += [f"Program {vw.VERIFIER_ID} invoke [2]",
                 f"Program {vw.VERIFIER_ID} consumed 99541 of 1180000 compute units",
                 f"Program {vw.VERIFIER_ID} success"]
    err_text = json.dumps(err, separators=(",", ":")) if err is not None else "null"
    stdout = list(before) + [f"[{label}] simulated err={err_text} units=9000 size=883 B verifier_invoked={str(verifier).lower()}"]
    if sim:
        stdout.append(sim)
    stdout += [
        f"[{label}] sent signature={signature} skip_preflight={str(err is not None).lower()}",
        f"[{label}] {outcome} slot=2000 err={err_text} units=9000 fee=5000 size=883 B verifier_invoked={str(verifier).lower()}",
        f"[{label}] explorer=https://explorer.solana.com/tx/{signature}?cluster=devnet",
    ]
    answer = {"stdout": stdout, "exit": exit, "stderr": stderr}
    if log:
        answer["log"] = [{
            "kind": "tx", "label": label, "expect": expect, "payer": BUYER, "signature": signature, "size": 883,
            "outcome": outcome, "slot": 2000, "err": err, "units": 9000, "fee": 5000, "logs": logs,
            "verifier_invoked": verifier, "explorer": f"https://explorer.solana.com/tx/{signature}?cluster=devnet",
            "snapshot": {"unchanged": True} if err is not None else None,
        }]
    if pause:
        answer["pause_after"] = {"prefix": f"[{label}] sent signature=", "seconds": pause}
    return answer


def create_answer(job_id, **kwargs):
    before = [f"buyer={BUYER}", f"create.job_id={job_id}", f"create.executor={EXECUTOR}", "create.amount=1000000"]
    return tx("create+fund", f"create {job_id}", before=before, **kwargs)


NEGATIVES = {
    "job refund-timeout escrow:6021": tx("refund_on_timeout", "6021", err=custom(0, 6021),
                                         sim=sim_line("refund_on_timeout", vw.ESCROW_ID, 6021)),
    "job settle verifier:6003": tx("deliver+release", "6003", err=custom(1, 6003), verifier=True,
                                   sim=sim_line("deliver+release", vw.VERIFIER_ID, 6003)),
    "job settle escrow:6007": tx("release", "6007", err=custom(0, 6007), sim=sim_line("release", vw.ESCROW_ID, 6007)),
    "job settle escrow:6014": tx("deliver+release", "6014", err=custom(1, 6014),
                                 sim=sim_line("deliver+release", vw.ESCROW_ID, 6014)),
}


def cli_scenario(**overrides):
    scenario = {
        "probe-buyer": [{"stdout": [f"buyer={BUYER}"], "stderr": vw.PROBE_ERRORS["buyer"] + "\n", "exit": 1}],
        "probe-executor": [{"stdout": [f"executor={EXECUTOR}"], "stderr": vw.PROBE_ERRORS["executor"] + "\n", "exit": 1}],
        "check": [{"stdout": CHECK_OUTPUT}],
        "job show": [show()],
        "job create": [create_answer(JOB)],
        "job settle": [tx("deliver+release", "settle", verifier=True)],
        "job refund-timeout": [tx("refund_on_timeout", "refund")],
        **{key: [answer] for key, answer in NEGATIVES.items()},
    }
    scenario.update(overrides)
    return scenario


def compress_output(shim_dir, run=RUN_LINE, docker_shim=None):
    return [
        "guest.bytes=180300",
        "compress.input_receipt_type=Composite",
        "compress.input_local_verify=ok",
        f"compress.docker_image={vw.GROTH16_IMAGE}",
        f"compress.docker_shim={docker_shim or shim_dir}",
        "compress.work_dir={dir}/groth16-work",
        "compress.prover=LocalProver",
        "compress.seconds=131.2",
        "compress.receipt_type=Groth16",
        "compress.local_verify=ok",
        "compress.negative.wrong_image=rejected",
        "compress.journal_equal_to_composite=true",
        "compress.groth16_receipt_bytes=827",
        "compress.groth16_receipt_sha256=" + "7a" * 32,
        "compress.selector=73c457ba",
        f"compress.image_id={vw.ADMITTED_IMAGE_ID}",
        "compress.verdict=PASS",
        "compress.journal_sha256=" + "62" * 32,
        "compress.journal_digest=" + "62" * 32,
        "compress.seal_sha256=" + "dd" * 32,
    ] + ([f"compress.docker_run={run}"] if run else [])


def prover_scenario(shim_dir, **overrides):
    scenario = {
        "check": [{"stdout": PROVER_CHECK_OUTPUT}],
        "prove": [{"stdout": PROVE_OUTPUT}],
        "compress": [{"stdout": compress_output(shim_dir), "shim_log": RUN_LINE + "\n"}],
    }
    scenario.update(overrides)
    return scenario


class Fixture:
    """A temporary root with the fakes, a fake shim, fake key files (never read
    by the worker) and the worker's directories, all mode 0700/0600."""

    def __init__(self, case, cli=None, prover=None, embed_shim=True):
        self.case = case
        self.root = Path(tempfile.mkdtemp(prefix="vcw-"))
        case.addCleanup(shutil.rmtree, self.root, True)
        os.chmod(self.root, 0o700)
        self.bin = self.root / "bin"
        self.bin.mkdir(mode=0o700)
        self.cli, self.prover = self.bin / "vericode", self.bin / "vericode-prover"
        shutil.copyfile(FAKES / "fake-vericode", self.cli)
        shutil.copyfile(FAKES / "fake-vericode-prover", self.prover)
        self.shim_dir = self.root / "src" / "prover" / "docker-shim"
        self.shim_dir.mkdir(parents=True)
        self.shim = self.shim_dir / "docker"
        self.shim.write_text("#!/bin/sh\n# TEST FAKE shim: never runs Docker\nexit 2\n")
        os.chmod(self.shim, 0o755)
        if embed_shim:  # the real prover embeds its shim source directory
            with open(self.prover, "a") as file:
                file.write(f"\n# embedded shim source dir (test): {self.shim_dir.parent}\n")
        os.chmod(self.cli, 0o755)
        os.chmod(self.prover, 0o755)
        self.keys = self.root / "keys"
        self.keys.mkdir(mode=0o700)
        self.key_bytes = json.dumps([(index * 37 + 11) % 256 for index in range(64)])
        for role in ("buyer", "executor"):
            path = self.keys / f"{role}.json"
            path.write_text(self.key_bytes)
            os.chmod(path, 0o600)
        for name in ("data", "home", "tmp"):
            (self.root / name).mkdir(mode=0o700)
            os.chmod(self.root / name, 0o700)
        self.data = self.root / "data"
        self.set_cli(cli or cli_scenario())
        self.set_prover(prover or prover_scenario(str(self.shim_dir)))
        self.mem_kb = 8_000_000
        self.responses = []
        self.server = None

    def set_cli(self, scenario):
        self._set("vericode", scenario)

    def set_prover(self, scenario):
        self._set("vericode-prover", scenario)

    def _set(self, name, scenario):
        (self.bin / f"{name}.scenario.json").write_text(json.dumps(scenario))
        state = self.bin / f"{name}.state.json"
        if state.exists():
            state.unlink()

    def config(self, **overrides):
        values = dict(
            cli=str(self.cli), cli_sha256=sha256(self.cli),
            prover=str(self.prover), prover_sha256=sha256(self.prover),
            shim=str(self.shim), shim_sha256=sha256(self.shim),
            buyer_keypair=str(self.keys / "buyer.json"), buyer_pubkey=BUYER,
            executor_keypair=str(self.keys / "executor.json"), executor_pubkey=EXECUTOR,
            data_dir=str(self.data), home_dir=str(self.root / "home"), tmp_dir=str(self.root / "tmp"), port=0,
        )
        values.update(overrides)
        return vw.Config(**values)

    def worker(self, **overrides):
        return vw.Worker(self.config(**overrides), mem_available_kb=lambda: self.mem_kb, http_log=None)

    def start(self, **overrides):
        self.stop()
        self.w = self.worker(**overrides)
        self.w.startup()
        self.server = vw.Server(self.w, 0)
        self.port = self.w.port
        threading.Thread(target=self.server.serve_forever, daemon=True).start()
        self.case.addCleanup(self.stop)
        return self.w

    def stop(self):
        if self.server is not None:
            self.server.shutdown()
            self.server.server_close()
            self.server = None

    def calls(self, name=None):
        path = self.bin / "calls.jsonl"
        calls = [json.loads(line) for line in path.read_text().splitlines()] if path.exists() else []
        return [call for call in calls if name is None or call["bin"] == name]

    def request(self, method, path, body=None, *, token=True, hosts=None, headers=(), raw=None,
                content_type="application/json"):
        connection = http.client.HTTPConnection("127.0.0.1", self.port, timeout=20)
        connection.putrequest(method, path, skip_host=True, skip_accept_encoding=True)
        for host in ([f"127.0.0.1:{self.port}"] if hosts is None else hosts):
            connection.putheader("Host", host)
        if token is True:
            token = self.w.token
        if token:
            connection.putheader("X-VeriCode-Token", token)
        data = raw if raw is not None else (json.dumps(body).encode() if body is not None else b"")
        if method == "POST" or data:
            connection.putheader("Content-Length", str(len(data)))
        if data and content_type:
            connection.putheader("Content-Type", content_type)
        for name, value in headers:
            connection.putheader(name, value)
        connection.endheaders(data or None)
        response = connection.getresponse()
        payload = response.read()
        connection.close()
        self.responses.append((response.getheaders(), payload.decode()))
        return response.status, dict((name.lower(), value) for name, value in response.getheaders()), (
            json.loads(payload) if payload else None)

    def post(self, path, body=None):
        return self.request("POST", path, {} if body is None else body)

    def wait(self, op_id, timeout=30):
        deadline = time.time() + timeout
        while time.time() < deadline:
            status, _, op = self.request("GET", f"/api/ops/{op_id}")
            assert status == 200, op
            if op["status"] != "running":
                return op
            time.sleep(0.05)
        raise AssertionError(f"op {op_id} still running")

    def run(self, path, body=None):
        status, _, started = self.post(path, body)
        assert status == 202, (status, started)
        return self.wait(started["op_id"])

    def job(self, job_id=JOB):
        status, _, view = self.request("GET", f"/api/jobs/{job_id}")
        assert status == 200, view
        return view

    def create(self, job_id=JOB):
        op = self.run("/api/jobs", {"deadline_offset": 9000})
        assert op["status"] == "ok" and op["job_id"] == job_id, op
        return op

    def prove(self, job_id=JOB):
        return self.run(f"/api/jobs/{job_id}/prove", {"input": 21, "claimed_output": 42})


# --- startup -------------------------------------------------------------------------


class StartupTests(unittest.TestCase):
    def test_a_binary_with_another_hash_is_refused_at_startup(self):
        fx = Fixture(self)
        for field in ("cli_sha256", "prover_sha256"):
            with self.subTest(field=field):
                with self.assertRaisesRegex(vw.StartupError, "C10-1"):
                    fx.worker(**{field: "0" * 64}).startup()
        with open(fx.cli, "a") as file:  # changed after the hash was configured
            file.write("# changed\n")
        with self.assertRaisesRegex(vw.StartupError, "C10-1"):
            fx.worker(cli_sha256=sha256(FAKES / "fake-vericode")).startup()
        self.assertEqual(fx.calls(), [], "nothing may run before the hashes match")

    def test_load_config_accepts_only_the_pinned_d10a_values(self):
        fx = Fixture(self)
        base = dataclass_dict(fx.config(port=8710))
        base.update(vw.PINNED_SHA256, shim=vw.PINNED_SHIM_PATH)
        path = fx.root / "worker.json"

        def load(**changes):
            path.write_text(json.dumps({**base, **changes}))
            os.chmod(path, 0o600)
            return vw.load_config(path)

        self.assertEqual(load().cli_sha256, vw.PINNED_SHA256["cli_sha256"])
        with self.assertRaisesRegex(vw.StartupError, "D9"):
            load(cli_sha256="7e7a9260" + "0" * 56)
        with self.assertRaisesRegex(vw.StartupError, "D9"):
            load(prover_sha256="79b83528" + "0" * 56)
        with self.assertRaisesRegex(vw.StartupError, "pinned"):
            load(prover_sha256="ab" * 32)
        with self.assertRaisesRegex(vw.StartupError, "pinned"):
            load(shim_sha256="ab" * 32)
        with self.assertRaisesRegex(vw.StartupError, "shim"):
            load(shim=str(fx.shim))
        with self.assertRaisesRegex(vw.StartupError, "deployer"):
            load(buyer_pubkey=vw.DEPLOYER_PUBKEY)
        with self.assertRaisesRegex(vw.StartupError, "port"):
            load(port=80)
        with self.assertRaisesRegex(vw.StartupError, "unknown"):
            load(extra=1)
        path.write_text(json.dumps(base))
        os.chmod(path, 0o644)
        with self.assertRaisesRegex(vw.StartupError, "0600"):
            vw.load_config(path)
        repo_config = Path(__file__).resolve()  # a file inside the Git work tree
        with self.assertRaises(vw.StartupError):
            vw.load_config(repo_config)

    def test_a_shim_with_another_hash_or_mode_is_refused_at_startup(self):
        fx = Fixture(self)
        os.chmod(fx.shim, 0o644)
        with self.assertRaisesRegex(vw.StartupError, "0755"):
            fx.worker().startup()
        os.chmod(fx.shim, 0o755)
        with self.assertRaisesRegex(vw.StartupError, "C10-2"):
            fx.worker(shim_sha256="ab" * 32).startup()

    def test_the_prover_must_run_the_configured_shim(self):
        fx = Fixture(self, embed_shim=False)
        with self.assertRaisesRegex(vw.StartupError, "shim"):
            fx.worker().startup()

    def test_key_probes_refuse_the_deployer_and_any_other_key(self):
        for stdout, exit_code, needle in (
            ([f"buyer={vw.DEPLOYER_PUBKEY}"], 1, "deployer"),
            ([f"buyer={fake_pubkey('someone else')}"], 1, "probe failed"),
            ([f"buyer={BUYER}"], 0, "probe failed"),
            ([f"buyer={BUYER}", "something else"], 1, "probe failed"),
            ([], 1, "probe failed"),
        ):
            with self.subTest(stdout=stdout, exit=exit_code):
                fx = Fixture(self, cli=cli_scenario(**{"probe-buyer": [
                    {"stdout": stdout, "stderr": vw.PROBE_ERRORS["buyer"] + "\n", "exit": exit_code}]}))
                with self.assertRaisesRegex(vw.StartupError, needle):
                    fx.worker().startup()

    def test_both_preflight_checks_need_their_exact_lines(self):
        wrong_verifier = [line.replace("34ae6e5c", "34ae6e5d") for line in CHECK_OUTPUT]
        for cli, prover in (
            ([{"stdout": CHECK_OUTPUT[:-1]}], None),
            ([{"stdout": wrong_verifier}], None),
            ([{"stdout": [line for line in CHECK_OUTPUT if not line.startswith("check.verifier")]}], None),
            ([{"stdout": CHECK_OUTPUT, "exit": 1}], None),
            (None, [{"stdout": PROVER_CHECK_OUTPUT[:-1]}]),
            (None, [{"stdout": PROVER_CHECK_OUTPUT, "exit": 1}]),
        ):
            with self.subTest(cli=bool(cli), prover=bool(prover)):
                fx = Fixture(self)
                if cli:
                    fx.set_cli(cli_scenario(check=cli))
                if prover:
                    fx.set_prover(prover_scenario(str(fx.shim_dir), check=prover))
                with self.assertRaisesRegex(vw.StartupError, "check failed"):
                    fx.worker().startup()


def dataclass_dict(config):
    return {name: getattr(config, name) for name in vw.FIELDS}


# --- HTTP ------------------------------------------------------------------------------


class HttpTests(unittest.TestCase):
    def setUp(self):
        self.fx = Fixture(self)
        self.fx.start()
        self.startup_calls = len(self.fx.calls())

    def test_host_must_be_the_loopback_address_and_port(self):
        fx = self.fx
        for hosts, expected in (
            ([f"localhost:{fx.port}"], 421),
            (["127.0.0.1:1"], 421),
            (["evil.example"], 421),
            ([f"127.0.0.1:{fx.port}.evil.example"], 421),
            ([], 400),
            ([f"127.0.0.1:{fx.port}", f"127.0.0.1:{fx.port}"], 400),
        ):
            with self.subTest(hosts=hosts):
                self.assertEqual(fx.request("GET", "/api/health", hosts=hosts)[0], expected)
                self.assertEqual(fx.request("POST", "/api/check", {}, hosts=hosts)[0], expected)
        self.assertEqual(len(fx.calls()), self.startup_calls)

    def test_the_token_is_required_on_every_api_route(self):
        fx = self.fx
        for method, path, body in (("GET", "/api/health", None), ("GET", f"/api/jobs/{JOB}", None),
                                   ("POST", "/api/check", {}), ("POST", "/api/jobs", {"deadline_offset": 9000})):
            with self.subTest(path=path):
                self.assertEqual(fx.request(method, path, body, token=None)[0], 401)
                self.assertEqual(fx.request(method, path, body, token="x" * 43)[0], 403)
                self.assertEqual(fx.request(method, path, body, token=fx.w.token + "x")[0], 403)
        self.assertEqual(fx.request("GET", "/api/health")[0], 200)
        self.assertEqual(len(fx.calls()), self.startup_calls)

    def test_no_response_carries_cors_headers(self):
        fx = self.fx
        origin = [("Origin", "https://evil.example"), ("Access-Control-Request-Method", "POST"),
                  ("Access-Control-Request-Headers", "x-vericode-token")]
        responses = [
            fx.request("GET", "/api/health", headers=origin),
            fx.request("GET", "/api/health", token=None, headers=origin),
            fx.request("OPTIONS", "/api/jobs", headers=origin, token=None),
            fx.request("OPTIONS", "/api/jobs", headers=origin),
            fx.request("POST", "/api/jobs", {"deadline_offset": 1}, headers=origin),
            fx.request("GET", "/api/nothing", headers=origin),
            fx.request("GET", "/", headers=origin),
            fx.request("GET", "/api/health", hosts=["evil.example"], headers=origin),
        ]
        self.assertEqual(responses[2][0], 405)
        for status, headers, _ in responses:
            self.assertFalse([name for name in headers if name.startswith("access-control-")], (status, headers))
            self.assertEqual(headers.get("x-content-type-options"), "nosniff")

    def test_invalid_parameters_get_400_and_run_nothing(self):
        fx = self.fx
        bad = [
            ("/api/jobs", {"deadline_offset": 1559}),
            ("/api/jobs", {"deadline_offset": 9001}),
            ("/api/jobs", {"deadline_offset": "9000"}),
            ("/api/jobs", {"deadline_offset": 9000.0}),
            ("/api/jobs", {"deadline_offset": True}),
            ("/api/jobs", {}),
            ("/api/jobs", {"deadline_offset": 9000, "amount": 5}),
            (f"/api/jobs/{JOB.upper()}/show", {}),
            (f"/api/jobs/{JOB[:-1]}/show", {}),
            (f"/api/jobs/{'11' * 32}/show", {}),
            (f"/api/jobs/{'zz' * 32}/show", {}),
            (f"/api/jobs/{JOB}/show", {"x": 1}),
            (f"/api/jobs/{JOB}/prove", {"input": -1, "claimed_output": 42}),
            (f"/api/jobs/{JOB}/prove", {"input": 2**32, "claimed_output": 42}),
            (f"/api/jobs/{JOB}/prove", {"input": 21, "claimed_output": 2**32}),
            (f"/api/jobs/{JOB}/prove", {"input": 21}),
            (f"/api/jobs/{JOB}/prove", {"input": "21", "claimed_output": 42}),
            (f"/api/jobs/{JOB}/prove", {"input": 21.0, "claimed_output": 42}),
            (f"/api/jobs/{JOB}/prove", {"input": False, "claimed_output": 42}),
            (f"/api/jobs/{JOB}/negative/escrow-6000", {}),
            (f"/api/jobs/{JOB}/negative/verifier-6014", {}),
            (f"/api/jobs/{JOB}/negative/escrow:6021", {}),
            ("/api/check", {"x": 1}),
        ]
        for path, body in bad:
            with self.subTest(path=path, body=body):
                status, _, payload = fx.post(path, body)
                self.assertEqual(status, 400, payload)
        for kwargs, expected in (
            ({"raw": b"not json"}, 400),
            ({"raw": b"[1, 2]"}, 400),
            ({"raw": b'{"deadline_offset": 9000}', "content_type": "text/plain"}, 415),
            ({"raw": b"{" + b" " * 5000 + b"}"}, 413),
        ):
            with self.subTest(kwargs=kwargs):
                self.assertEqual(fx.request("POST", "/api/jobs", **kwargs)[0], expected)
        self.assertEqual(fx.request("GET", "/api/health?x=1")[0], 400)
        self.assertEqual(fx.request("GET", "/api/ops/xyz")[0], 400)
        self.assertEqual(fx.request("GET", f"/api/jobs/{JOB.upper()}")[0], 400)
        self.assertEqual(len(fx.calls()), self.startup_calls, "an invalid request ran something")

    def test_fixed_routes_and_methods(self):
        fx = self.fx
        self.assertEqual(fx.request("GET", "/api/nothing")[0], 404)
        self.assertEqual(fx.request("GET", "/")[0], 404)
        self.assertEqual(fx.request("GET", "/api/../etc/passwd")[0], 404)
        self.assertEqual(fx.request("DELETE", "/api/health")[0], 405)
        self.assertEqual(fx.request("GET", "/api/check")[0], 405)
        self.assertEqual(fx.request("POST", "/api/health", {})[0], 405)
        self.assertEqual(fx.request("GET", f"/api/jobs/{JOB}")[0], 404)
        self.assertEqual(fx.request("GET", f"/api/ops/{'ab' * 8}")[0], 404)

    def test_a_second_operation_gets_409(self):
        fx = self.fx
        fx.set_cli(cli_scenario(**{"job create": [dict(create_answer(JOB), sleep=1.5)]}))
        status, _, started = fx.post("/api/jobs", {"deadline_offset": 9000})
        self.assertEqual(status, 202)
        for path, body in (("/api/check", {}), ("/api/jobs", {"deadline_offset": 9000}),
                           (f"/api/jobs/{EXTERNAL}/show", {}), (f"/api/jobs/{EXTERNAL}/refund-timeout", {})):
            with self.subTest(path=path):
                status, _, payload = fx.post(path, body)
                self.assertEqual(status, 409)
                self.assertEqual(payload["error"], "busy")
                self.assertEqual(payload["running_op"]["op_id"], started["op_id"])
        self.assertEqual(fx.wait(started["op_id"])["status"], "ok")
        self.assertEqual(fx.post("/api/check")[0], 200)
        ran = [(call["bin"], [arg for arg in call["argv"] if not arg.startswith("/")][:3])
               for call in fx.calls()[self.startup_calls:]]
        self.assertEqual(ran, [("vericode", ["--log", "job", "create"]), ("vericode", ["job", "show", "--job-id"]),
                               ("vericode", ["check"]), ("vericode-prover", ["check"])],
                         "only the create, its show and the later check ran")


# --- flows ---------------------------------------------------------------------------------


class FlowTests(unittest.TestCase):
    def flow_scenario(self):
        shows = [
            show(slot=20_000, deadline=10_000),               # external Job, before the refund
            show(status="RefundedOnTimeout", vault=0),       # after the refund
            show(),                                          # after create
            show(slot=1_000, deadline=10_000),               # right before escrow:6021 (C10-8)
            show(),                                          # after escrow:6021
            show(),                                          # after verifier:6003
            RELEASED,                                        # after settle
            RELEASED,                                        # after escrow:6007
        ]
        return cli_scenario(**{"job show": shows})

    def test_full_flow_fixed_argv_and_only_the_built_environment(self):
        fx = Fixture(self, cli=self.flow_scenario())
        with mock.patch.dict(os.environ, HOSTILE):
            fx.start()
            status, _, shown = fx.post(f"/api/jobs/{EXTERNAL}/show")
            self.assertEqual((status, shown["job"]["state"], shown["job"]["origin"]), (200, "Funded", "external"))
            refund = fx.run(f"/api/jobs/{EXTERNAL}/refund-timeout")
            create = fx.create()
            n6021 = fx.run(f"/api/jobs/{JOB}/negative/escrow-6021")
            prove = fx.prove()
            n6003 = fx.run(f"/api/jobs/{JOB}/negative/verifier-6003")
            settle = fx.run(f"/api/jobs/{JOB}/settle")
            n6007 = fx.run(f"/api/jobs/{JOB}/negative/escrow-6007")
        for op in (refund, create, n6021, prove, n6003, settle, n6007):
            self.assertEqual(op["status"], "ok", op)
        self.assertEqual(fx.job(EXTERNAL)["state"], "RefundedOnTimeout")
        view = fx.job()
        self.assertEqual((view["state"], view["state_scope"], view["chain"]["status"]), ("Released", "chain", "Released"))
        self.assertEqual(view["receipt"]["status"], "usable")
        self.assertNotIn("dir", view["receipt"])
        for op, program, code in ((n6021, "escrow", 6021), (n6003, "verifier", 6003), (n6007, "escrow", 6007)):
            self.assertEqual((op["rejection"]["program"], op["rejection"]["code"]), (program, code))
            self.assertEqual(op["rejection"]["program_id"], vw.PROGRAM_IDS[program])
            self.assertTrue(op["rejection"]["explorer"].startswith("https://explorer.solana.com/tx/"))
        self.assertTrue(settle["transaction"]["verifier_invoked"])
        self.assertEqual(settle["transaction"]["verifier_units"], 99541)
        self.assertEqual(n6021["c10_8"]["margin"], 9_000)
        self.assertTrue(prove["receipt"]["docker_run"].endswith(vw.GROTH16_IMAGE))

        data, home, tmp = fx.data, str(fx.root / "home"), str(fx.root / "tmp")
        bk, ek = str(fx.keys / "buyer.json"), str(fx.keys / "executor.json")
        rd = str(data / "receipts" / JOB / "1")
        cli, prover = str(fx.cli), str(fx.prover)

        def log(op):
            return ["--log", str(data / "ops" / op["op_id"] / "cli-tx.jsonl")]

        expected = [
            [cli, "job", "create", "--buyer-keypair", bk, "--executor", "1"],
            [cli, "job", "deliver", "--executor-keypair", ek, "--job-id", "00"],
            [cli, "check"],
            [prover, "check"],
            [cli, "job", "show", "--job-id", EXTERNAL],
            [cli, *log(refund), "job", "refund-timeout", "--job-id", EXTERNAL, "--payer-keypair", bk],
            [cli, "job", "show", "--job-id", EXTERNAL],
            [cli, *log(create), "job", "create", "--buyer-keypair", bk, "--executor", EXECUTOR, "--amount", "1000000",
             "--deadline-offset", "9000"],
            [cli, "job", "show", "--job-id", JOB],
            [cli, "job", "show", "--job-id", JOB],
            [cli, *log(n6021), "job", "refund-timeout", "--job-id", JOB, "--payer-keypair", bk, "--expect-error",
             "escrow:6021"],
            [cli, "job", "show", "--job-id", JOB],
            [prover, "prove", JOB, "21", "42", rd],
            [prover, "compress", rd],
            [cli, *log(n6003), "job", "settle", "--job-id", JOB, "--receipt", rd, "--deliver", "--executor-keypair", ek,
             "--tamper-seal", "--expect-error", "verifier:6003"],
            [cli, "job", "show", "--job-id", JOB],
            [cli, *log(settle), "job", "settle", "--job-id", JOB, "--receipt", rd, "--deliver", "--executor-keypair", ek],
            [cli, "job", "show", "--job-id", JOB],
            [cli, *log(n6007), "job", "settle", "--job-id", JOB, "--receipt", rd, "--payer-keypair", ek,
             "--expect-error", "escrow:6007"],
            [cli, "job", "show", "--job-id", JOB],
        ]
        calls = fx.calls()
        self.assertEqual([[call_path(fx, call)] + call["argv"] for call in calls], expected)
        cli_env = {"HOME": home, "PATH": "/usr/bin:/bin", "TMPDIR": tmp}
        for call in calls:
            with self.subTest(argv=call["argv"][:3]):
                expected_env = {**cli_env, "RISC0_PROVER": "local"} if call["bin"] == "vericode-prover" else cli_env
                self.assertEqual(call["env"], expected_env)
                self.assertEqual(call["cwd"], home)
        self.assertEqual(fx.post(f"/api/jobs/{EXTERNAL}/prove", {"input": 21, "claimed_output": 42})[2]["error"],
                         "not_a_worker_job")

    def test_unexpected_is_never_reported_as_a_rejection(self):
        label = "refund_on_timeout"
        variants = {
            "unexpected": tx(label, "u1", err=None, outcome="UNEXPECTED", exit=1,
                             sim=sim_line(label, vw.ESCROW_ID, 6021),
                             stderr="error: [refund_on_timeout] landed transaction did not match Failure\n"),
            "no simulation line": tx(label, "u2", err=custom(0, 6021)),
            "other program": tx(label, "u3", err=custom(0, 6021), sim=sim_line(label, vw.VERIFIER_ID, 6021)),
            "other code": tx(label, "u4", err=custom(0, 6022), sim=sim_line(label, vw.ESCROW_ID, 6021)),
        }
        for name, answer in variants.items():
            with self.subTest(name):
                after = show(status="RefundedOnTimeout", vault=0) if name == "unexpected" else show()
                fx = Fixture(self, cli=cli_scenario(**{"job show": [show(), show(), after],
                                                       "job refund-timeout escrow:6021": [answer]}))
                fx.start()
                fx.create()
                op = fx.run(f"/api/jobs/{JOB}/negative/escrow-6021")
                self.assertIsNone(op["rejection"])
                self.assertEqual(op["status"], "failed")
                self.assertIn("no rejection", op["error"])
                if name == "unexpected":
                    self.assertEqual(op["outcome"], "UNEXPECTED")
                    self.assertEqual(fx.job()["state"], "RefundedOnTimeout")
                summary = fx.job()["ops"][-1]
                self.assertIsNone(summary["rejection"])

    def test_a_nonzero_exit_after_a_landed_pass_is_reconciled_from_show_and_log(self):
        late = tx("deliver+release", "late", verifier=True, exit=1,
                  stderr="error: RPC answered at slot 1999, before the transaction slot 2000\n")
        fx = Fixture(self, cli=cli_scenario(**{"job show": [show(), RELEASED], "job settle": [late]}))
        fx.start()
        fx.create()
        self.assertEqual(fx.prove()["status"], "ok")
        op = fx.run(f"/api/jobs/{JOB}/settle")
        self.assertEqual((op["status"], op["outcome"], op["cli_exit_code"], op["reconciled"]), ("ok", "PASS", 1, True))
        self.assertIn("before the transaction slot", op["cli_error"])
        self.assertEqual(fx.job()["state"], "Released")

    def test_a_failed_create_is_reconciled_before_any_other_action(self):
        broken = create_answer(JOB, exit=1, log=False, stderr="error: rpc getTransaction: HTTP 503 after 8 attempts\n")
        broken["pause_after"] = {"prefix": "create.job_id=", "seconds": 1.0}
        unreadable = {"stdout": [], "stderr": "error: rpc getAccountInfo: HTTP 429 after 8 attempts\n", "exit": 1}
        fx = Fixture(self, cli=cli_scenario(**{"job create": [broken], "job show": [unreadable, show()]}))
        fx.start()
        status, _, started = fx.post("/api/jobs", {"deadline_offset": 9000})
        self.assertEqual(status, 202)
        deadline = time.time() + 5
        while time.time() < deadline:  # the job_id is recorded while the CLI still runs
            op = fx.request("GET", f"/api/ops/{started['op_id']}")[2]
            if op["job_id"]:
                break
            time.sleep(0.05)
        self.assertEqual((op["job_id"], op["status"]), (JOB, "running"))
        events = [json.loads(line) for line in (fx.data / "worker-ops.jsonl").read_text().splitlines()]
        self.assertIn({"event": "job_id", "op_id": started["op_id"], "job_id": JOB},
                      [{k: event[k] for k in ("event", "op_id", "job_id") if k in event} for event in events])
        op = fx.wait(started["op_id"])
        self.assertEqual((op["status"], op["outcome"], op["reconciled"]), ("failed", "UNKNOWN", False))
        view = fx.job()
        self.assertEqual((view["state"], view["reconcile_pending"]), ("Submitted", True))
        for path, body in (("/api/check", {}), ("/api/jobs", {"deadline_offset": 9000}),
                           (f"/api/jobs/{EXTERNAL}/show", {}), (f"/api/jobs/{EXTERNAL}/refund-timeout", {})):
            with self.subTest(path=path):
                status, _, payload = fx.post(path, body)
                self.assertEqual((status, payload["error"]), (409, "reconcile_pending"))
        status, _, shown = fx.post(f"/api/jobs/{JOB}/show")
        self.assertEqual((status, shown["job"]["state"], shown["job"]["reconcile_pending"]), (200, "Funded", False))
        self.assertEqual(fx.post("/api/check")[0], 200)

    def test_a_create_that_left_no_job_on_chain_is_failed(self):
        refused = create_answer(JOB, exit=1, outcome="SIMULATION_UNEXPECTED_NOT_SENT",
                                stderr="error: [create+fund] simulation did not match Success; nothing was sent\n")
        absent = {"stdout": [], "stderr": f"error: no Job {fake_pubkey('job pda')} for job_id {JOB}\n", "exit": 1}
        fx = Fixture(self, cli=cli_scenario(**{"job create": [refused], "job show": [absent]}))
        fx.start()
        op = fx.run("/api/jobs", {"deadline_offset": 9000})
        self.assertEqual((op["status"], op["outcome"], op["reconciled"]), ("failed", "SIMULATION_UNEXPECTED_NOT_SENT", True))
        view = fx.job()
        self.assertEqual((view["state"], view["chain"]["absent"], view["reconcile_pending"]), ("Failed", True, False))

    def test_compress_requires_the_shim_check_memory_and_exactly_one_docker_run_line(self):
        def compress(**changes):
            answer = {"stdout": None, "shim_log": RUN_LINE + "\n"}
            run = changes.pop("run", RUN_LINE)
            answer.update(changes)
            if answer["stdout"] is None:
                answer["stdout"] = compress_output("{shim}", run=run)
            return answer

        tag = RUN_LINE.replace(vw.GROTH16_IMAGE, "risczero/risc0-groth16-prover:v2025-04-03.1")
        variants = {
            "shim mode": ({}, lambda fx: os.chmod(fx.shim, 0o644)),
            "shim content": ({}, lambda fx: fx.shim.write_text("#!/bin/sh\nexec /usr/bin/docker \"$@\"\n")),
            "low memory": ({}, lambda fx: setattr(fx, "mem_kb", vw.MIN_MEM_AVAILABLE_KB - 1)),
            "no docker_run line": (compress(run=None), None),
            "two docker_run lines": (compress(stdout=compress_output("{shim}") + [f"compress.docker_run={RUN_LINE}"]), None),
            "another directory": (compress(run=RUN_LINE.replace("{dir}", "/home/lucas/elsewhere"),
                                           shim_log=RUN_LINE.replace("{dir}", "/home/lucas/elsewhere") + "\n"), None),
            "tag, not digest": (compress(run=tag, shim_log=tag + "\n"), None),
            "network allowed": (compress(run=RUN_LINE.replace(" --network=none", ""),
                                         shim_log=RUN_LINE.replace(" --network=none", "") + "\n"), None),
            "old time": (compress(run=RUN_LINE.replace("{now}", "2026-01-01T00:00:00-03:00"),
                                  shim_log=RUN_LINE.replace("{now}", "2026-01-01T00:00:00-03:00") + "\n"), None),
            "shim log differs": (compress(shim_log=""), None),
            "shim log missing": (compress(shim_log=None), None),
            "other docker_shim": (compress(stdout=compress_output("{shim}", docker_shim="/home/lucas/other")), None),
            "exit 137": (compress(exit=137), None),
        }
        for name, (answer, tamper) in variants.items():
            with self.subTest(name):
                fx = Fixture(self)
                if answer:
                    answer = json.loads(json.dumps(answer).replace("{shim}", str(fx.shim_dir)))
                    fx.set_prover(prover_scenario(str(fx.shim_dir), compress=[answer]))
                fx.start()
                fx.create()
                if tamper:
                    tamper(fx)
                op = fx.prove()
                self.assertEqual(op["status"], "failed", op)
                compress_ran = any(call["argv"][0] == "compress" for call in fx.calls("vericode-prover"))
                self.assertEqual(compress_ran, tamper is None)
                view = fx.job()
                self.assertEqual((view["state"], view["receipt"]["status"]), ("Failed", "failed"))
                for path in (f"/api/jobs/{JOB}/settle", f"/api/jobs/{JOB}/negative/verifier-6003",
                             f"/api/jobs/{JOB}/negative/escrow-6007"):
                    status, _, payload = fx.post(path)
                    self.assertEqual((status, payload["error"]), (409, "no_usable_receipt"))
                self.assertFalse([call for call in fx.calls("vericode") if "settle" in call["argv"]])

    def test_proving_and_submitted_are_visible_while_they_happen(self):
        fx = Fixture(self, cli=cli_scenario(**{
            "job show": [show(), RELEASED],
            "job settle": [tx("deliver+release", "settle", verifier=True, pause=1.0)]}))
        fx.set_prover(prover_scenario(str(fx.shim_dir), prove=[{"stdout": PROVE_OUTPUT, "sleep": 1.0}]))
        fx.start()
        fx.create()
        self.assertEqual(fx.job()["state"], "Funded")
        started = fx.post(f"/api/jobs/{JOB}/prove", {"input": 21, "claimed_output": 42})[2]
        time.sleep(0.5)
        view = fx.job()
        self.assertEqual((view["state"], view["state_scope"], view["running_op"]["kind"]), ("Proving", "local", "prove"))
        self.assertEqual(fx.wait(started["op_id"])["status"], "ok")
        started = fx.post(f"/api/jobs/{JOB}/settle")[2]
        deadline = time.time() + 5
        while time.time() < deadline and fx.job()["state"] != "Submitted":
            time.sleep(0.05)
        self.assertEqual(fx.job()["state"], "Submitted")
        self.assertEqual(fx.wait(started["op_id"])["status"], "ok")
        self.assertEqual(fx.job()["state"], "Released")

    def test_c10_8_needs_300_slots_before_the_deadline(self):
        for margin, sent in ((299, False), (300, True), (-5, False)):
            with self.subTest(margin=margin):
                fx = Fixture(self, cli=cli_scenario(**{"job show": [show(), show(slot=10_000 - margin), show()]}))
                fx.start()
                fx.create()
                op = fx.run(f"/api/jobs/{JOB}/negative/escrow-6021")
                ran = any("--expect-error" in call["argv"] for call in fx.calls("vericode"))
                self.assertEqual(ran, sent)
                self.assertEqual(op["status"], "ok" if sent else "failed")
                if not sent:
                    self.assertIn("C10-8", op["error"])

    def test_no_key_path_or_key_bytes_leave_the_worker(self):
        fx = Fixture(self)
        scenario = cli_scenario(**{"job show": [show(), show(slot=1_000), show(), RELEASED]})
        for key, answers in scenario.items():
            if key.startswith("probe"):
                continue
            for answer in answers:  # a leaky CLI: echoes its argv, key paths included
                answer["stdout"] = list(answer.get("stdout", [])) + ["argv={argv}"]
                answer["stderr"] = answer.get("stderr", "") + "{argv}\n"
        scenario["job settle"][0]["stdout"].append(fx.key_bytes)  # and key-shaped bytes
        fx.set_cli(scenario)
        fx.start()
        fx.create()
        fx.run(f"/api/jobs/{JOB}/negative/escrow-6021")
        fx.prove()
        fx.run(f"/api/jobs/{JOB}/settle")
        fx.request("GET", "/api/health")
        fx.request("GET", "/api/jobs")
        for op_id in fx.w.ops:
            fx.request("GET", f"/api/ops/{op_id}")
        fx.request("GET", f"/api/jobs/{JOB}")
        texts = [payload for _, payload in fx.responses]
        texts += [path.read_text() for path in fx.data.rglob("*") if path.is_file()]
        self.assertTrue(any("<buyer-keypair>" in text for text in texts), "the leaky fake did echo the key paths")
        for needle in ("/keys/", "buyer.json", "executor.json", str(fx.keys), fx.key_bytes, fx.w.token):
            self.assertFalse([text for text in texts if needle in text], needle)
        self.assertEqual(fx.request("GET", "/api/health")[2]["incidents"][0]["what"],
                         "64-number array in subprocess output (redacted)")

    def test_escrow_6014_uses_and_names_the_receipt_of_another_worker_job(self):
        fx = Fixture(self, cli=cli_scenario(**{"job create": [create_answer(JOB), create_answer(OTHER)]}))
        fx.start()
        fx.create(JOB)
        fx.create(OTHER)
        self.assertEqual(fx.post(f"/api/jobs/{JOB}/negative/escrow-6014", {"receipt_job_id": OTHER})[2]["error"],
                         "no_usable_receipt")
        fx.prove(OTHER)
        self.assertEqual(fx.post(f"/api/jobs/{JOB}/negative/escrow-6014", {"receipt_job_id": JOB})[0], 400)
        self.assertEqual(fx.post(f"/api/jobs/{JOB}/negative/escrow-6014", {"receipt_job_id": EXTERNAL})[0], 404)
        self.assertEqual(fx.post(f"/api/jobs/{JOB}/negative/escrow-6014", {})[0], 400)
        op = fx.run(f"/api/jobs/{JOB}/negative/escrow-6014", {"receipt_job_id": OTHER})
        self.assertEqual((op["status"], op["receipt_job_id"], op["rejection"]["code"]), ("ok", OTHER, 6014))
        settle_call = [call for call in fx.calls("vericode") if "--expect-error" in call["argv"]][-1]
        receipt = settle_call["argv"][settle_call["argv"].index("--receipt") + 1]
        self.assertEqual(receipt, str(fx.data / "receipts" / OTHER / "1"))

    def test_a_restart_after_an_interrupted_write_needs_reconciliation(self):
        fx = Fixture(self)
        fx.start()
        fx.create()
        fx.stop()
        op_id = "0123456789abcdef"
        (fx.data / "ops" / op_id).mkdir(mode=0o700)
        vw.write_json(fx.data / "ops" / op_id / "op.json", {
            "op_id": op_id, "kind": "settle", "job_id": JOB, "status": "running", "started_at": vw.now_iso(),
            "steps": [], "signature": None})
        fx.set_cli(cli_scenario())
        fx.start()
        self.assertEqual(fx.request("GET", f"/api/ops/{op_id}")[2]["status"], "interrupted")
        view = fx.job()
        self.assertEqual((view["state"], view["reconcile_pending"]), ("Submitted", True))
        self.assertEqual(fx.post("/api/check")[2]["error"], "reconcile_pending")
        self.assertEqual(fx.post(f"/api/jobs/{JOB}/show")[0], 200)
        self.assertEqual(fx.job()["state"], "Funded")
        self.assertEqual(fx.post("/api/check")[0], 200)


def call_path(fx, call):
    return str(fx.bin / call["bin"])


if __name__ == "__main__":
    unittest.main()
