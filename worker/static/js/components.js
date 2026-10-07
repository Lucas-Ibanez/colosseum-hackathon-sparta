// Shared components (HIVE_MVP_UI_GUIDE.md §11; DESIGN.md, Components). Values only from
// tokens (app.css); text only from i18n.js; data only from the worker.
import { el, svg, rich, explorerHref, visuallyHidden } from "./dom.js";
import { icon } from "./icons.js";
import { t } from "./i18n.js";
import { fmtInt, fmtUtc, truncate } from "./format.js";

const COPIED_MS = 1500; // HashField: "Copiado" visible for ~1.5 s (DESIGN.md)
const TOAST_MS = 5000; // Toast: ~5 s, errors stay (DESIGN.md)

export function addressExplorer(address) {
  return explorerHref(`https://explorer.solana.com/address/${address}?cluster=devnet`);
}

// --- HashField ---------------------------------------------------------------------

export function hashField(value, { kind = "hex", label, explorer = null, href = null, size = "md" } = {}) {
  if (typeof value !== "string" || !value) return unavailable();
  const short = truncate(value, kind);
  const wrap = el("span", { class: ["hash", `hash--${size}`] });
  let valueNode;
  if (href) {
    valueNode = el("a", { class: "hash__value hash__value--link", attrs: { href } }, visuallyHidden(`${label}: `), short);
  } else if (short === value) {
    valueNode = el("code", { class: "hash__value" }, visuallyHidden(`${label}: `), value);
  } else {
    const text = el("span", { text: short });
    valueNode = el(
      "code",
      { class: "hash__value hash__value--toggle", attrs: { tabindex: "0", role: "button", "aria-expanded": "false" } },
      visuallyHidden(`${t("action.expand", { label })}: `),
      text,
    );
    const toggle = () => {
      const expanded = valueNode.getAttribute("aria-expanded") !== "true";
      valueNode.setAttribute("aria-expanded", String(expanded));
      text.textContent = expanded ? value : short;
      wrap.classList.toggle("is-expanded", expanded);
    };
    valueNode.addEventListener("click", () => {
      const selection = window.getSelection();
      if (selection && String(selection).length > 0) return; // selecting text is not a toggle
      toggle();
    });
    valueNode.addEventListener("keydown", (event) => {
      if (event.key === "Enter" || event.key === " ") {
        event.preventDefault();
        toggle();
      }
    });
  }
  wrap.append(valueNode, copyButton(value, label, valueNode));
  const link = explorerHref(explorer);
  if (link) {
    wrap.append(
      el(
        "a",
        { class: "icon-link", attrs: { href: link, target: "_blank", rel: "noopener noreferrer", "aria-label": t("action.explorerLabel", { label }) } },
        icon("external-link", "sm"),
      ),
    );
  }
  return wrap;
}

function copyButton(value, label, valueNode) {
  const status = el("span", { class: "copy-status", attrs: { "aria-live": "polite" } });
  const button = el(
    "button",
    { class: "icon-button", attrs: { type: "button", "aria-label": t("action.copyLabel", { label }) } },
    icon("copy", "sm"),
  );
  let timer = null;
  button.addEventListener("click", async () => {
    try {
      await navigator.clipboard.writeText(value);
      button.replaceChildren(icon("check", "sm"));
      status.textContent = t("action.copied");
      clearTimeout(timer);
      timer = setTimeout(() => {
        button.replaceChildren(icon("copy", "sm"));
        status.textContent = "";
      }, COPIED_MS);
    } catch {
      // Clipboard refused: show the whole value selected, for a manual copy.
      if (valueNode.getAttribute("aria-expanded") === "false") valueNode.click();
      const range = document.createRange();
      range.selectNodeContents(valueNode);
      const selection = window.getSelection();
      selection.removeAllRanges();
      selection.addRange(range);
    }
  });
  return el("span", { class: "copy" }, button, status);
}

export function unavailable(note) {
  return el("span", { class: "unavailable" }, icon("minus", "sm"), t("data.unavailable"), note ? ` (${note})` : null);
}

// --- ProvenanceBadge ---------------------------------------------------------------------

const PROVENANCE = {
  chain: ["blocks", (o) => t("prov.chain", { n: fmtInt(o.slot) })],
  tx: ["blocks", (o) => t("prov.tx", { n: fmtInt(o.slot) })],
  worker: ["server", () => t("prov.worker")],
  local: ["cpu", () => t("prov.local")],
  fixed: ["lock", () => t("prov.fixed")],
};

export function provenance(kind, options = {}) {
  const [iconName, text] = PROVENANCE[kind];
  const badge = el("span", { class: "provenance" }, icon(iconName, "sm"), el("span", { text: text(options) }));
  if (!options.at) return badge;
  return el("span", { class: "provenance-wrap" }, badge, el("span", { class: "meta", text: fmtUtc(options.at) }));
}

