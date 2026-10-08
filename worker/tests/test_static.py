"""Offline tests of the Hive interface layer of the worker (gate D11-D12): static
routes and headers, verified copies, tokens, the read-only views (journal, anatomy,
balances, v1 facts) and scans of the served text. Same rules as test_worker.py: no
network, no real key, no Docker. Run from the repository root:

    TMPDIR=<dir> python3 -B -m unittest discover -s worker/tests -v
"""

import hashlib
import http.client
import json
import re
import sys
import unittest
from pathlib import Path

sys.dont_write_bytecode = True
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import vericode_worker as vw  # noqa: E402
from test_worker import (  # noqa: E402
    BUYER, CHECK_OUTPUT, EXECUTOR, JOB, PROVE_OUTPUT, RUN_LINE, Fixture, cli_scenario, compress_output, prover_scenario, show)

REPO = Path(__file__).resolve().parents[2]
STATIC = REPO / "worker" / "static"
UI_TOOLS = REPO / "worker" / "ui-tools"
FIXTURES = REPO / "anchor" / "tests-local" / "fixtures" / "groth16"
TX_FIXTURES = Path(__file__).resolve().parent / "fixtures" / "d10-tx"
SPEC = "af642b561ac73a0f78ae767123f95a0c6ece6ce89f2a5e5e20cb86bec49fb778"
HARNESS = "01124025c6ad84bb8490f216e95ff241d862dc2faf316d0e28bcdb55b0996b50"
PAGE_CSP = ("default-src 'none'; script-src 'self'; style-src 'self'; font-src 'self'; img-src 'self'; "
            "connect-src 'self'; base-uri 'none'; form-action 'none'; frame-ancestors 'none'")


def sha256_bytes(data):
    return hashlib.sha256(data).hexdigest()


def served_text_files():
    return [path for path in STATIC.rglob("*") if path.suffix in (".html", ".js", ".css") and path.parent.name != "fonts"]


# --- static routes ---------------------------------------------------------------------


class StaticRouteTests(unittest.TestCase):
    def setUp(self):
        self.fx = Fixture(self)
        self.fx.start()

    def test_every_route_of_the_table_serves_its_file_with_the_page_headers(self):
        for route, name in vw.STATIC_ROUTES.items():
            with self.subTest(route=route):
                status, headers, body = self.raw("GET", route)
                self.assertEqual(status, 200)
                self.assertEqual(body, (STATIC / name).read_bytes())
                self.assertEqual(headers["content-type"], vw.CONTENT_TYPES[Path(name).suffix])
                self.assertEqual(headers["content-security-policy"], PAGE_CSP)
                self.assertEqual(headers["x-content-type-options"], "nosniff")
                self.assertEqual(headers["referrer-policy"], "no-referrer")
                self.assertEqual(headers["cache-control"], "no-store")
                self.assertEqual(headers["x-frame-options"], "DENY")
                self.assertFalse([key for key in headers if key.startswith("access-control-")])

    def test_the_table_is_exactly_the_files_under_worker_static(self):
        on_disk = {str(path.relative_to(STATIC)) for path in STATIC.rglob("*") if path.is_file()}
        self.assertEqual(on_disk, set(vw.STATIC_ROUTES.values()))
        self.assertEqual(len(vw.STATIC_FILES), len(set(vw.STATIC_FILES)))

    def test_names_outside_the_table_are_refused(self):
        for path, expected in (
            ("/ui", 404), ("/ui/index.html", 404), ("/ui/../vericode_worker.py", 404),
            ("/ui/%2e%2e/vericode_worker.py", 404), ("/ui/%2E%2E/vericode_worker.py", 404), ("/ui//css/app.css", 404),
("//ui/", 400), ("//api/health", 400), ("/ui/css/", 404), ("/ui/js", 404), ("/ui/icons/lock.SVG", 404), ("/ui/nothing.js", 404),
            ("/ui/css/../js/main.js", 404), ("/static/css/app.css", 404), ("/", 404), ("/worker/static/index.html", 404),
            ("/ui/?x=1", 400), ("/ui/js/main.js?v=1", 400),
        ):
            with self.subTest(path=path):
                self.assertEqual(self.raw("GET", path)[0], expected)

    def test_only_get_and_the_host_is_checked(self):
        self.assertEqual(self.fx.request("POST", "/ui/", {})[0], 405)
        self.assertEqual(self.fx.request("POST", "/ui/js/main.js", {}, token=None)[0], 405)
        self.assertEqual(self.raw("GET", "/ui/", hosts=["localhost:%d" % self.fx.port])[0], 421)
        self.assertEqual(self.raw("GET", "/ui/", hosts=["evil.example"])[0], 421)
        self.assertEqual(self.raw("GET", "/ui/", hosts=[])[0], 400)

    def test_static_files_need_no_token_and_the_api_still_does(self):
        self.assertEqual(self.raw("GET", "/ui/")[0], 200)
        for method, path in (("GET", "/api/health"), ("GET", "/api/jobs"), ("POST", "/api/check")):
            with self.subTest(path=path):
                self.assertEqual(self.fx.request(method, path, {} if method == "POST" else None, token=None)[0], 401)

    def test_no_token_and_no_key_path_in_what_is_served(self):
        for route in vw.STATIC_ROUTES:
            _, _, body = self.raw("GET", route)
            text = body.decode("utf-8", "replace")
            with self.subTest(route=route):
                self.assertNotIn(self.fx.w.token, text)
                for needle in ("/keys/", "buyer.json", "executor.json", "deployer.json", "d4/keys", str(self.fx.keys)):
                    self.assertNotIn(needle, text)

    def raw(self, method, path, hosts=None):
        connection = http.client.HTTPConnection("127.0.0.1", self.fx.port, timeout=20)
        connection.putrequest(method, path, skip_host=True, skip_accept_encoding=True)
        for host in ([f"127.0.0.1:{self.fx.port}"] if hosts is None else hosts):
            connection.putheader("Host", host)
        connection.endheaders()
        response = connection.getresponse()
        body = response.read()
        connection.close()
        return response.status, {name.lower(): value for name, value in response.getheaders()}, body


