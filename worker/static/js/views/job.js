// Detalhe do job: the product screen (HIVE_MVP_UI_GUIDE.md §4.4, adapted by
// HIVE_MVP_UI_ADAPTATION.md §3). Fixed order: header, A timeline, B commitments, C
// verification, D settlement, adversarial scenarios, E limits, F CLI. The interface shows
// evidence and starts real operations; the program decides.
import { el, replace, rich, explorerHref } from "../dom.js";
import { icon } from "../icons.js";
import { t } from "../i18n.js";
import { fmtDecimal, fmtDuration, fmtInt, fmtToken, fmtTokenDelta, fmtUtc, parseTime, truncate } from "../format.js";
import { getJob, getJobs, getOp, postNegative, postProve, postRefund, postSettle, postShow, workerNow } from "../api.js";
import {
  action, addressExplorer, button, cliEquivalent, confirmDialog, definitionList, describeError, errorBanner, hashField,
  loading, panel, partyMark, provenance, resultLabel, statusBanner, statusLabel, toast,
} from "../components.js";
import { limitations } from "./new-job.js";

const FAST_MS = 2000;
const SLOW_MS = 15000;
const U32_MAX = 4294967295;
const TERMINAL = ["Released", "RefundedOnFail", "RefundedOnTimeout"];
const NEGATIVES = ["escrow-6021", "escrow-6014", "verifier-6003", "escrow-6007"];
const NEGATIVE_META = {
  "escrow-6014": ["danger", "unlink", "escrow:6014"],
  "verifier-6003": ["danger", "file-x", "verifier:6003"],
  "escrow-6007": ["caution", "repeat", "escrow:6007"],
  "escrow-6021": ["caution", "hourglass", "escrow:6021"],
};
const BANNERS = {
  Released: ["success", "circle-check"],
  RefundedOnFail: ["danger", "circle-x"],
  RefundedOnTimeout: ["caution", "clock"],
  Failed: ["caution", "triangle-alert"],
  Submitted: ["pending", "file-check"],
};

const reducedMotion = () => window.matchMedia("(prefers-reduced-motion: reduce)").matches;