// --- StatusLabel / StatusBanner ---------------------------------------------------------------

export const STATE_META = {
  Draft: ["pending", "circle-dashed"],
  Created: ["pending", "circle-dashed"],
  Funded: ["pending", "lock"],
  Proving: ["pending", "loader-circle"],
  Submitted: ["pending", "file-check"],
  Delivered: ["pending", "package-check"],
  Released: ["success", "circle-check"],
  RefundedOnFail: ["pending", "undo-2"],
  RefundedOnTimeout: ["pending", "undo-2"],
  Failed: ["caution", "triangle-alert"],
};

export function statusLabel(state, { scope = null, size = "sm", active = false } = {}) {
  if (!state) return el("span", { class: ["status-label", "status--pending"] }, icon("minus", size), t("state.none"));
  const [family, iconName] = STATE_META[state] || ["pending", "info"];
  const node = el(
    "span",
    { class: ["status-label", `status--${family}`, `status-label--${size}`] },
    icon(iconName, size, state === "Proving" && active ? "icon--spin" : ""),
    el("span", { text: t(`state.${state}`) }),
  );
  if (scope && scope !== "chain") node.append(el("span", { class: "status-label__scope", text: `(${t(`scope.${scope}`)})` }));
  return node;
}

// family: success | danger | caution | pending
export function statusBanner({ family, iconName, title, detail, children = [], live = false, cls = "" }) {
  return el(
    "div",
    { class: ["banner", `banner--${family}`, cls], attrs: live ? { role: "status" } : {} },
    el("span", { class: "banner__icon" }, icon(iconName, "md")),
    el(
      "div",
      { class: "banner__body" },
      rich(title, "p", { class: "banner__title" }),
      detail ? rich(detail, "p", { class: "banner__detail" }) : null,
      ...children,
    ),
  );
}

// small icon + label + family, without container (link results, checks)
export function resultLabel(family, iconName, text) {
  return el("span", { class: ["status-label", `status--${family}`] }, icon(iconName, "sm"), rich(text));
}

// --- PartyMark (DESIGN.md, Shapes: hexagon, identical for every outcome) ------------------------

export function partyMark(role, { label = true } = {}) {
  const points = "12,2 20.66,7 20.66,17 12,22 3.34,17 3.34,7";
  return el(
    "span",
    { class: "party" },
    svg("svg", { class: "partymark", viewBox: "0 0 24 24", "aria-hidden": "true", focusable: "false" }, svg("polygon", { points })),
    label ? el("span", { class: "party__role", text: t(`party.${role}`) }) : null,
  );
}

// --- buttons and panels -------------------------------------------------------------------

export function button(label, { variant = "secondary", iconName = null, onClick = null, disabled = false, focusKey = null } = {}) {
  const node = el(
    "button",
    {
      class: ["button", `button--${variant}`],
      attrs: { type: "button", disabled: disabled || null, "data-focus-key": focusKey },
      on: onClick ? { click: onClick } : {},
    },
    iconName ? icon(iconName, "sm") : null,
    el("span", { text: label }),
  );
  return node;
}

// A button whose disabled state always shows why (DESIGN.md, Botões).
export function action(label, options = {}, reason = null) {
  const node = button(label, { ...options, disabled: options.disabled || Boolean(reason) });
  if (!reason) return el("div", { class: "action" }, node);
  const id = `why-${Math.random().toString(36).slice(2, 10)}`;
  node.setAttribute("aria-describedby", id);
  return el("div", { class: "action" }, node, rich(reason, "p", { class: "field-help", attrs: { id } }));
}

export function panel({ title, lead = null, id = null, children = [], cls = "", headingLevel = 2, aside = null }) {
  const heading = el(`h${headingLevel}`, { class: "panel__title", attrs: { id } }, title);
  return el(
    "section",
    { class: ["panel", cls], attrs: id ? { "aria-labelledby": id } : {} },
    el("div", { class: "panel__head" }, heading, aside),
    lead ? rich(lead, "p", { class: "panel__lead" }) : null,
    ...children,
  );
}

export function definitionList(rows) {
  const list = el("dl", { class: "facts" });
  for (const [term, value, extra] of rows) {
    if (term === null) continue;
    list.append(el("div", { class: "facts__row" }, el("dt", {}, rich(term)), el("dd", {}, value, extra || null)));
  }
  return list;
}

// --- CliEquivalent --------------------------------------------------------------------------

function quote(arg) {
  return /^[A-Za-z0-9_@%+=:,./<>-]+$/.test(arg) ? arg : `'${arg.replace(/'/g, "'\\''")}'`;
}

