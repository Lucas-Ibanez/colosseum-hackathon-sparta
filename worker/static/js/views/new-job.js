// Novo job (HIVE_MVP_UI_ADAPTATION.md §3.1): the commitment is fixed by the program and
// read-only; the only field is the deadline offset. Nothing is hashed in the browser.
import { el, replace, rich } from "../dom.js";
import { icon } from "../icons.js";
import { t } from "../i18n.js";
import { fmtDecimal, fmtDuration, fmtInt, fmtToken, fmtUtc, parseTime } from "../format.js";
import { getOp, postCreate, postShow, workerNow } from "../api.js";
import {
  action, addressExplorer, button, cliEquivalent, confirmDialog, definitionList, describeError, errorBanner, hashField,
  panel, provenance, statusBanner, statusLabel, toast, unavailable,
} from "../components.js";

const OP_POLL_MS = 1500;

export function newJobView(ctx) {
  const state = {
    offset: null,
    opId: null,
    op: null,
    submitting: false,
    reading: false,
    error: null,
    fieldError: null,
  };
  const node = el("section", { class: "page" });
  let timer = null;
  let disposed = false;

  const limits = () => {
    const v1 = ctx.health() && ctx.health().v1;
    return (v1 && v1.deadline_offset) || { min: 1560, max: 9000 };
  };

  function argvTemplate(offset) {
    const v1 = (ctx.health() || {}).v1 || {};
    return ["vericode", "--log", "LOG", "job", "create", "--buyer-keypair", "<buyer-keypair>", "--executor", v1.executor || t("data.unavailable"),
      "--amount", String(v1.amount || 1000000), "--deadline-offset", String(offset)];
  }

  async function reread() {
    const clock = ctx.health() && ctx.health().slot_clock;
    const jobId = clock && clock.latest && clock.latest.job_id;
    if (!jobId) return;
    state.reading = true;
    state.error = null;
    render();
    try {
      await postShow(jobId);
      await ctx.refreshHealth();
    } catch (error) {
      state.error = error;
    }
    state.reading = false;
    render();
  }

  async function fund() {
    const { min, max } = limits();
    const offset = state.offset;
    if (!Number.isInteger(offset) || offset < min || offset > max) {
      state.fieldError = t("new.deadlineError", { min: fmtInt(min), max: fmtInt(max) });
      render();
      return;
    }
    const v1 = (ctx.health() || {}).v1 || {};
    const ok = await confirmDialog({
      title: t("new.fund"),
      body: [
        t("new.confirmWhat"),
        t("new.confirmProgram", { id: (v1.escrow && v1.escrow.program_id) || t("data.unavailable") }),
        t("new.confirmAmount", { amount: fmtToken(v1.amount) }),
        t("new.confirmIds"),
        t("new.confirmImmutable"),
      ],
      argv: argvTemplate(offset),
      confirmLabel: t("new.fund"),
    });
    if (!ok || disposed) return;
    state.submitting = true;
    state.error = null;
    render();
    try {
      const started = await postCreate(offset);
      state.opId = started.op_id;
      ctx.refreshHealth();
      poll();
    } catch (error) {
      state.error = error;
      state.submitting = false;
      render();
    }
  }

  async function poll() {
    if (disposed || !state.opId) return;
    try {
      state.op = await getOp(state.opId);
    } catch (error) {
      state.error = error;
    }
    render();
    if (state.op && state.op.status === "running") {
      timer = setTimeout(poll, OP_POLL_MS);
    } else if (state.op) {
      state.submitting = false;
      ctx.refreshHealth();
      if (state.op.status === "ok") toast(t("new.funded"));
      else toast(t("new.failed"), { error: true });
      render();
    }
  }

  function render() {
    const health = ctx.health();
    const v1 = (health && health.v1) || {};
    const terms = v1.terms || {};
    const mint = v1.mint || {};
    const clock = (health && health.slot_clock) || {};
    const { min, max } = limits();
    if (state.offset === null) state.offset = max;
    const locked = Boolean(state.opId);

    // form column: the fixed commitment and the one field
    const input = el("input", {
      class: "field__input field__input--numeric",
      attrs: {
        id: "deadline", type: "number", inputmode: "numeric", min, max, step: "1", value: String(state.offset),
        "aria-describedby": "deadline-help deadline-error", disabled: locked || null, "data-focus-key": "deadline",
      },
    });
    input.addEventListener("input", () => {
      state.offset = Number(input.value);
      state.fieldError = null;
      renderSummary();
    });
    const termsRows = definitionList([
      [t("new.spec"), el("span", { text: t("new.specValue") })],
      [t("new.specHash"), hashField(terms.spec_hash, { label: "spec_hash" })],
      [t("new.harnessHash"), hashField(terms.harness_hash, { label: "harness_hash" })],
      [t("new.imageId"), hashField(terms.image_id, { label: "image_id" })],
      [t("new.executor"), hashField(v1.executor, { kind: "address", label: t("new.executor"), explorer: v1.executor && addressExplorer(v1.executor) }), rich(t("new.executorHelp"), "span", { class: "meta meta--block" })],
      [t("new.amount"), el("span", { class: "numeric" }, fmtToken(v1.amount) || t("data.unavailable"), " ", el("span", { class: "unit", text: t("settle.unit") })), el("span", { class: "meta meta--block", text: t("new.amountHelp", { units: fmtInt(v1.amount) }) })],
      [t("new.mint"), hashField(mint.address, { kind: "address", label: t("new.mint"), explorer: mint.address && addressExplorer(mint.address) }), el("span", { class: "meta meta--block", text: t("new.mintHelp", { d: mint.decimals }) })],
    ]);
    const latest = clock.latest;
    const slotLine = latest
      ? el("p", { class: "field-help" }, el("span", { class: "numeric", text: t("new.slotNow", { n: fmtInt(latest.slot) }) }), " ", el("span", { class: "meta", text: t("new.slotAge", { at: fmtUtc(latest.read_at), ago: fmtDuration(Math.max(0, (workerNow() - parseTime(latest.read_at)) / 1000)) || t("data.unavailable") }) }))
      : el("p", { class: "field-help", text: t("new.slotNone") });
    const reread = button(state.reading ? t("chain.reading") : t("chain.reread"), {
      iconName: "refresh-cw", onClick: rereadHandler, disabled: state.reading || !latest || locked, focusKey: "reread",
    });
    const form = panel({
      title: t("new.terms"),
      id: "terms-title",
      lead: t("new.termsHelp"),
      aside: provenance("fixed"),
      children: [
        termsRows,
        el(
          "div",
          { class: "field" },
          el("label", { class: "field__label", attrs: { for: "deadline" }, text: t("new.deadline") }),
          input,
          el("p", { class: "field-help", attrs: { id: "deadline-help" }, text: t("new.deadlineHelp", { min: fmtInt(min), max: fmtInt(max) }) }),
          state.fieldError ? el("p", { class: "field-error", attrs: { id: "deadline-error", role: "alert" } }, icon("triangle-alert", "sm"), state.fieldError) : el("p", { attrs: { id: "deadline-error" } }),
          slotLine,
          el("div", { class: "inline-actions" }, reread),
        ),
        cliEquivalent(argvTemplate(state.offset || max), { caption: t("cli.template") }),
        locked ? null : action(t("new.fund"), { variant: "primary", iconName: "lock", onClick: fund, disabled: state.submitting, focusKey: "fund" }),
      ],
    });
    form.classList.add("new-job__form");

    const summary = el("aside", { class: "new-job__summary", attrs: { "aria-labelledby": "summary-title" } });
    replace(
      node,
      el("div", { class: "page__head" }, el("h1", { class: "page__title", text: t("new.title") })),
      rich(t("new.lead"), "p", { class: "page__lead" }),
      state.error ? errorBanner(state.error) : null,
      el("div", { class: "grid-12 new-job" }, form, summary),
      limitations(),
    );
    renderSummary(summary);
  }

  function rereadHandler() {
    reread();
  }

  function renderSummary(target) {
    const summary = target || node.querySelector(".new-job__summary");
    if (!summary) return;
    const health = ctx.health() || {};
    const v1 = health.v1 || {};
    const clock = health.slot_clock || {};
    const op = state.op;
    const live = (op && op.create_live) || {};
    const offset = state.offset;
    const latest = clock.latest;
    const rate = clock.seconds_per_slot;
    const valid = Number.isInteger(offset);
    const forecast = latest && valid ? el("p", { class: "numeric", text: t("new.forecast", { n: fmtInt(latest.slot + offset) }) }) : null;
    const minutes = valid && rate
      ? el("p", { class: "meta", text: t("new.minutes", { m: fmtInt(Math.round((offset * rate) / 60)), s: fmtDecimal(rate, 2) }) })
      : el("p", { class: "meta", text: t("new.minutesNone") });
    const jobId = op && op.job_id;
    const opState = op ? (op.status === "running" ? (op.signature ? "Submitted" : "Draft") : op.status === "ok" ? "Funded" : "Failed") : "Draft";
    const rows = definitionList([
      [t("new.state"), statusLabel(opState, { scope: opState === "Funded" ? "chain" : opState === "Draft" ? "interface" : "worker" })],
      [t("new.jobId"), jobId ? hashField(jobId, { label: "job_id" }) : el("span", { class: "meta", text: t("new.generated") })],
      [t("new.vault"), live.vault ? hashField(live.vault, { kind: "address", label: t("new.vault"), explorer: addressExplorer(live.vault) }) : el("span", { class: "meta", text: t("new.generated") })],
      [t("new.amount"), el("span", { class: "numeric" }, fmtToken(v1.amount) || t("data.unavailable"), " ", el("span", { class: "unit", text: t("settle.unit") }))],
      [t("job.deadline"), live.slot
        ? el("span", { class: "numeric", text: t("job.deadlineSlot", { n: fmtInt(live.slot.deadline_slot) }) })
        : el("div", {}, forecast || unavailable(), minutes)],
    ]);
    const parts = [rows];
    if (op && op.status === "running") {
      parts.push(statusBanner({ family: "pending", iconName: "loader-circle", title: t("new.funding"), live: true }));
    }
    if (op && op.status !== "running") {
      const tx = op.transaction || {};
      if (op.status === "ok") {
        parts.push(
          statusBanner({
            family: "pending",
            iconName: "lock",
            title: t("new.funded"),
            children: [
              tx.signature ? hashField(tx.signature, { kind: "signature", label: t("ops.signature"), explorer: tx.explorer }) : null,
              op.chain && op.chain.slot ? provenance("chain", { slot: op.chain.slot, at: op.chain.read_at }) : null,
            ],
            live: true,
          }),
          button(t("new.open"), { variant: "primary", onClick: () => ctx.navigate(`#/jobs/${jobId}`), focusKey: "open" }),
        );
      } else {
        parts.push(
          statusBanner({
            family: "caution",
            iconName: "triangle-alert",
            title: t("new.failed"),
            children: [el("p", { class: "mono-detail", text: op.error || op.cli_error || op.outcome || t("data.unavailable") })],
            live: true,
          }),
        );
        if (jobId) parts.push(button(t("new.open"), { onClick: () => ctx.navigate(`#/jobs/${jobId}`) }));
      }
    }
    replace(summary, el("h2", { class: "panel__title", attrs: { id: "summary-title" }, text: t("new.summary") }), ...parts);
  }

  render();
  if (!ctx.health() || !ctx.health().v1) ctx.refreshHealth().then(() => !disposed && render());
  return {
    node,
    dispose() {
      disposed = true;
      clearTimeout(timer);
    },
  };
}

export function limitations() {
  return el(
    "aside",
    { class: "limitations", attrs: { "aria-label": t("limit.title") } },
    el("h2", { class: "limitations__title", text: t("limit.title") }),
    rich(t("limit.p3"), "p"),
    rich(t("limit.proving"), "p"),
    rich(t("claim.f5"), "p"),
  );
}

export { describeError };