# --- verified copies and tokens ----------------------------------------------------------------


class CopyAndTokenTests(unittest.TestCase):
    def test_each_copied_asset_matches_the_table_and_its_source(self):
        table = json.loads((UI_TOOLS / "static-assets.json").read_text())["files"]
        listed = {entry["dest"] for entry in table}
        copied = {str(path.relative_to(STATIC)) for folder in ("fonts", "brand", "icons")
                  for path in (STATIC / folder).rglob("*") if path.is_file()}
        self.assertEqual(copied, listed, "fonts/, brand/ and icons/ hold exactly the table")
        lucide = UI_TOOLS / "node_modules" / "lucide-static"
        for entry in table:
            with self.subTest(dest=entry["dest"]):
                self.assertRegex(entry["sha256"], r"^[0-9a-f]{64}$")
                self.assertEqual(sha256_bytes((STATIC / entry["dest"]).read_bytes()), entry["sha256"])
                if entry["source"].startswith("brand/"):
                    self.assertEqual(sha256_bytes((REPO / entry["source"]).read_bytes()), entry["sha256"])
                elif lucide.is_dir():  # development only; the pinned hash is checked above anyway
                    self.assertEqual(sha256_bytes((UI_TOOLS / "node_modules" / entry["source"]).read_bytes()), entry["sha256"])

    def test_fonts_match_the_hashes_of_brand_fonts_readme(self):
        readme = (REPO / "brand" / "fonts" / "README.md").read_text()
        rows = re.findall(r"\| `([^`]+)` \|[^|]*\|[^|]*\|[^|]*\| (\d+) \| `([0-9a-f]{64})` \|", readme)
        self.assertEqual(len(rows), 8)
        for name, size, digest in rows:
            data = (STATIC / "fonts" / name).read_bytes()
            self.assertEqual((len(data), sha256_bytes(data)), (int(size), digest), name)

    def test_tokens_css_comes_from_the_current_design_md(self):
        tokens = (STATIC / "css" / "tokens.css").read_text()
        design = (REPO / "DESIGN.md").read_bytes()
        self.assertIn(f"DESIGN.md sha256={sha256_bytes(design)}", tokens.splitlines()[1])
        colors = re.findall(r'^\s{2}([a-z0-9-]+): "#([0-9A-Fa-f]{6})"', design.decode(), re.MULTILINE)
        self.assertEqual(len(colors), 38)
        for name, value in colors:
            self.assertIn(f"--hive-color-{name}: #{value.lower()};", tokens)
        light = tokens.split(":root {\n  color-scheme: light;")[1].split("}")[0]
        dark = tokens.split('[data-theme="dark"] {\n  color-scheme: dark;')[1].split("}")[0]
        for role in ("bg-page", "bg-nav", "bg-panel", "bg-sunken", "bg-selected", "bg-chip", "text-primary",
                     "text-secondary", "text-on-nav", "text-on-accent", "accent", "border-subtle", "border-strong",
                     "focus", "link", "status-success", "status-success-bg", "status-danger", "status-danger-bg",
                     "status-caution", "status-caution-bg", "status-pending", "status-pending-bg"):
            self.assertIn(f"--hive-{role}: var(--hive-color-", light, role)
            self.assertIn(f"--hive-{role}: var(--hive-color-", dark, role)
        self.assertIn("--hive-accent: var(--hive-color-tertiary);", light)
        self.assertIn("--hive-focus: var(--hive-color-focus-ring);", light)
        self.assertIn("--hive-focus: var(--hive-color-dark-focus-ring);", dark)


