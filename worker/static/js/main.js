// Shell, routing and the token gate. Navigation never reloads the page, so the token
// stays in this tab's memory only (api.js).
import { el, replace, rich } from "./dom.js";
import { preloadIcons, icon } from "./icons.js";
import { t, LANG } from "./i18n.js";
import { fmtInt, truncate } from "./format.js";
import { ApiError, getHealth, hasToken, onTokenRefused, setToken } from "./api.js";
import { hashField, statusBanner, addressExplorer, loading } from "./components.js";
import { jobsView } from "./views/jobs.js";
import { newJobView } from "./views/new-job.js";
import { jobView } from "./views/job.js";

const HEALTH_MS = 20000;
const JOB_ROUTE = /^#\/jobs\/([0-9a-f]{64})$/;

const app = {
  health: null,
  healthError: null,
  view: null,
  gateMessage: null,
  nodes: {},
};

const ctx = {
  health: () => app.health,
  refreshHealth,
  navigate: (hash) => {
    if (location.hash === hash) route();
    else location.hash = hash;
  },
};

// --- shell -------------------------------------------------------------------------------

function navItem(hash, iconName, label, key) {
  return el(
    "li",
    {},
    el("a", { class: "app-nav__item", attrs: { href: hash }, data: { nav: key } }, icon(iconName, "md"), el("span", { text: label })),
  );
}

function buildShell() {
  const root = document.getElementById("app");
  const brand = el(
    "a",
    { class: "brand", attrs: { href: "#/jobs", "aria-label": t("brand.home") } },
    // BrandMark: the provisional signature of brand/ (TODO(brand): swap for the vector master)
    el("img", { class: "brand__mark", attrs: { src: "/ui/brand/hive-horizontal-branco.svg", alt: t("brand.name") } }),
  );
  const nav = el(
    "nav",
    { class: "app-nav", attrs: { "aria-label": t("nav.label") } },
    brand,
    el("ul", { class: "app-nav__list" }, navItem("#/jobs", "list", t("nav.jobs"), "jobs"), navItem("#/new", "plus", t("nav.new"), "new")),
  );
  const crumbs = el("nav", { class: "crumbs", attrs: { "aria-label": t("crumb.label") } });
  const running = el("span", { class: "top-bar__running", attrs: { role: "status" } });
  const top = el(
    "header",
    { class: "top-bar" },
    crumbs,
    el(
      "div",
      { class: "top-bar__right" },
      running,
      el("span", { class: "env-chip" }, t("top.devnet")),
      el("span", { class: "env-chip" }, t("top.testToken")),
      el("span", { class: "signer" }, icon("terminal", "sm"), t("top.signer")),
    ),
  );
  const main = el("main", { class: "content", attrs: { id: "main", tabindex: "-1" } });
  const statusItems = el("div", { class: "status-bar__items" });
  const detailsButton = el(
    "button",
    { class: "status-bar__details", attrs: { type: "button", "aria-expanded": "false", "aria-controls": "env-details" } },
    icon("info", "sm"),
    el("span", { text: t("env.detailsShort") }),
  );
  detailsButton.setAttribute("aria-label", t("env.details"));
  const details = el("div", { class: "env-details", attrs: { id: "env-details", hidden: true, role: "region", "aria-label": t("env.details") } });
  const status = el("footer", { class: "status-bar", attrs: { "aria-label": t("env.label") } }, statusItems, detailsButton);
  detailsButton.addEventListener("click", () => toggleDetails());
  document.addEventListener("keydown", (event) => {
    if (event.key === "Escape" && !details.hidden) {
      toggleDetails(false);
      detailsButton.focus();
    }
  });
  const skip = el("button", { class: "skip-link", attrs: { type: "button" }, text: t("skip") });
  skip.addEventListener("click", () => main.focus());
  replace(
    root,
    skip,
    el("div", { class: "app" }, nav, el("div", { class: "app-main" }, top, main)),
    details,
    status,
    el("div", { class: "toasts", attrs: { id: "toasts", "aria-live": "polite" } }),
  );
  app.nodes = { crumbs, main, running, statusItems, details, detailsButton };
}

function toggleDetails(force) {
  const { details, detailsButton } = app.nodes;
  const open = force === undefined ? details.hidden : force;
  details.hidden = !open;
  detailsButton.setAttribute("aria-expanded", String(open));
}

function setActiveNav(key) {
  for (const link of document.querySelectorAll(".app-nav__item")) {
    const active = link.dataset.nav === key;
    link.classList.toggle("is-selected", active);
    if (active) link.setAttribute("aria-current", "page");
    else link.removeAttribute("aria-current");
  }
}