// Binaries by name (HIVE_MVP_UI_ADAPTATION.md §3.6); keys stay as the worker's placeholders.
// First line: the binary, `--log LOG` and the subcommand; then one option per line.
export function commandLines(argv) {
  const words = argv.map((arg, index) => (index === 0 ? arg.split("/").pop() : arg)).map(quote);
  const first = [words[0]];
  let index = 1;
  while (index < words.length) {
    if (words[index] === "--log" && index + 1 < words.length) {
      first.push(words[index], words[index + 1]);
      index += 2;
    } else if (!words[index].startsWith("--")) {
      first.push(words[index++]);
    } else {
      break;
    }
  }
  const lines = [first.join(" ")];
  while (index < words.length) {
    const line = [words[index++]];
    if (index < words.length && !words[index].startsWith("--")) line.push(words[index++]);
    lines.push(line.join(" "));
  }
  return { lines, single: words.join(" ") };
}

export function cliEquivalent(argv, { caption = null } = {}) {
  const { lines, single } = commandLines(argv);
  const code = el("code", { text: lines.join(" \\\n  ") });
  const status = el("span", { class: "copy-status", attrs: { "aria-live": "polite" } });
  const copy = button(t("cli.copy"), { iconName: "copy" });
  let timer = null;
  copy.addEventListener("click", async () => {
    try {
      await navigator.clipboard.writeText(single);
      status.textContent = t("action.copied");
      clearTimeout(timer);
      timer = setTimeout(() => (status.textContent = ""), COPIED_MS);
    } catch {
      const range = document.createRange();
      range.selectNodeContents(code);
      window.getSelection().removeAllRanges();
      window.getSelection().addRange(range);
    }
  });
  return el(
    "figure",
    { class: "cli" },
    el("div", { class: "cli__head" }, caption ? rich(caption, "figcaption", { class: "cli__caption" }) : el("span"), el("div", { class: "cli__actions" }, status, copy)),
    el("pre", { class: "cli__code" }, el("span", { class: "cli__prompt", attrs: { "aria-hidden": "true" }, text: "$ " }), code),
  );
}

// --- ConfirmDialog (DESIGN.md: confirmation before any transaction) -----------------------------

export function confirmDialog({ title, body = [], argv = null, confirmLabel, primary = true }) {
  return new Promise((resolve) => {
    const id = `dlg-${Math.random().toString(36).slice(2, 10)}`;
    const dialog = el("dialog", { class: "dialog", attrs: { "aria-labelledby": id } });
    const cancel = button(t("action.cancel"));
    const confirm = button(confirmLabel, { variant: primary ? "primary" : "secondary" });
    cancel.addEventListener("click", () => dialog.close("cancel"));
    confirm.addEventListener("click", () => dialog.close("confirm"));
    dialog.append(
      el("h2", { class: "dialog__title", attrs: { id }, text: title }),
      el("div", { class: "dialog__body" }, ...body.map((item) => (typeof item === "string" ? rich(item, "p") : item))),
      argv ? cliEquivalent(argv, { caption: t("cli.template") }) : null,
      el("div", { class: "dialog__actions" }, cancel, confirm),
    );
    dialog.addEventListener("close", () => {
      resolve(dialog.returnValue === "confirm");
      dialog.remove();
    });
    document.body.append(dialog);
    dialog.showModal();
    cancel.focus();
  });
}

// --- Toast ----------------------------------------------------------------------------------

export function toast(text, { error = false } = {}) {
  const region = document.getElementById("toasts");
  if (!region) return;
  const close = el("button", { class: "icon-button", attrs: { type: "button", "aria-label": t("action.dismiss") } }, icon("x", "sm"));
  const node = el(
    "div",
    { class: ["toast", error ? "toast--error" : null] },
    icon(error ? "triangle-alert" : "check", "sm"),
    rich(text, "span", { class: "toast__text" }),
    close,
  );
  close.addEventListener("click", () => node.remove());
  region.append(node);
  if (!error) setTimeout(() => node.remove(), TOAST_MS);
}

// --- API errors as text ---------------------------------------------------------------------------

export function describeError(error) {
  const payload = (error && error.payload) || {};
  if (payload.error === "busy") return t("err.busy", { kind: (payload.running_op && payload.running_op.kind) || "?" });
  if (payload.error === "reconcile_pending") return t("err.pending", { jobs: (payload.jobs || []).map((id) => truncate(id)).join(", ") });
  if (payload.error === "unreachable") return t("err.read", { detail: payload.detail || "" });
  return t("err.http", { status: error ? error.status : "?", code: payload.error || "?" });
}

export function errorBanner(error, { stale = null } = {}) {
  return statusBanner({
    family: "caution",
    iconName: "triangle-alert",
    title: describeError(error),
    detail: stale ? t("data.stale", { at: fmtUtc(stale) }) : null,
    live: true,
  });
}

export function loading(text) {
  return el("p", { class: "loading", attrs: { role: "status" } }, icon("loader-circle", "sm", "icon--spin"), text);
}