# --- journal (lacuna 1) ---------------------------------------------------------------------------


def fixture_journal(path):
    return dict(line.split("=", 1) for line in path.read_text().splitlines() if "=" in line and not line.startswith("#"))["journal"]


class JournalTests(unittest.TestCase):
    def test_decoded_fields_match_the_groth16_fixtures(self):
        cases = {
            FIXTURES / "pass.txt": ("11" * 32, "PASS"),
            FIXTURES / "fail.txt": ("11" * 32, "FAIL"),
            FIXTURES / "d4b" / "S.txt": ("fe6d25fe", "PASS"),
            FIXTURES / "d4b" / "A.txt": ("3f0dd1c8", "PASS"),
            FIXTURES / "d4b" / "A-fail.txt": ("3f0dd1c8", "FAIL"),
            FIXTURES / "d4b" / "B.txt": ("5a25ae48", "FAIL"),
        }
        for path, (job_prefix, verdict) in cases.items():
            with self.subTest(path.name):
                text = fixture_journal(path)
                journal = vw.decode_journal(text)
                self.assertEqual(journal["schema_version"], 1)
                self.assertTrue(journal["job_id"].startswith(job_prefix))
                self.assertEqual((journal["spec_hash"], journal["harness_hash"]), (SPEC, HARNESS))
                self.assertEqual(journal["image_id"], vw.ADMITTED_IMAGE_ID)
                self.assertEqual(journal["verdict"], verdict)
                self.assertEqual(journal["artifact_hash"], text[200:264])
                self.assertEqual(journal["hex"], text)
        self.assertEqual(vw.decode_journal(fixture_journal(FIXTURES / "pass.txt"))["artifact_hash"],
                         "d5aa9223d6d2a1ba23bd73ca325b411c75027a739c285a95ff63b963442a224c")

    def test_anything_but_the_frozen_layout_is_refused(self):
        good = fixture_journal(FIXTURES / "pass.txt")
        for name, text in (
            ("164 bytes", good[:-2]), ("166 bytes", good + "00"), ("schema 2", "02" + good[2:]),
            ("verdict 2", good[:-2] + "02"), ("uppercase", good.upper()), ("not hex", "zz" * 165), ("none", None),
        ):
            with self.subTest(name):
                with self.assertRaises(ValueError):
                    vw.decode_journal(text)

    def test_the_receipt_view_carries_the_journal_and_the_local_checks(self):
        journal_hex = "01000000" + JOB + SPEC + HARNESS + "22" * 32 + vw.ADMITTED_IMAGE_ID + "00"
        prove = PROVE_OUTPUT + [f"prove.journal_hex={journal_hex}"]
        fx = Fixture(self)
        fx.set_prover(prover_scenario(str(fx.shim_dir), prove=[{"stdout": prove}],
                                      compress=[{"stdout": compress_output(str(fx.shim_dir)) + [f"compress.journal_hex={journal_hex}"],
                                                 "shim_log": RUN_LINE + "\n"}]))
        fx.start()
        fx.create()
        self.assertEqual(fx.prove()["status"], "ok")
        receipt = fx.job()["receipt"]
        self.assertEqual(receipt["journal"]["job_id"], JOB)
        self.assertEqual(receipt["journal"]["artifact_hash"], "22" * 32)
        self.assertEqual(receipt["journal"]["verdict"], "PASS")
        self.assertEqual(receipt["journal"]["source"], "compress.journal_hex")
        self.assertEqual(receipt["local"]["prove.receipt_type"], "Composite")
        self.assertEqual(receipt["local"]["compress.receipt_type"], "Groth16")
        self.assertEqual(receipt["local"]["compress.selector"], "73c457ba")
        self.assertNotIn("dir", receipt)

    def test_a_journal_that_differs_between_prove_and_compress_is_not_shown(self):
        base = "01000000" + JOB + SPEC + HARNESS + "22" * 32 + vw.ADMITTED_IMAGE_ID
        fx = Fixture(self)
        fx.set_prover(prover_scenario(str(fx.shim_dir), prove=[{"stdout": PROVE_OUTPUT + [f"prove.journal_hex={base}00"]}],
                                      compress=[{"stdout": compress_output(str(fx.shim_dir)) + [f"compress.journal_hex={base}01"],
                                                 "shim_log": RUN_LINE + "\n"}]))
        fx.start()
        fx.create()
        fx.prove()
        receipt = fx.job()["receipt"]
        self.assertIsNone(receipt["journal"])
        self.assertIn("differs", receipt["journal_error"])