function setCrumbs(items) {
  const list = el("ol", { class: "crumbs__list" });
  items.forEach(([label, hash], index) => {
    const last = index === items.length - 1;
    list.append(
      el(
        "li",
        {},
        last ? el("span", { attrs: { "aria-current": "page" } }, rich(label)) : el("a", { class: "link", attrs: { href: hash } }, rich(label)),
      ),
    );
  });
  replace(app.nodes.crumbs, list);
}

// --- environment bar (EnvironmentBar) ----------------------------------------------------

function renderEnvironment() {
  const { statusItems, details } = app.nodes;
  const v1 = app.health && app.health.v1;
  if (!v1) {
    replace(statusItems, el("span", { class: "status-bar__item" }, t("top.devnet")), el("span", { class: "status-bar__item", text: t("env.waiting") }));
    replace(details, el("p", { text: t("env.waiting") }));
    return;
  }
  const verifier = v1.verifier || {};
  const escrow = v1.escrow || {};
  const mint = v1.mint || {};
  const terms = v1.terms || {};
  const commit = app.health.app_commit;
  const code = (value) => el("code", { class: "status-bar__code", text: value });
  replace(
    statusItems,
    el("span", { class: "status-bar__item status-bar__cluster", text: t("top.devnet") }),
    el("span", { class: "status-bar__item" }, t("env.mint"), " ", code(truncate(mint.address || "", "address"))),
    el("span", { class: "status-bar__item" }, rich(t("env.imageIdShort")), " ", code(truncate(terms.image_id || ""))),
    el("span", { class: "status-bar__item" }, t("env.verifierShort"), " ", code(truncate(verifier.program_id || "", "address")), rich(t("env.immutable"))),
    el("span", { class: "status-bar__item" }, t("env.escrowShort"), " ", code(truncate(escrow.program_id || "", "address")), rich(t("env.authorityShort", { value: escrow.upgrade_authority || t("data.unavailable") }))),
    // TODO(data): the commit is unavailable when .git cannot be read at the worker's start
    el("span", { class: "status-bar__item" }, t("env.commit"), " ", commit ? code(commit.slice(0, 7)) : t("data.unavailable")),
  );
  const row = (label, value, note) => el("div", { class: "facts__row" }, el("dt", {}, rich(label)), el("dd", {}, value, note ? rich(note, "span", { class: "meta" }) : null));
  replace(
    details,
    el("h2", { class: "env-details__title", text: t("env.details") }),
    el(
      "dl",
      { class: "facts" },
      row(t("env.cluster"), el("span", { text: t("top.devnet") }), null),
      row(t("env.genesis"), hashField(v1.cluster && v1.cluster.genesis, { kind: "address", label: t("env.genesis") })),
      row(t("env.escrow"), hashField(escrow.program_id, { kind: "address", label: t("env.escrow").replace(/`/g, ""), explorer: escrow.program_id && addressExplorer(escrow.program_id) }), t("env.authority", { value: escrow.upgrade_authority || t("data.unavailable") })),
      row(t("env.sha"), hashField(escrow.program_data_sha256, { label: t("env.sha") }), t("env.bytes", { n: fmtInt(escrow.program_data_bytes) })),
      row(t("env.verifier"), hashField(verifier.program_id, { kind: "address", label: t("env.verifier"), explorer: verifier.program_id && addressExplorer(verifier.program_id) }), t("env.verifierNote", { release: verifier.release || "" })),
      row(t("env.sha"), hashField(verifier.sha256, { label: t("env.sha") }), t("env.bytes", { n: fmtInt(verifier.bytes) })),
      row(t("env.mint"), hashField(mint.address, { kind: "address", label: t("env.mint"), explorer: mint.address && addressExplorer(mint.address) }), t("env.mintDetail", { d: mint.decimals, f: mint.freeze_authority || t("data.unavailable") })),
      row(t("env.imageId"), hashField(terms.image_id, { label: "image_id", size: "lg" })),
      row(t("env.commit"), commit ? hashField(commit, { label: t("env.commit") }) : el("span", { text: t("data.unavailable") }), t("env.commitNote")),
    ),
    rich(t("claim.f4"), "p", { class: "env-details__claim" }),
    rich(t("claim.f5"), "p", { class: "env-details__claim" }),
    rich(t("env.source"), "p", { class: "meta" }),
  );
}

function renderRunning() {
  const op = app.health && app.health.running_op;
  replace(app.nodes.running, op ? el("span", { class: "running" }, icon("loader-circle", "sm", "icon--spin"), t("top.running", { kind: op.negative_kind || op.kind })) : "");
}

async function refreshHealth() {
  if (!hasToken()) return null;
  try {
    app.health = await getHealth();
    app.healthError = null;
  } catch (error) {
    app.healthError = error;
  }
  renderEnvironment();
  renderRunning();
  return app.health;
}

// --- token gate (TokenGate) --------------------------------------------------------------------

function tokenGate() {
  const input = el("input", {
    class: "field__input field__input--mono",
    attrs: { id: "token", type: "password", autocomplete: "off", spellcheck: "false", "aria-describedby": "token-help token-error" },
  });
  const error = el("p", { class: "field-error", attrs: { id: "token-error", role: "alert" } });
  const submit = el("button", { class: "button button--primary", attrs: { type: "button" } }, icon("key-round", "sm"), el("span", { text: t("gate.connect") }));
  const status = el("p", { class: "meta", attrs: { role: "status" } });
  const connect = async () => {
    const value = input.value.trim();
    error.replaceChildren();
    if (!value) {
      error.append(icon("triangle-alert", "sm"), t("gate.empty"));
      return;
    }
    submit.disabled = true;
    status.textContent = t("gate.connecting");
    setToken(value);
    input.value = "";
    try {
      app.health = await getHealth();
      app.gateMessage = null;
      renderEnvironment();
      renderRunning();
      route();
    } catch (err) {
      setToken(null);
      submit.disabled = false;
      status.textContent = "";
      const text = err instanceof ApiError && err.status ? t("gate.refused", { status: err.status }) : t("gate.unreachable", { origin: location.origin });
      error.append(icon("triangle-alert", "sm"), text);
    }
  };
  submit.addEventListener("click", connect);
  input.addEventListener("keydown", (event) => {
    if (event.key === "Enter") connect();
  });
  const node = el(
    "section",
    { class: "page page--narrow" },
    el("h1", { class: "page__title", text: t("gate.title") }),
    app.gateMessage ? statusBanner({ family: "caution", iconName: "triangle-alert", title: app.gateMessage }) : null,
    el(
      "div",
      { class: "panel" },
      rich(t("gate.help"), "p", { class: "panel__lead", attrs: { id: "token-help" } }),
      el("div", { class: "field" }, el("label", { class: "field__label", attrs: { for: "token" }, text: t("gate.field") }), input, error),
      el("div", { class: "form-actions" }, submit, status),
    ),
  );
  return { node, dispose() {}, focus: input };
}

// --- routing ----------------------------------------------------------------------------------

function mount(view, title, crumbs, navKey) {
  if (app.view && app.view.dispose) app.view.dispose();
  app.view = view;
  document.title = title;
  setCrumbs(crumbs);
  setActiveNav(navKey);
  replace(app.nodes.main, view.node);
  if (view.focus) view.focus.focus();
}

function route() {
  // Until the first health read succeeds, the token is not confirmed: stay on the gate.
  if (!hasToken() || !app.health) {
    mount(tokenGate(), t("title.gate"), [[t("gate.title"), "#/jobs"]], null);
    return;
  }
  const hash = location.hash || "#/jobs";
  let match;
  if (hash === "#/jobs" || hash === "#/" || hash === "#") {
    mount(jobsView(ctx), t("title.jobs"), [[t("nav.jobs"), "#/jobs"]], "jobs");
  } else if (hash === "#/new") {
    mount(newJobView(ctx), t("title.new"), [[t("nav.jobs"), "#/jobs"], [t("nav.new"), "#/new"]], "new");
  } else if ((match = JOB_ROUTE.exec(hash))) {
    const id = match[1];
    mount(jobView(ctx, id), t("title.job", { id: truncate(id) }), [[t("nav.jobs"), "#/jobs"], [t("title.job", { id: truncate(id) }).split(" | ")[0], hash]], "jobs");
  } else {
    mount(
      { node: el("section", { class: "page" }, el("h1", { class: "page__title", text: t("missing.title") }), el("p", { text: t("missing.body") })) },
      t("title.missing"),
      [[t("nav.jobs"), "#/jobs"]],
      null,
    );
  }
}

async function boot() {
  document.documentElement.lang = LANG;
  buildShell();
  replace(app.nodes.main, loading(t("data.loadingHealth")));
  await preloadIcons();
  buildShell();
  renderEnvironment();
  onTokenRefused((error) => {
    app.gateMessage = t("gate.lost", { status: error.status });
    app.health = null;
    renderEnvironment();
    route();
  });
  window.addEventListener("hashchange", route);
  setInterval(refreshHealth, HEALTH_MS);
  route();
}

boot();
