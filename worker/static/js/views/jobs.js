// Jobs: the short list of Jobs this worker knows (GET /api/jobs). The unit of navigation
// is the Job; the list only leads to it (HIVE_MVP_UI_GUIDE.md §4.2).
import { el, replace, rich } from "../dom.js";
import { t } from "../i18n.js";
import { fmtInt, fmtToken } from "../format.js";
import { getJobs } from "../api.js";
import { button, errorBanner, hashField, loading, statusLabel } from "../components.js";

const POLL_MS = 15000;

export function jobsView(ctx) {
  const body = el("div", { class: "page__body" }, loading(t("data.loadingJobs")));
  const node = el(
    "section",
    { class: "page" },
    el(
      "div",
      { class: "page__head" },
      el("h1", { class: "page__title", text: t("jobs.title") }),
      button(t("jobs.new"), { variant: "primary", iconName: "plus", onClick: () => ctx.navigate("#/new") }),
    ),
    body,
  );
  let timer = null;
  let disposed = false;
  let last = null;
  let lastOk = null;

  async function load() {
    try {
      const data = await getJobs();
      last = data.jobs;
      lastOk = Date.now();
      render(null);
    } catch (error) {
      if (error.status === 401 || error.status === 403) return;
      render(error);
    }
    if (!disposed) timer = setTimeout(load, POLL_MS);
  }

  function render(error) {
    const parts = [];
    if (error) parts.push(errorBanner(error, { stale: last ? lastOk : null }));
    if (last && last.length === 0) {
      parts.push(
        el("div", { class: "panel empty" }, el("p", { text: t("empty.jobs") }), button(t("jobs.new"), { iconName: "plus", onClick: () => ctx.navigate("#/new") })),
      );
    } else if (last) {
      parts.push(table(last.slice().reverse(), ctx));
    }
    replace(body, ...parts);
  }

  load();
  return {
    node,
    dispose() {
      disposed = true;
      clearTimeout(timer);
    },
  };
}

function table(jobs, ctx) {
  const head = el(
    "tr",
    {},
    ...["jobs.col.job", "jobs.col.task", "jobs.col.state", "jobs.col.amount", "jobs.col.deadline"].map((key) =>
      el("th", { class: key === "jobs.col.amount" ? "num" : null, attrs: { scope: "col" }, text: t(key) }),
    ),
  );
  const rows = jobs.map((job) => {
    const href = `#/jobs/${job.job_id}`;
    const row = el(
      "tr",
      { class: "row-link" },
      el(
        "td",
        {},
        hashField(job.job_id, { label: "job_id", href }),
        job.origin !== "worker" ? el("span", { class: "meta meta--block", text: t("jobs.external") }) : null,
      ),
      el("td", { text: t("jobs.task") }),
      el("td", {}, statusLabel(job.state, { scope: job.state_scope })),
      el("td", { class: "num" }, job.amount != null ? el("span", {}, fmtToken(job.amount), " ", el("span", { class: "unit", text: t("settle.unit") })) : t("data.unavailable")),
      el(
        "td",
        {},
        job.deadline_slot != null ? el("span", { class: "numeric", text: t("jobs.slot", { n: fmtInt(job.deadline_slot) }) }) : t("data.unavailable"),
        job.read_slot != null ? el("span", { class: "meta meta--block", text: t("jobs.readAt", { n: fmtInt(job.read_slot) }) }) : null,
      ),
    );
    // The whole row opens the Job; the link in the first cell is the keyboard path.
    row.addEventListener("click", (event) => {
      if (event.target.closest("a, button")) return;
      const selection = window.getSelection();
      if (selection && String(selection).length > 0) return;
      ctx.navigate(href);
    });
    return row;
  });
  return el(
    "div",
    { class: "panel panel--table" },
    rich(t("jobs.caption"), "p", { class: "panel__lead" }),
    el("div", { class: "table-wrap" }, el("table", { class: "table" }, el("thead", {}, head), el("tbody", {}, ...rows))),
  );
}