# --- anatomy and balances (lacunas 2 and 3) ------------------------------------------------------


def tx_fixture(name):
    return json.loads((TX_FIXTURES / f"{name}.json").read_text())


class AnatomyTests(unittest.TestCase):
    def test_settlement_invocations_from_a_real_log(self):
        case = tx_fixture("settle-ByGF4BFP")
        self.assertTrue(case["entry"]["signature"].startswith("ByGF4BFP"))
        calls = vw.invocations_of(case["entry"]["logs"])
        summary = [(call["program"], call["depth"], call["instruction"], call["units"], call["result"]) for call in calls]
        self.assertEqual(summary, [
            ("escrow", 1, "Deliver", 4947, "success"),
            ("escrow", 1, "Release", 117209, "success"),
            ("verifier", 2, "Verify", 99541, "success"),
            ("token", 2, None, 105, "success"),
        ])

    def test_settlement_balances_from_a_real_log(self):
        case = tx_fixture("settle-ByGF4BFP")
        rows = vw.balances_of(case["entry"], case["chain"])
        by_role = {row["role"]: row for row in rows}
        self.assertEqual(set(by_role), {"vault", "executor"})
        self.assertEqual((by_role["vault"]["before"], by_role["vault"]["after"]), (1_000_000, 0))
        self.assertEqual(by_role["vault"]["token_account"], case["chain"]["vault"]["address"])
        self.assertEqual((by_role["executor"]["before"], by_role["executor"]["after"]), (6_000_000, 7_000_000))
        self.assertEqual(by_role["executor"]["token_account"], case["chain"]["executor_ata"]["address"])
        self.assertEqual(by_role["executor"]["decimals"], 6)

    def test_a_verifier_rejection_names_the_failing_program_and_error(self):
        case = tx_fixture("verifier-6003-4cKK83JB")
        calls = vw.invocations_of(case["entry"]["logs"])
        verifier = [call for call in calls if call["program"] == "verifier"]
        self.assertEqual(len(verifier), 1)
        self.assertEqual((verifier[0]["result"], verifier[0]["anchor_error"]), ("failed", {"name": "PairingError", "code": 6003}))
        self.assertEqual(calls[1]["result"], "failed")  # the escrow instruction that made the CPI fails too
        self.assertIsNone(calls[1]["anchor_error"])
        rows = vw.balances_of(case["entry"], case["chain"])
        self.assertTrue(all(row["before"] == row["after"] for row in rows), "nothing moved")

    def test_a_timeout_refund_from_a_real_log(self):
        case = tx_fixture("refund-timeout-57UYbVX9")
        calls = vw.invocations_of(case["entry"]["logs"])
        self.assertEqual([(call["program"], call["instruction"]) for call in calls], [("escrow", "RefundOnTimeout"), ("token", None)])
        rows = {row["role"]: row for row in vw.balances_of(case["entry"], case["chain"])}
        self.assertEqual(rows["vault"]["after"] - rows["vault"]["before"], -1_000_000)
        self.assertEqual(rows["buyer"]["after"] - rows["buyer"]["before"], 1_000_000)

    def test_logs_that_do_not_match_are_ignored(self):
        self.assertEqual(vw.invocations_of(["Program log: Instruction: Release", "not a log"]), [])
        calls = vw.invocations_of([f"Program {vw.ESCROW_ID} invoke [1]", f"Program {vw.VERIFIER_ID} success"])
        self.assertEqual(calls[0]["result"], None, "a success line of another program does not close the frame")
        self.assertEqual(vw.balances_of({"pre_token_balances": [{"accountIndex": "x"}], "post_token_balances": None}, None), [])

    def test_the_operation_view_has_the_anatomy_of_its_transaction(self):
        fx = Fixture(self, cli=cli_scenario(**{"job show": [show(), show(status="Released", artifact="22" * 32, vault=0)]}))
        fx.start()
        fx.create()
        fx.prove()
        op = fx.run(f"/api/jobs/{JOB}/settle")
        view = fx.request("GET", f"/api/ops/{op['op_id']}")[2]
        anatomy = view["anatomy"]
        self.assertTrue(anatomy["landed"])
        self.assertEqual([call["program"] for call in anatomy["invocations"]], ["escrow", "verifier"])
        create_view = fx.request("GET", f"/api/ops/{fx.job()['ops'][0]['op_id']}")[2]
        self.assertIsNone(create_view.get("create_live"), "the fake create prints no create.vault/job/slot lines")