export function jobView(ctx, jobId) {
  const node = el("section", { class: "page page--job" }, loading(t("data.loadingJob")));
  const st = {
    job: null,
    ops: new Map(),
    others: new Map(),
    candidates: [],
    firstSeen: null,
    error: null,
    lastOk: null,
    reading: false,
    starting: false,
    watchOp: null,
    inputs: { input: "", output: "" },
    inputError: null,
    receiptChoice: null,
    lastStep: null,
  };
  let timer = null;
  let ticker = null;
  let disposed = false;
  let signature = null;
  const live = el("p", { class: "visually-hidden", attrs: { "aria-live": "polite" } });

  const summaries = () => (st.job ? st.job.ops : []);
  const detail = (opId) => st.ops.get(opId) || null;
  const health = () => ctx.health() || {};
  const busy = () => Boolean(health().running_op || (st.job && st.job.running_op) || st.starting);

  async function load() {
    clearTimeout(timer);
    try {
      const job = await getJob(jobId);
      for (const summary of job.ops) {
        const cached = st.ops.get(summary.op_id);
        if (!cached || cached.status === "running" || cached.status !== summary.status) st.ops.set(summary.op_id, await getOp(summary.op_id));
      }
      for (const summary of job.ops) {
        if (summary.receipt_job_id && !st.others.has(summary.receipt_job_id)) st.others.set(summary.receipt_job_id, await getJob(summary.receipt_job_id));
      }
      if (job.origin === "worker") {
        const list = await getJobs();
        // D-NEG and D-6014: when this Job was first seen, and the other receipts newest first.
        st.firstSeen = (list.jobs.find((item) => item.job_id === jobId) || {}).first_seen || null;
        st.candidates = list.jobs
          .filter((item) => item.job_id !== jobId && item.origin === "worker" && item.receipt_status === "usable")
          .sort((a, b) => (parseTime(b.first_seen) ?? 0) - (parseTime(a.first_seen) ?? 0));
      }
      st.job = job;
      st.error = null;
      st.lastOk = Date.now();
      noteFinished();
    } catch (error) {
      if (error.status === 401 || error.status === 403) return;
      st.error = error;
    }
    if (disposed) return;
    renderIfChanged();
    timer = setTimeout(load, active() ? FAST_MS : SLOW_MS);
  }

  function active() {
    if (!st.job) return false;
    return Boolean(st.job.running_op || st.watchOp || ["Proving", "Submitted"].includes(st.job.state) || health().running_op);
  }

  function noteFinished() {
    if (!st.watchOp) return;
    const op = st.ops.get(st.watchOp);
    if (!op || op.status === "running") return;
    if (op.status === "ok") toast(t(`ops.done.${op.kind}`));
    else toast(t("ops.failed"), { error: true });
    st.watchOp = null;
    ctx.refreshHealth();
  }

  function renderIfChanged() {
    const next = JSON.stringify([
      st.job,
      [...st.ops.values()].map((op) => [op.op_id, op.status, op.running_step, op.signature, (op.steps || []).length]),
      st.candidates.map((item) => item.job_id),
      st.firstSeen,
      st.error && [st.error.status, st.error.payload && st.error.payload.error],
      st.reading,
      st.starting,
      Boolean(health().running_op),
      st.inputError,
    ]);
    if (next === signature) return;
    signature = next;
    render();
  }

  // --- operations ------------------------------------------------------------------------

  async function start(fn) {
    st.starting = true;
    renderIfChanged();
    try {
      const started = await fn();
      st.watchOp = started.op_id;
    } catch (error) {
      toast(describeError(error), { error: true });
    }
    st.starting = false;
    await ctx.refreshHealth();
    load();
  }

  async function reread() {
    st.reading = true;
    renderIfChanged();
    try {
      await postShow(jobId);
      toast(t("chain.readDone"));
    } catch (error) {
      toast(describeError(error), { error: true });
    }
    st.reading = false;
    await ctx.refreshHealth();
    load();
  }

  async function prove() {
    const parse = (text) => (/^\d{1,10}$/.test(text.trim()) ? Number(text.trim()) : NaN);
    const input = parse(st.inputs.input);
    const output = parse(st.inputs.output);
    if (!(input <= U32_MAX && output <= U32_MAX)) {
      st.inputError = t("ops.u32");
      renderIfChanged();
      return;
    }
    st.inputError = null;
    const ok = await confirmDialog({
      title: t("ops.proveTitle"),
      body: [t("ops.proveWhat"), t("ops.proveArtifact", { input: fmtInt(input), output: fmtInt(output) }), t("limit.proving")],
      argv: ["vericode-prover", "prove", jobId, String(input), String(output), "RD"],
      confirmLabel: t("ops.prove"),
    });
    if (ok) start(() => postProve(jobId, input, output));
  }

  async function settle(receipt) {
    const v1 = health().v1 || {};
    const instruction = receipt.verdict === "FAIL" ? "refund_on_fail" : "release";
    const ok = await confirmDialog({
      title: t("ops.settleTitle"),
      body: [
        t("ops.settleWhat", { instruction, verdict: receipt.verdict }),
        t("ops.settleProgram", { id: (v1.escrow && v1.escrow.program_id) || t("data.unavailable"), verifier: (v1.verifier && v1.verifier.program_id) || t("data.unavailable") }),
      ],
      argv: ["vericode", "--log", "LOG", "job", "settle", "--job-id", jobId, "--receipt", "RD", "--deliver", "--executor-keypair", "<executor-keypair>"],
      confirmLabel: t("ops.settle"),
    });
    if (ok) start(() => postSettle(jobId));
  }

  async function refund() {
    const v1 = health().v1 || {};
    const ok = await confirmDialog({
      title: t("ops.refundTitle"),
      body: [t("ops.refundWhat"), t("ops.refundProgram", { id: (v1.escrow && v1.escrow.program_id) || t("data.unavailable") })],
      argv: ["vericode", "--log", "LOG", "job", "refund-timeout", "--job-id", jobId, "--payer-keypair", "<buyer-keypair>"],
      confirmLabel: t("ops.refund"),
      primary: false,
    });
    if (ok) start(() => postRefund(jobId));
  }

  async function negative(kind) {
    const other = kind === "escrow-6014" ? st.receiptChoice || (st.candidates[0] && st.candidates[0].job_id) : null;
    const base = ["vericode", "--log", "LOG", "job"];
    const argv = {
      "escrow-6021": [...base, "refund-timeout", "--job-id", jobId, "--payer-keypair", "<buyer-keypair>", "--expect-error", "escrow:6021"],
      "escrow-6014": [...base, "settle", "--job-id", jobId, "--receipt", "RD", "--deliver", "--executor-keypair", "<executor-keypair>", "--expect-error", "escrow:6014"],
      "verifier-6003": [...base, "settle", "--job-id", jobId, "--receipt", "RD", "--deliver", "--executor-keypair", "<executor-keypair>", "--tamper-seal", "--expect-error", "verifier:6003"],
      "escrow-6007": [...base, "settle", "--job-id", jobId, "--receipt", "RD", "--payer-keypair", "<executor-keypair>", "--expect-error", "escrow:6007"],
    }[kind];
    const name = t(`adv.${kind.split("-")[1]}`);
    const ok = await confirmDialog({
      title: t("adv.confirmTitle", { name }),
      body: [t(`adv.what.${kind}`, { other: other || t("data.unavailable") }), t("adv.expect", { expect: NEGATIVE_META[kind][2] })],
      argv,
      confirmLabel: name,
      primary: false,
    });
    if (ok) start(() => postNegative(jobId, kind, other));
  }

  // --- rendering ---------------------------------------------------------------------------

  function render() {
    const focusKey = document.activeElement && document.activeElement.dataset ? document.activeElement.dataset.focusKey : null;
    if (!st.job) {
      replace(node, st.error ? errorBanner(st.error) : loading(t("data.loadingJob")));
      return;
    }
    const job = st.job;
    replace(
      node,
      header(job),
      st.error ? errorBanner(st.error, { stale: st.lastOk }) : null,
      resultBanner(job),
      showOperations(job) ? operations(job) : null,
      timeline(job),
      commitments(job),
      verification(job),
      settlement(job),
      job.origin === "worker" ? adversarial(job) : null,
      limits(),
      cli(job),
      limitations(),
      live,
    );
    if (focusKey) {
      const target = node.querySelector(`[data-focus-key="${CSS.escape(focusKey)}"]`);
      if (target) target.focus();
    }
    startTicker();
  }

  function startTicker() {
    clearInterval(ticker);
    const nodes = node.querySelectorAll("[data-elapsed-start]");
    if (!nodes.length) return;
    const tick = () => {
      for (const item of nodes) {
        const startAt = parseTime(item.dataset.elapsedStart);
        if (startAt !== null) item.textContent = fmtDuration(Math.max(0, (workerNow() - startAt) / 1000)) || "";
      }
    };
    tick();
    ticker = setInterval(tick, 1000);
  }

  function header(job) {
    const chain = job.chain && !job.chain.absent ? job.chain : null;
    const amount = chain ? el("span", { class: "metric" }, fmtToken(chain.amount), " ", el("span", { class: "unit", text: t("settle.unit") })) : el("span", { text: t("data.unavailable") });
    let deadline = el("span", { text: t("data.unavailable") });
    if (chain) {
      const remaining = chain.deadline_slot - chain.slot;
      deadline = el(
        "span",
        {},
        el("span", { class: "numeric", text: t("job.deadlineSlot", { n: fmtInt(chain.deadline_slot) }) }),
        // A settled Job has no deadline left to count (RUI-11).
        TERMINAL.includes(chain.status) ? null : el("span", { class: "meta meta--block", text: chain.past_deadline ? t("job.passed") : t("job.remaining", { n: fmtInt(remaining) }) }),
      );
    }
    const rereadButton = button(st.reading ? t("chain.reading") : t("chain.reread"), {
      iconName: "refresh-cw", onClick: reread, disabled: st.reading || busy(), focusKey: "reread",
    });
    const parties = chain
      ? definitionList([
          [t("job.account"), hashField(chain.address, { kind: "address", label: t("job.account"), explorer: chain.explorer })],
          [t("job.vault"), chain.vault ? hashField(chain.vault.address, { kind: "address", label: t("job.vault"), explorer: addressExplorer(chain.vault.address) }) : el("span", { text: t("data.unavailable") })],
          [t("job.buyer"), el("span", { class: "party-row" }, partyMark("buyer", { label: false }), hashField(chain.buyer, { kind: "address", label: t("job.buyer"), explorer: addressExplorer(chain.buyer) }))],
          [t("job.executor"), el("span", { class: "party-row" }, partyMark("executor", { label: false }), hashField(chain.executor, { kind: "address", label: t("job.executor"), explorer: addressExplorer(chain.executor) }))],
        ])
      : null;
    return el(
      "header",
      { class: "job-head panel" },
      el("div", { class: "job-head__id" }, el("h1", { class: "page__title" }, el("span", { text: `${t("job.title")} ` }), hashField(job.job_id, { label: "job_id", size: "lg" }))),
      el(
        "div",
        { class: "job-head__facts" },
        el("div", { class: "job-head__fact" }, el("span", { class: "fact-label", text: t("jobs.col.state") }), statusLabel(job.state, { scope: job.state_scope, size: "md", active: true })),
        el("div", { class: "job-head__fact" }, el("span", { class: "fact-label", text: t("job.amount") }), amount),
        el("div", { class: "job-head__fact" }, el("span", { class: "fact-label", text: t("job.deadline") }), deadline),
        el(
          "div",
          { class: "job-head__fact job-head__fact--read" },
          chain ? provenance("chain", { slot: chain.slot, at: chain.read_at }) : el("span", { text: t("data.unavailable") }),
          rereadButton,
        ),
      ),
      job.origin !== "worker" ? rich(t("job.external"), "p", { class: "meta" }) : null,
      parties,
    );
  }

  function resultBanner(job) {
    const parts = [];
    if (job.reconcile_pending) {
      parts.push(statusBanner({ family: "caution", iconName: "triangle-alert", title: t("job.pending"), live: true }));
    }
    const key = BANNERS[job.state] ? job.state : null;
    if (!key) return parts.length ? el("div", { class: "stack" }, ...parts) : null;
    const [family, iconName] = BANNERS[key];
    const settle = settlementOp(job);
    const link = settle && explorerHref(settle.explorer);
    parts.push(
      statusBanner({
        family,
        iconName,
        title: t(`banner.${key}.title`),
        // A create that left no Job account on the chain has nothing to prove or settle (RUI-05).
        detail: key === "Failed" && job.chain && job.chain.absent ? t("banner.FailedCreate.detail") : t(`banner.${key}.detail`),
        children: [
          link
            ? el("p", { class: "banner__links" },
                el("a", { class: "link", attrs: { href: link, target: "_blank", rel: "noopener noreferrer" } }, icon("external-link", "sm"), t("action.explorer")),
                " ",
                el("button", { class: "link link--button", attrs: { type: "button" }, on: { click: () => focusBlock("blk-d") } }, t("settle.anatomy")))
            : null,
        ],
        live: true,
        cls: "banner--result",
      }),
    );
    return el("div", { class: "stack" }, ...parts);
  }

  function focusBlock(id) {
    const target = document.getElementById(id);
    if (!target) return;
    target.setAttribute("tabindex", "-1");
    target.scrollIntoView({ behavior: reducedMotion() ? "auto" : "smooth", block: "start" });
    target.focus({ preventScroll: true });
  }

  // --- operations panel ---------------------------------------------------------------------

  // Operations belong to a Job that can still move; a settled Job shows its evidence only.
  function showOperations(job) {
    const status = job.chain && !job.chain.absent ? job.chain.status : null;
    return Boolean(job.running_op) || ["Funded", "Delivered"].includes(status) || job.state === "Failed" || job.state === "Submitted";
  }

  function operations(job) {
    const chain = job.chain && !job.chain.absent ? job.chain : null;
    const status = chain ? chain.status : null;
    const receipt = job.receipt;
    const usable = receipt && receipt.status === "usable";
    const worker = job.origin === "worker";
    const isBusy = busy();
    const children = [];

    const running = job.running_op ? detail(job.running_op.op_id) : null;
    if (job.running_op && job.running_op.kind === "prove") {
      children.push(provingProgress(running || job.running_op));
    } else if (job.running_op) {
      const kind = job.running_op.negative_kind || job.running_op.kind;
      children.push(
        statusBanner({
          family: "pending",
          iconName: "loader-circle",
          title: t("ops.running", { kind }),
          children: [job.running_op.signature ? hashField(job.running_op.signature, { kind: "signature", label: t("ops.signature") }) : null],
          live: true,
        }),
      );
    } else if (usable && receipt.local) {
      children.push(provingFinal(receipt));
    }

    if (worker) {
      const proveReason = isBusy ? t("ops.busy") : status !== "Funded" ? t("ops.notFunded") : null;
      const settleReason = isBusy ? t("ops.busy") : status !== "Funded" ? t("ops.notFunded") : !usable ? t("ops.noReceipt") : null;
      const proveIsNext = !proveReason && !usable;
      const inputField = (key, label) => {
        const input = el("input", {
          class: "field__input field__input--numeric",
          attrs: { id: `prove-${key}`, type: "text", inputmode: "numeric", autocomplete: "off", value: st.inputs[key], "data-focus-key": `prove-${key}`, disabled: proveReason ? true : null, "aria-describedby": "prove-help" },
        });
        input.addEventListener("input", () => {
          st.inputs[key] = input.value;
        });
        return el("div", { class: "field field--inline" }, el("label", { class: "field__label", attrs: { for: `prove-${key}` }, text: label }), input);
      };
      children.push(
        el(
          "div",
          { class: "op-group" },
          el("h3", { class: "op-group__title", text: t("ops.prove") }),
          rich(t("ops.proveHelp"), "p", { class: "field-help", attrs: { id: "prove-help" } }),
          el("div", { class: "field-row" }, inputField("input", t("ops.input")), inputField("output", t("ops.output"))),
          st.inputError ? el("p", { class: "field-error", attrs: { role: "alert" } }, icon("triangle-alert", "sm"), st.inputError) : null,
          action(t("ops.prove"), { variant: proveIsNext ? "primary" : "secondary", iconName: "cpu", onClick: prove, focusKey: "prove" }, proveReason),
        ),
        el(
          "div",
          { class: "op-group" },
          el("h3", { class: "op-group__title", text: t("ops.settle") }),
          rich(t("ops.settleHelp"), "p", { class: "field-help" }),
          usable ? rich(t("ops.hasReceipt", { verdict: `\`${receipt.verdict}\`` }), "p", { class: "meta" }) : null,
          action(t("ops.settle"), { variant: !settleReason ? "primary" : "secondary", iconName: "file-check", onClick: () => settle(receipt), focusKey: "settle" }, settleReason),
        ),
      );
    } else {
      children.push(rich(t("ops.externalJob"), "p", { class: "meta" }));
    }

    let refundReason = null;
    if (isBusy) refundReason = t("ops.busy");
    else if (!chain || !["Funded", "Delivered"].includes(status)) refundReason = t("ops.notFunded");
    else if (!chain.past_deadline) refundReason = `${t("ops.refundDisabled", { n: fmtInt(chain.deadline_slot + 1) })}. ${t("ops.refundRead")}`;
    children.push(
      el(
        "div",
        { class: "op-group" },
        el("h3", { class: "op-group__title", text: t("ops.refund") }),
        rich(t("ops.refundHelp"), "p", { class: "field-help" }),
        action(t("ops.refund"), { iconName: "undo-2", onClick: refund, focusKey: "refund" }, refundReason),
      ),
    );
    return panel({ title: t("ops.title"), id: "blk-ops", children: [el("div", { class: "op-grid" }, ...children)] });
  }

  function provingProgress(op) {
    const step = op.running_step ? op.running_step.name : null;
    if (step !== st.lastStep) {
      st.lastStep = step;
      live.textContent = step ? t("proving.live", { step }) : "";
    }
    return el(
      "div",
      { class: "proving", attrs: { role: "group", "aria-labelledby": "proving-title" } },
      el("h3", { class: "proving__title", attrs: { id: "proving-title" } }, icon("loader-circle", "md", "icon--spin"), t("proving.title")),
      el(
        "p",
        { class: "proving__elapsed" },
        el("span", { class: "fact-label", text: t("proving.elapsed") }),
        el("span", { class: "metric-md", data: { elapsedStart: op.started_at } }),
      ),
      rich(step ? t("proving.step", { step }) : t("proving.stepNone"), "p", { class: "proving__step" }),
      rich(t("proving.scope"), "p", { class: "meta" }),
      provenance("local"),
    );
  }

  function provingFinal(receipt) {
    const p = Number(receipt.local["prove.seconds"]);
    const c = Number(receipt.local["compress.seconds"]);
    if (!Number.isFinite(p) || !Number.isFinite(c)) return null;
    return el(
      "div",
      { class: "proving proving--done" },
      el("h3", { class: "proving__title", text: t("proving.final") }),
      el("p", { class: "metric-md", text: fmtDuration(p + c) }),
      rich(t("proving.finalDetail", { p: fmtDecimal(p), c: fmtDecimal(c) }), "p", { class: "meta" }),
      provenance("local"),
    );
  }

  // --- A: StateTimeline ----------------------------------------------------------------------

  function timeline(job) {
    const ops = summaries();
    const chain = job.chain && !job.chain.absent ? job.chain : null;
    const status = chain ? chain.status : null;
    const current = job.state || status;
    const create = ops.find((op) => op.kind === "create" && op.outcome === "PASS");
    const proves = ops.filter((op) => op.kind === "prove");
    const settle = ops.filter((op) => op.kind === "settle" && op.signature).at(-1);
    const refundOp = ops.filter((op) => op.kind === "refund-timeout" && op.outcome === "PASS").at(-1);
    const terminal = TERMINAL.includes(status);
    const reached = {
      Draft: true,
      Funded: Boolean(create) || ["Funded", "Delivered", ...TERMINAL].includes(status),
      Proving: proves.length > 0,
      Submitted: Boolean(settle),
      Delivered: ["Delivered", "Released", "RefundedOnFail"].includes(status),
      Released: status === "Released",
      RefundedOnFail: status === "RefundedOnFail",
      RefundedOnTimeout: status === "RefundedOnTimeout",
      Failed: current === "Failed",
    };
    const order = ["Draft", "Funded", "Proving", "Submitted", "Delivered", "terminal"];
    const position = { Failed: 2, Released: 5, RefundedOnFail: 5, RefundedOnTimeout: 1 }[current] ?? order.indexOf(current);
    const phase = (name, index) => {
      if (name === current) return "current";
      if (reached[name]) return "done";
      if (!terminal && index > position) return "future";
      return null;
    };
    const link = (name) => {
      const op = name === "Funded" ? create : ["Submitted", "Delivered", "Released", "RefundedOnFail"].includes(name) ? settle : name === "RefundedOnTimeout" ? refundOp : null;
      const href = op && explorerHref(op.explorer);
      return href
        ? el("a", { class: "link timeline__link", attrs: { href, target: "_blank", rel: "noopener noreferrer", "aria-label": t("action.explorerLabel", { label: name }) } }, icon("external-link", "sm"), t("action.explorer"))
        : null;
    };
    const scopeOf = { Draft: "interface", Proving: "local", Submitted: "worker", Failed: "worker" };
    const nodeFor = (name, phaseName, note = null) =>
      el(
        "div",
        { class: ["timeline__node", `timeline__node--${phaseName}`], attrs: { "aria-current": phaseName === "current" ? "step" : null } },
        el("span", { class: "timeline__dot", attrs: { "aria-hidden": "true" } }),
        el("span", { class: "timeline__label" }, statusLabel(name, { active: phaseName === "current" })),
        el("span", { class: "meta timeline__scope", text: `${t(`scope.${scopeOf[name] || "chain"}`)}, ${t(`timeline.${phaseName}`)}` }),
        note ? rich(note, "span", { class: "meta timeline__note" }) : null,
        phaseName !== "future" ? link(name) : null,
      );

    const columns = [];
    order.forEach((name, index) => {
      if (name === "terminal") {
        const alternatives = ["Released", "RefundedOnFail"]
          .map((alt) => [alt, phase(alt, index)])
          .filter(([, phaseName]) => phaseName);
        if (alternatives.length) columns.push({ name, nodes: alternatives.map(([alt, phaseName]) => nodeFor(alt, phaseName)) });
        return;
      }
      const phaseName = phase(name, index);
      if (!phaseName) return;
      let note = null;
      if (name === "Delivered" && reached.Delivered) note = t("timeline.sameTx");
      if (name === "Funded" && !create && reached.Funded) note = t("timeline.external");
      columns.push({ name, nodes: [nodeFor(name, phaseName, note)] });
    });
    const branches = {
      Funded: reached.RefundedOnTimeout ? nodeFor("RefundedOnTimeout", phase("RefundedOnTimeout", 99) || "current", t("timeline.byTimeout"))
        : status === "Funded" ? nodeFor("RefundedOnTimeout", "future", t("timeline.byTimeout")) : null,
      Proving: reached.Failed ? nodeFor("Failed", "current", t("timeline.operational")) : null,
    };
    const track = el("ol", { class: "timeline__track" }, ...columns.map((column) => el("li", { class: "timeline__col" }, ...column.nodes)));
    const hasBranch = columns.some((column) => branches[column.name]);
    const branchRow = hasBranch
      ? el("div", { class: "timeline__branches", attrs: { "aria-label": t("timeline.title") } },
          ...columns.map((column) => el("div", { class: "timeline__col" }, branches[column.name] || null)))
      : null;
    return panel({
      title: t("timeline.title"),
      id: "blk-a",
      children: [el("div", { class: "timeline" }, track, branchRow), el("p", { class: "meta", text: t("timeline.legend") })],
    });
  }

  // --- B: CommitmentVsObserved -------------------------------------------------------------------

  function commitments(job) {
    const chain = job.chain && !job.chain.absent ? job.chain : null;
    const receipt = job.receipt;
    const journal = receipt ? receipt.journal : null;
    return panel({
      title: t("commit.title"),
      id: "blk-b",
      children: [
        el(
          "div",
          { class: "sources" },
          chain ? el("span", { class: "sources__item" }, el("span", { class: "fact-label", text: t("commit.committed") }), provenance("chain", { slot: chain.slot, at: chain.read_at })) : null,
          el("span", { class: "sources__item" }, el("span", { class: "fact-label", text: t("commit.observed") }), provenance("local")),
        ),
        commitmentTable({ jobId: job.job_id, chain, journal }),
        journal ? rich(t("commit.source"), "p", { class: "meta" }) : null,
        rich(t("commit.note"), "p", { class: "note" }),
      ],
    });
  }

  // --- C: VerificationPanel ----------------------------------------------------------------------

  function verification(job) {
    return panel({
      title: t("verify.title"),
      id: "blk-c",
      children: [el("div", { class: "grid-2" }, localVerification(job), onchainVerification(job))],
    });
  }

  function localVerification(job) {
    const receipt = job.receipt;
    const v1 = health().v1 || {};
    const body = [];
    if (!receipt) {
      body.push(el("p", { class: "meta", text: t("verify.localNone") }));
    } else if (receipt.status === "running") {
      body.push(statusBanner({ family: "pending", iconName: "loader-circle", title: t("proving.title") }));
    } else if (receipt.status !== "usable") {
      body.push(statusBanner({ family: "caution", iconName: "triangle-alert", title: t("verify.localFailed"), children: [el("p", { class: "mono-detail", text: receipt.failure || "" })] }));
    } else {
      const local = receipt.local || {};
      const terms = v1.terms || {};
      const guest = v1.guest || {};
      const check = (key, expected, label = null) => {
        const value = local[key];
        const ok = value !== null && value !== undefined && value === expected;
        return el(
          "li",
          { class: "check" },
          resultLabel(ok ? "success" : "danger", ok ? "circle-check" : "circle-x", label || `\`${key}=${value ?? t("data.unavailable")}\``),
          label ? el("span", { class: "check__value" }, el("code", { class: "code-inline", text: `${key}=` }), hashField(value, { label: key, size: "sm" })) : null,
        );
      };
      const seconds = (key) => Number(local[key]);
      body.push(
        el("p", { class: "meta" }, rich(t("verify.receiptOf", { n: receipt.n, op: receipt.op_id })), " ", provenance("local")),
        el(
          "ul",
          { class: "checks" },
          check("prove.receipt_type", "Composite"),
          check("prove.local_verify", "ok"),
          check("prove.journal_equal_to_core", "true"),
          check("compress.receipt_type", "Groth16"),
          check("compress.local_verify", "ok"),
          check("compress.journal_equal_to_composite", "true"),
          check("compress.image_id", terms.image_id, t("verify.imageAdmitted")),
          check("compress.selector", guest.selector || "73c457ba"),
        ),
        definitionList([
          [t("verify.times"), el("span", { class: "numeric", text: fmtDuration(seconds("prove.seconds") + seconds("compress.seconds")) || t("data.unavailable") }), rich(t("proving.finalDetail", { p: fmtDecimal(seconds("prove.seconds")), c: fmtDecimal(seconds("compress.seconds")) }), "span", { class: "meta meta--block" })],
          [t("verify.dockerRun"), el("code", { class: "mono-detail", text: receipt.docker_run || "" })],
          [t("verify.shim"), receipt.shim ? el("span", {}, hashField(receipt.shim.sha256, { label: "shim" }), el("span", { class: "meta", text: ` ${receipt.shim.mode}` })) : el("span", { text: t("data.unavailable") })],
          [t("verify.memory"), receipt.mem_available_kb_before_compress ? el("span", { class: "numeric", text: t("verify.mib", { n: fmtInt(Math.round(receipt.mem_available_kb_before_compress / 1024)) }) }) : el("span", { text: t("data.unavailable") })],
        ]),
        rich(t("claim.f8"), "p", { class: "claim" }),
      );
    }
    return el("section", { class: "subpanel", attrs: { "aria-labelledby": "verify-local" } }, el("h3", { class: "subpanel__title", attrs: { id: "verify-local" }, text: t("verify.local") }), ...body);
  }

  function onchainVerification(job) {
    const status = job.chain && !job.chain.absent ? job.chain.status : null;
    const settle = settlementOp(job);
    const settleDetail = settle ? detail(settle.op_id) : null;
    const tx = settleDetail ? settleDetail.transaction || {} : {};
    const v1 = health().v1 || {};
    const body = [];
    if (settle && settle.kind === "settle" && tx.verifier_invoked === true) {
      const verifierCall = ((settleDetail.anatomy && settleDetail.anatomy.invocations) || []).find((call) => call.program === "verifier");
      const href = explorerHref(tx.explorer);
      // "Verified on-chain" and phrase 1 only next to the settlement link (RUI-08).
      if (href) body.push(statusBanner({ family: "success", iconName: "circle-check", title: t("verify.onchainOk") }));
      body.push(
        definitionList([
          [t("verify.program"), hashField(v1.verifier && v1.verifier.program_id, { kind: "address", label: t("program.verifier"), explorer: v1.verifier && addressExplorer(v1.verifier.program_id) }), el("span", { class: "meta meta--block", text: t("env.verifierNote", { release: (v1.verifier && v1.verifier.release) || "" }) })],
          [t("verify.units"), el("span", { class: "numeric", text: tx.verifier_units != null ? `${fmtInt(tx.verifier_units)} CU` : t("data.unavailable") })],
          [t("verify.result"), verifierCall && verifierCall.result === "success" && verifierCall.instruction ? resultLabel("success", "circle-check", `\`${verifierCall.instruction}\` ${t("settle.ok")}`) : el("span", { text: t("data.unavailable") })],
          [t("verify.tx"), hashField(tx.signature, { kind: "signature", label: t("verify.tx"), explorer: tx.explorer }), tx.slot ? provenance("tx", { slot: tx.slot }) : null],
        ]),
        href
          ? el(
              "div",
              { class: "claim claim--linked" },
              rich(t("claim.f1"), "p"),
              el("a", { class: "link", attrs: { href, target: "_blank", rel: "noopener noreferrer" } }, icon("external-link", "sm"), t("action.explorer")),
            )
          : null,
        reconcileNote(settle, status),
      );
    } else if (!settle && ["Released", "RefundedOnFail"].includes(status)) {
      // Terminal on the chain, settled by a transaction this worker did not record (RUI-04).
      body.push(statusBanner({ family: "pending", iconName: "blocks", title: t("settle.external") }));
    } else if (status === "RefundedOnTimeout") {
      body.push(statusBanner({ family: "pending", iconName: "clock", title: t("verify.onchainPending"), detail: t("verify.timeoutNone") }));
    } else {
      body.push(statusBanner({ family: "pending", iconName: "circle-dashed", title: t("verify.onchainPending"), detail: t("verify.onchainPendingDetail") }));
    }
    return el("section", { class: "subpanel", attrs: { "aria-labelledby": "verify-onchain" } }, el("h3", { class: "subpanel__title", attrs: { id: "verify-onchain" }, text: t("verify.onchain") }), ...body);
  }

  // --- D: settlement (TransactionAnatomy, BalanceDelta) -------------------------------------------

  // The landed settlement: a PASS outcome (the transaction landed as expected) whose
  // operation is no longer running. Its status may be "failed" when only the reconcile
  // `job show` failed (RUI-03); the chain state still comes from the last `job show`.
  function settlementOp(job) {
    return summaries()
      .filter((op) => (op.kind === "settle" || op.kind === "refund-timeout") && op.outcome === "PASS" && op.status !== "running")
      .at(-1) || null;
  }

  function reconcileNote(settle, status) {
    if (!settle || settle.status !== "failed" || !TERMINAL.includes(status)) return null;
    return el("p", { class: "meta", text: t("settle.reconcileFailed") });
  }

  function settlement(job) {
    const status = job.chain && !job.chain.absent ? job.chain.status : null;
    const settle = settlementOp(job);
    const opDetail = settle ? detail(settle.op_id) : null;
    if (!settle && TERMINAL.includes(status)) {
      return panel({
        title: t("settle.title"),
        id: "blk-d",
        children: [
          el("p", { class: "settle-result" }, statusLabel(status, { size: "md" }), " ", rich(t("settle.reason", { reason: t(`settle.reason.${status}`) }))),
          statusBanner({ family: "pending", iconName: "blocks", title: t("settle.external") }),
        ],
      });
    }
    if (!settle || !opDetail || !TERMINAL.includes(status)) {
      return panel({ title: t("settle.title"), id: "blk-d", children: [statusBanner({ family: "pending", iconName: "circle-dashed", title: t("settle.none") })] });
    }
    return panel({
      title: t("settle.title"),
      id: "blk-d",
      children: [
        el("p", { class: "settle-result" }, statusLabel(status, { size: "md" }), " ", rich(t("settle.reason", { reason: t(`settle.reason.${status}`) }))),
        reconcileNote(settle, status),
        el("div", { class: "grid-12 settle-grid" }, anatomyBlock(opDetail), balancesBlock(opDetail)),
      ],
    });
  }

  function anatomyBlock(opDetail) {
    const tx = opDetail.transaction || {};
    const anatomy = opDetail.anatomy;
    const head = definitionList([
      [t("verify.tx"), hashField(tx.signature, { kind: "signature", label: t("verify.tx"), explorer: tx.explorer })],
      [t("settle.slot"), el("span", { class: "numeric", text: fmtInt(tx.slot) || t("data.unavailable") })],
      [t("settle.cu"), el("span", { class: "numeric", text: tx.units != null ? `${fmtInt(tx.units)} CU` : t("data.unavailable") })],
      [t("settle.fee"), el("span", { class: "numeric", text: tx.fee != null ? t("settle.lamports", { n: fmtInt(tx.fee) }) : t("data.unavailable") })],
      [t("settle.status"), tx.err === null ? resultLabel("pending", "check", t("settle.statusOk")) : rich(t("settle.statusErr", { err: JSON.stringify(tx.err) }))],
    ]);
    const body = [head];
    if (!anatomy || !anatomy.landed) {
      body.push(el("p", { class: "meta", text: t("settle.notLanded") }));
    } else {
      body.push(invocationList(anatomy.invocations));
    }
    return el(
      "section",
      { class: "anatomy", attrs: { "aria-labelledby": "anatomy-title" } },
      el("h3", { class: "subpanel__title", attrs: { id: "anatomy-title" }, text: t("settle.anatomy") }),
      rich(t("settle.anatomyHelp"), "p", { class: "meta" }),
      el("div", { class: "anatomy__tx" }, ...body),
    );
  }

  function invocationList(invocations) {
    const tops = [];
    for (const call of invocations) {
      if (call.depth === 1 || !tops.length) tops.push({ call, children: [] });
      else tops[tops.length - 1].children.push(call);
    }
    const row = (call, label) =>
      el(
        "div",
        { class: ["invocation", call.result === "failed" ? "invocation--failed" : null] },
        el(
          "div",
          { class: "invocation__head" },
          el("span", { class: "invocation__index", text: label }),
          el("span", { class: "invocation__program" }, rich(t(`program.${call.program || "unknown"}`))),
          hashField(call.program_id, { kind: "address", label: t(`program.${call.program || "unknown"}`).replace(/`/g, ""), size: "sm" }),
        ),
        el(
          "div",
          { class: "invocation__body" },
          call.instruction ? el("code", { class: "invocation__ix", text: call.instruction }) : el("span", { class: "meta", text: t("settle.noInstruction") }),
          el("span", { class: "numeric invocation__cu", text: call.units != null ? `${fmtInt(call.units)} CU` : "" }),
          call.result === "failed"
            ? resultLabel("danger", "circle-x", call.anchor_error ? `${t("settle.fail")}: \`${call.anchor_error.name}\` (${call.anchor_error.code})` : t("settle.fail"))
            : resultLabel("pending", "check", t("settle.ok")),
        ),
      );
    return el(
      "ol",
      { class: "anatomy__list" },
      ...tops.map((top, index) =>
        el(
          "li",
          { class: "anatomy__item" },
          row(top.call, t("settle.instruction", { n: index + 1 })),
          top.children.length ? el("ol", { class: "anatomy__cpis" }, ...top.children.map((child) => el("li", {}, row(child, t("settle.cpi"))))) : null,
        ),
      ),
    );
  }

  function balancesBlock(opDetail) {
    const anatomy = opDetail.anatomy;
    const tx = opDetail.transaction || {};
    const rows = (anatomy && anatomy.balances) || [];
    const table = el(
      "table",
      { class: "table table--compact" },
      el("thead", {}, el("tr", {}, ...["settle.account", "settle.before", "settle.after", "settle.delta"].map((key, index) => el("th", { class: index ? "num" : null, attrs: { scope: "col" }, text: t(key) })))),
      el(
        "tbody",
        {},
        ...rows.map((row) => {
          const roleKey = row.role ? `settle.role.${row.role}` : "settle.role.other";
          const delta = row.after != null ? row.after - (row.before || 0) : null;
          return el(
            "tr",
            {},
            el(
              "th",
              { attrs: { scope: "row" } },
              el("span", { class: "party-row" }, row.role === "buyer" || row.role === "executor" ? partyMark(row.role) : icon("lock", "sm"), el("span", { text: t(roleKey) })),
              hashField(row.token_account || row.owner, { kind: "address", label: t(roleKey), explorer: (row.token_account || row.owner) && addressExplorer(row.token_account || row.owner), size: "sm" }),
            ),
            el("td", { class: "num" }, row.before != null ? fmtToken(row.before) : el("span", { class: "meta", text: t("settle.absent") })),
            el("td", { class: "num" }, row.after != null ? fmtToken(row.after) : t("data.unavailable")),
            el("td", { class: "num" }, delta != null ? fmtTokenDelta(delta) : t("data.unavailable")),
          );
        }),
      ),
    );
    return el(
      "section",
      { class: "balances", attrs: { "aria-labelledby": "balances-title" } },
      el("h3", { class: "subpanel__title", attrs: { id: "balances-title" }, text: t("settle.balances") }),
      el("p", { class: "meta" }, `${t("settle.unit")} `, tx.slot ? provenance("tx", { slot: tx.slot }) : null),
      el("div", { class: "table-wrap" }, table),
    );
  }

  // --- adversarial scenarios (ScenarioOutcome) -----------------------------------------------------

  function adversarial(job) {
    const chain = job.chain && !job.chain.absent ? job.chain : null;
    const status = chain ? chain.status : null;
    const receipt = job.receipt;
    const usable = receipt && receipt.status === "usable";
    const isBusy = busy();
    const margin = health().v1 ? health().v1.c10_8_margin : 300;
    // D-NEG (C-UI-2): writes only on Jobs this worker first saw since its current start, so a
    // take never sends a scenario to a consumed Job. Unknown times keep the scenarios off.
    const started = parseTime(health().startup && health().startup.started_at);
    const seen = parseTime(st.firstSeen);
    const thisRun = started !== null && seen !== null && seen >= started;
    const why = (reason) => (thisRun ? reason : t("adv.why.thisRun"));
    const reasons = {
      "escrow-6021": why(isBusy ? t("ops.busy") : status !== "Funded" ? t("adv.why.funded") : chain.deadline_slot - chain.slot < margin ? t("adv.why.margin", { m: fmtInt(margin) }) : null),
      "escrow-6014": why(isBusy ? t("ops.busy") : status !== "Funded" ? t("adv.why.funded") : !st.candidates.length ? t("adv.noOther") : null),
      "verifier-6003": why(isBusy ? t("ops.busy") : status !== "Funded" ? t("adv.why.funded") : !usable ? t("adv.why.receipt") : null),
      "escrow-6007": why(isBusy ? t("ops.busy") : !["Released", "RefundedOnFail"].includes(status) ? t("adv.why.settled") : !usable ? t("adv.why.receipt") : null),
    };
    const select = st.candidates.length
      ? (() => {
          const node = el("select", { class: "field__input field__input--mono", attrs: { id: "receipt-choice", "data-focus-key": "receipt-choice" } },
            ...st.candidates.map((item) => el("option", { attrs: { value: item.job_id, selected: (st.receiptChoice || st.candidates[0].job_id) === item.job_id ? true : null }, text: truncate(item.job_id) })));
          node.addEventListener("change", () => (st.receiptChoice = node.value));
          return el("div", { class: "field field--inline" }, el("label", { class: "field__label", attrs: { for: "receipt-choice" }, text: t("adv.receiptOf") }), node);
        })()
      : null;
    const buttons = NEGATIVES.map((kind) =>
      el(
        "div",
        { class: "op-group" },
        kind === "escrow-6014" ? select : null,
        action(t(`adv.${kind.split("-")[1]}`), { iconName: NEGATIVE_META[kind][1], onClick: () => negative(kind), focusKey: kind }, reasons[kind]),
      ),
    );
    const history = summaries().filter((op) => op.kind === "negative").slice().reverse();
    return panel({
      title: t("adv.title"),
      id: "blk-adv",
      lead: t("adv.lead"),
      cls: "panel--adversarial",
      children: [
        el("div", { class: "op-grid op-grid--4" }, ...buttons),
        el("h3", { class: "subpanel__title", text: t("adv.history") }),
        history.length ? el("div", { class: "stack" }, ...history.map((op) => scenarioOutcome(op, job))) : el("p", { class: "meta", text: t("adv.none") }),
      ],
    });
  }

  function scenarioOutcome(summary, job) {
    const kind = summary.negative_kind;
    const [family, iconName] = NEGATIVE_META[kind] || ["caution", "triangle-alert"];
    const opDetail = detail(summary.op_id) || {};
    if (summary.status === "running") {
      return statusBanner({ family: "pending", iconName: "loader-circle", title: `${t(`scenario.${kind}`)}: ${t("scenario.running")}`, live: true });
    }
    const rejection = summary.rejection;
    if (!rejection) {
      return statusBanner({
        family: "caution",
        iconName: "triangle-alert",
        title: t("scenario.noneTitle"),
        detail: t("scenario.noneDetail", { outcome: summary.outcome || t("data.unavailable") }),
        children: [el("p", { class: "mono-detail", text: opDetail.error || "" }), el("p", { class: "meta", text: fmtUtc(summary.ended_at) || "" })],
      });
    }
    const calls = ((opDetail.anatomy && opDetail.anatomy.invocations) || []).filter((call) => call.program_id === rejection.program_id && call.result === "failed");
    const failing = calls.sort((a, b) => b.depth - a.depth)[0];
    const named = failing && failing.anchor_error && failing.anchor_error.code === rejection.code ? failing.anchor_error.name : null;
    const program = t(`scenario.program.${rejection.program}`);
    const children = [
      rich(named ? t("scenario.rejectedBy", { program, name: named, code: rejection.code }) : t("scenario.rejectedByCode", { program, code: rejection.code }), "p", { class: "banner__line" }),
    ];
    const tx = opDetail.transaction || {};
    if (tx.watched_unchanged === true) children.push(el("p", { class: "banner__line", text: t("scenario.unchanged") }));
    if (kind === "escrow-6021" && opDetail.c10_8) {
      children.push(el("p", { class: "banner__line numeric", text: t("scenario.margin", { slot: fmtInt(opDetail.c10_8.slot), deadline: fmtInt(opDetail.c10_8.deadline_slot), margin: fmtInt(opDetail.c10_8.margin), min: fmtInt(opDetail.c10_8.required) }) }));
    }
    if (kind === "escrow-6014" && summary.receipt_job_id) {
      const other = st.others.get(summary.receipt_job_id);
      const journal = other && other.receipt ? other.receipt.journal : null;
      children.push(
        el("p", { class: "banner__line" }, rich(t("scenario.receiptOf", { id: truncate(summary.receipt_job_id) })), " ", hashField(summary.receipt_job_id, { label: "job_id", size: "sm" })),
        commitmentTable({ jobId: job.job_id, chain: job.chain, journal, only: ["job_id", "spec_hash", "harness_hash", "image_id"] }),
        journal ? rich(t("commit.otherSource", { id: truncate(summary.receipt_job_id) }), "p", { class: "meta" }) : null,
      );
    }
    children.push(
      el("div", { class: "banner__links" }, hashField(rejection.signature, { kind: "signature", label: t("verify.tx"), explorer: rejection.explorer, size: "sm" }), el("span", { class: "meta", text: fmtUtc(summary.ended_at) || "" })),
    );
    return statusBanner({ family, iconName, title: t(`scenario.${kind}`), detail: t(`scenario.detail.${kind}`), children, cls: "banner--scenario" });
  }

  // --- E and F --------------------------------------------------------------------------------------

  function limits() {
    return el(
      "section",
      { class: "limits", attrs: { "aria-labelledby": "blk-e" } },
      el("h2", { class: "panel__title", attrs: { id: "blk-e" }, text: t("limits.title") }),
      el(
        "div",
        { class: "grid-2" },
        el("div", {}, el("h3", { class: "limits__subtitle", text: t("limits.shows") }), rich(t("claim.f2"), "p", { class: "limits__text" })),
        el("div", {}, el("h3", { class: "limits__subtitle", text: t("limits.not") }), rich(t("claim.f3"), "p", { class: "limits__text" })),
      ),
    );
  }

  function cli(job) {
    const ops = summaries().filter((op) => op.kind !== "show" && op.kind !== "check");
    const shows = summaries().filter((op) => op.kind === "show");
    if (shows.length) ops.push(shows[shows.length - 1]);
    const blocks = [];
    for (const summary of ops) {
      const opDetail = detail(summary.op_id);
      if (!opDetail) continue;
      const steps = (opDetail.steps || []).filter((step) => ["cli", "prove", "compress", "show"].includes(step.name) && Array.isArray(step.argv));
      const kind = summary.kind === "negative" ? t("kind.negative", { neg: summary.negative_kind }) : t(`kind.${summary.kind}`);
      steps.forEach((step) => blocks.push(cliEquivalent(step.argv, { caption: t("cli.op", { kind: `\`${kind}\``, at: fmtUtc(step.started_at) || "" }) })));
    }
    return panel({
      title: t("cli.title"),
      id: "blk-f",
      lead: t("cli.lead"),
      children: blocks.length ? blocks : [el("p", { class: "meta", text: t("cli.none") })],
    });
  }

  load();
  return {
    node,
    dispose() {
      disposed = true;
      clearTimeout(timer);
      clearInterval(ticker);
    },
  };
}

// CommitmentVsObserved (adaptation §3.3): committed (job show) × published (journal).
export function commitmentTable({ jobId, chain, journal, only = null }) {
  const NA = Symbol("na");
  const NOT_DELIVERED = Symbol("not-delivered");
  const committedChain = chain && !chain.absent ? chain : {};
  const rows = [
    ["schema_version", NA, journal ? String(journal.schema_version) : null, "plain"],
    ["job_id", jobId, journal ? journal.job_id : null, "hex"],
    ["spec_hash", committedChain.spec_hash, journal ? journal.spec_hash : null, "hex"],
    ["harness_hash", committedChain.harness_hash, journal ? journal.harness_hash : null, "hex"],
    ["artifact_hash", committedChain.artifact_hash || NOT_DELIVERED, journal ? journal.artifact_hash : null, "hex"],
    ["image_id", committedChain.image_id, journal ? journal.image_id : null, "hex"],
    ["verdict", NA, journal ? journal.verdict : null, "verdict"],
  ].filter(([name]) => !only || only.includes(name));
  const cell = (value, kind, name) => {
    if (value === NA) return el("span", { class: "meta", text: t("commit.na") });
    if (value === NOT_DELIVERED) return el("span", { class: "meta", text: t("commit.notDelivered") });
    if (value === null || value === undefined) return el("span", { class: "meta", text: t("commit.noReceipt") });
    if (kind === "verdict") return el("span", {}, el("code", { class: "code-inline", text: value }), " ", t(value === "PASS" ? "verdict.pass" : "verdict.fail"));
    if (kind === "plain") return el("code", { class: "code-inline", text: value });
    return hashField(value, { label: name });
  };
  const body = rows.map(([name, committed, observed, kind]) => {
    let link;
    let diverge = false;
    if (committed === NA) link = resultLabel("pending", "minus", t("link.na"));
    else if (committed === NOT_DELIVERED || observed === null || observed === undefined || committed === undefined) link = resultLabel("pending", "circle-dashed", t("link.wait"));
    else if (committed === observed) link = resultLabel("success", "equal", t("link.ok"));
    else {
      link = resultLabel("danger", "equal-not", t("link.bad"));
      diverge = true;
    }
    return el(
      "tr",
      { class: diverge ? "row--diverge" : null },
      el("th", { attrs: { scope: "row" } }, el("code", { class: "code-inline", text: name }), name === "artifact_hash" && committed !== NOT_DELIVERED ? rich(t("commit.byDeliver"), "span", { class: "meta meta--block" }) : null),
      el("td", {}, cell(committed, kind, `${name} (${t("commit.committed")})`)),
      el("td", {}, cell(observed, kind, `${name} (${t("commit.observed")})`)),
      el("td", {}, link),
    );
  });
  return el(
    "div",
    { class: "table-wrap" },
    el(
      "table",
      { class: "table table--commit" },
      el("thead", {}, el("tr", {}, ...["commit.field", "commit.committed", "commit.observed", "commit.link"].map((key) => el("th", { attrs: { scope: "col" }, text: t(key) })))),
      el("tbody", {}, ...body),
    ),
  );
}