# --- v1 facts, slot clock, commit (lacuna 4 and 5) -----------------------------------------------------


class FactsTests(unittest.TestCase):
    def test_health_carries_the_v1_terms_programs_and_pubkeys(self):
        terms = f"check.terms_v1 spec_hash={SPEC} harness_hash={HARNESS} image_id={vw.ADMITTED_IMAGE_ID}"
        output = [line if not line.startswith("check.terms_v1") else terms for line in CHECK_OUTPUT]
        fx = Fixture(self, cli=cli_scenario(check=[{"stdout": output}]))
        fx.start()
        health = fx.request("GET", "/api/health")[2]
        v1 = health["v1"]
        self.assertEqual(v1["terms"], {"spec_hash": SPEC, "harness_hash": HARNESS, "image_id": vw.ADMITTED_IMAGE_ID})
        self.assertEqual(v1["escrow"]["program_id"], vw.ESCROW_ID)
        self.assertEqual(v1["escrow"]["upgrade_authority"], "none")
        self.assertEqual(v1["escrow"]["program_data_sha256"][:8], "cdf6967f")
        self.assertEqual((v1["verifier"]["program_id"], v1["verifier"]["sha256"][:8]), (vw.VERIFIER_ID, "34ae6e5c"))
        self.assertEqual(v1["verifier"]["release"], "risc0-solana v3.0.0")
        self.assertEqual((v1["mint"]["decimals"], v1["mint"]["freeze_authority"]), (6, "none"))
        self.assertEqual((v1["buyer"], v1["executor"], v1["amount"]), (BUYER, EXECUTOR, 1_000_000))
        self.assertEqual(v1["deadline_offset"], {"min": 1560, "max": 9000})
        self.assertEqual(v1["guest"]["selector"], "73c457ba")
        self.assertEqual(health["ui"], "/ui/")
        self.assertEqual(health["slot_clock"], {"latest": None, "reference": None, "seconds_per_slot": None})
        fx.create()
        clock = fx.request("GET", "/api/health")[2]["slot_clock"]
        self.assertEqual(clock["latest"]["slot"], 1_000)
        self.assertIsNone(clock["seconds_per_slot"], "under 10 minutes apart: no estimate")

    def test_the_slot_clock_measures_seconds_per_slot_between_reads(self):
        fx = Fixture(self)
        worker = fx.worker()
        worker.jobs = {
            "a" * 64: {"chain": {"slot": 1_000, "read_at": "2026-10-07T10:00:00-03:00"}},
            "b" * 64: {"chain": {"slot": 6_000, "read_at": "2026-10-07T10:20:00-03:00"}},
        }
        clock = worker.slot_clock()
        self.assertEqual(clock["latest"]["job_id"], "b" * 64)
        self.assertEqual(clock["reference"]["slot"], 1_000)
        self.assertEqual(clock["seconds_per_slot"], 0.24)

    def test_the_app_commit_is_read_from_git_files(self):
        commit = vw.read_app_commit(REPO)
        self.assertRegex(commit or "", r"^[0-9a-f]{40}$")
        self.assertIsNone(vw.read_app_commit(REPO / "worker"))


# --- scans of the served interface ---------------------------------------------------------------


README_SECTIONS = {"PT": "## Frases permitidas (congeladas no D9)\n", "EN": "## Frozen phrases (English, ratified R-UI)\n"}


def readme_section(lang):
    """One section of README.md, from its heading to the next `## ` heading."""
    readme = (REPO / "README.md").read_text()
    heading = README_SECTIONS[lang]
    if readme.count(heading) != 1:
        raise AssertionError(f"README.md must have exactly one {heading.strip()!r}")
    body = readme.split(heading, 1)[1]
    return body.split("\n## ", 1)[0]


def frozen_phrases(lang):
    """The 8 numbered phrases of one README section (PT or EN), never mixed."""
    phrases = {}
    for number, text in re.findall(r'^(\d)\. \*\*[^*]+\*\* "([^"]+)"', readme_section(lang), re.MULTILINE):
        if int(number) in phrases:
            raise AssertionError(f"phrase {number} twice in the {lang} section")
        phrases[int(number)] = text
    return phrases


def dictionary(name):
    """One dictionary of i18n.js (`const PT = {` or `const EN = {`): one entry per line."""
    text = (STATIC / "js" / "i18n.js").read_text()
    head = f"const {name} = {{\n"
    if text.count(head) != 1:
        raise AssertionError(f"i18n.js must define {name} once")
    body = text.split(head, 1)[1].split("\n};\n", 1)[0]
    entries = {}
    for line in body.splitlines():
        if not line.strip() or line.strip().startswith("//"):
            continue
        match = re.fullmatch(r'  "([^"]+)": "((?:[^"\\]|\\.)*)",', line)
        if not match:
            raise AssertionError(f"{name}: unparsed line {line!r}")
        if match.group(1) in entries:
            raise AssertionError(f"{name}: duplicate key {match.group(1)}")
        entries[match.group(1)] = match.group(2).replace("\\'", "'")
    return entries


# DESIGN.md ("Labels and vocabulary") keys that the code names differently; verify.fallback is
# not used in this MVP (HIVE_MVP_UI_ADAPTATION.md §3.4).
DESIGN_KEY_MAP = {"job.new": "nav.new", "job.fund": "new.fund", "job.funded": "new.funded", "action.refund": "ops.refund",
                  "refund.disabled": "ops.refundDisabled", "env.devnet": "top.devnet", "env.testToken": "top.testToken"}


class InterfaceScanTests(unittest.TestCase):
    def setUp(self):
        self.js = {path: path.read_text() for path in STATIC.rglob("*.js")}
        self.html = (STATIC / "index.html").read_text()
        self.css = (STATIC / "css" / "app.css").read_text()
        self.i18n = (STATIC / "js" / "i18n.js").read_text()

    def test_no_html_parsing_eval_or_browser_storage(self):
        forbidden = (r"\binnerHTML\b", r"\bouterHTML\b", r"\binsertAdjacentHTML\b", r"\beval\s*\(", r"\bnew\s+Function\b",
                     r"document\.write", r"\blocalStorage\b", r"\bsessionStorage\b", r"document\.cookie", r"\.style\b",
                     r"style=", r"setAttribute\(\s*[\"']on")
        for path, text in [*self.js.items(), (STATIC / "index.html", self.html)]:
            for pattern in forbidden:
                with self.subTest(file=path.name, pattern=pattern):
                    self.assertIsNone(re.search(pattern, text))

    def test_the_token_never_goes_to_a_url_or_storage(self):
        api = (STATIC / "js" / "api.js").read_text()
        self.assertIn('"X-VeriCode-Token": token', api)
        self.assertEqual(sum(text.count("X-VeriCode-Token") for text in self.js.values()), 1)
        for text in self.js.values():
            self.assertIsNone(re.search(r"[?&]token=|location\.(search|hash)\s*=.*token", text))

    def test_component_css_uses_only_tokens(self):
        lines = [line for line in self.css.splitlines() if not line.lstrip().startswith("@media")]
        body = re.sub(r"/\*.*?\*/", "", "\n".join(lines), flags=re.S)
        self.assertIsNone(re.search(r"#[0-9a-fA-F]{3,8}\b", body), "hex color")
        self.assertIsNone(re.search(r"\b(rgb|rgba|hsl|hsla)\(", body), "color function")
        self.assertIsNone(re.search(r"(?<![\w-])\d+(\.\d+)?(px|rem|em|ms|s|pt)\b", body), "literal measure or duration")
        self.assertIsNone(re.search(r"font-family|Manrope|Plex", body), "literal font")
        self.assertEqual(re.findall(r"@media[^{]*", self.css), ["@media (prefers-reduced-motion: reduce) ",
                                                              "@media (max-width: 1023px) "])

    def test_no_glyph_the_fonts_lack_and_no_middle_dot(self):
        for path, text in [*self.js.items(), (STATIC / "index.html", self.html), (STATIC / "css" / "app.css", self.css)]:
            for char in ("≈", "→", "·"):
                with self.subTest(file=path.name, char=char):
                    self.assertNotIn(char, text)

    def test_no_forbidden_words(self):
        words = ("verifier router", "fallback", "trustless", "auditad", "audited", "privad", "máquina nova",
                 "dinheiro real", "zk on-chain", "seguro", "confiável", "garantid", "certificad", "aprovad", "score",
                 "reputação", "qualquer repositório", "código está correto", "mascote")
        for path, text in [*self.js.items(), (STATIC / "index.html", self.html)]:
            lowered = text.lower()
            for word in words:
                with self.subTest(file=path.name, word=word):
                    self.assertNotIn(word, lowered)

    def test_mainnet_appears_only_in_the_frozen_phrase_5(self):
        hits = [(path.name, line) for path, text in self.js.items() for line in text.splitlines() if "mainnet" in line.lower()]
        self.assertEqual(len(hits), 2, hits)  # claim.f5 in PT and in EN
        for _, line in hits:
            self.assertIn('"claim.f5"', line)

    def test_portuguese_claims_are_the_frozen_phrases_with_the_brand_change_only(self):
        frozen = frozen_phrases("PT")
        self.assertEqual(sorted(frozen), [1, 2, 3, 4, 5, 6, 7, 8])
        claims = {key[len("claim.f"):]: text for key, text in dictionary("PT").items() if key.startswith("claim.")}
        self.assertEqual(sorted(claims), ["1", "2", "3", "4", "5", "8"])
        for number, text in claims.items():
            expected = frozen[int(number)]
            if number == "1":
                self.assertIn("A receipt Groth16 do VeriCode", expected)
                expected = expected.replace("do VeriCode", "da Hive")
            with self.subTest(claim=number):
                self.assertEqual(text, expected)
        self.assertNotIn("VeriCode", "".join(claims.values()))

    def test_english_claims_are_the_ratified_phrases(self):
        frozen = frozen_phrases("EN")
        self.assertEqual(sorted(frozen), [1, 2, 3, 4, 5, 6, 7, 8])
        self.assertTrue(frozen[1].startswith("Hive's Groth16 receipt is verified on devnet"))
        claims = {key[len("claim.f"):]: text for key, text in dictionary("EN").items() if key.startswith("claim.")}
        self.assertEqual(sorted(claims), ["1", "2", "3", "4", "5", "8"])
        for number, text in claims.items():
            with self.subTest(claim=number):
                self.assertEqual(text, frozen[int(number)])
        self.assertNotIn("VeriCode", "".join(claims.values()))

    def test_english_is_the_default_and_both_dictionaries_have_the_same_keys(self):
        self.assertIn('export const LANG = "en";', self.i18n)
        self.assertIn('export const LOCALE = "en-US";', self.i18n)
        pt, en = dictionary("PT"), dictionary("EN")
        self.assertEqual(set(pt) - set(en), set(), "PT keys without EN")
        self.assertEqual(set(en) - set(pt), set(), "EN keys without PT")
        self.assertIn("adv.why.thisRun", pt)
        for key in pt:
            with self.subTest(key=key):
                self.assertEqual(sorted(re.findall(r"\{(\w+)\}", pt[key])), sorted(re.findall(r"\{(\w+)\}", en[key])))
                self.assertEqual(pt[key].count("`"), en[key].count("`"))
                self.assertEqual(pt[key].count("`") % 2, 0)

    def test_design_md_labels_in_both_languages(self):
        design = (REPO / "DESIGN.md").read_text()
        section = design.split("## Labels and vocabulary\n", 1)[1].split("\n## ", 1)[0]
        rows = re.findall(r"^\| `([a-zA-Z.]+)` \| ([^|]+) \| ([^|]+) \|$", section, re.MULTILINE)
        self.assertGreaterEqual(len(rows), 30)
        pt, en = dictionary("PT"), dictionary("EN")
        for key, portuguese, english in rows:
            if key == "verify.fallback":
                self.assertNotIn(key, pt)
                continue
            code_key = DESIGN_KEY_MAP.get(key, key)
            with self.subTest(key=key):
                self.assertEqual(pt[code_key], portuguese.strip())
                self.assertEqual(en[code_key], english.strip())

    def test_no_forbidden_english_words(self):
        words = (r"verifier router", r"fallback", r"trustless", r"nobody needs to trust", r"\baudit", r"\bprivate\b",
                 r"new machine", r"real money", r"zk on-chain", r"\bsecure\b", r"\bsafe\b", r"trustworthy", r"guarantee",
                 r"certified", r"approved", r"verified agent", r"reputation", r"\bscore\b", r"trust level",
                 r"any repository", r"code is correct", r"mascot")
        for key, text in dictionary("EN").items():
            lowered = text.lower()
            for word in words:
                with self.subTest(key=key, word=word):
                    self.assertIsNone(re.search(word, lowered))
            # "rejected" names the program's rejection in the scenario results, never a UI verdict
            if "reject" in lowered:
                with self.subTest(key=key):
                    self.assertTrue(key.startswith("scenario.") or key in ("adv.expect", "adv.lead"), key)

    def test_r_ui_fixes_stay_in_the_job_view(self):
        job = (STATIC / "js" / "views" / "job.js").read_text()
        # C-UI-3: the landed settlement, not only the "ok" one
        self.assertIn('op.outcome === "PASS" && op.status !== "running"', job)
        self.assertNotIn('op.status === "ok")\n', job.split("function settlementOp", 1)[1].split("}", 1)[0])
        # C-UI-2: scenarios only for Jobs first seen since the worker's current start
        self.assertIn('t("adv.why.thisRun")', job)
        self.assertIn("health().startup && health().startup.started_at", job)
        self.assertIn("seen >= started", job)

    def test_brand_and_legacy_names(self):
        self.assertIn("<title>Hive</title>", self.html)
        self.assertIn('<html lang="en">', self.html)
        self.assertIn("<noscript><p>The Hive interface needs JavaScript.</p></noscript>", self.html)
        for path, text in self.js.items():
            for line in text.splitlines():
                if "VeriCode" in line:
                    with self.subTest(file=path.name):
                        self.assertIn("X-VeriCode-Token", line)


if __name__ == "__main__":
    unittest.main()
