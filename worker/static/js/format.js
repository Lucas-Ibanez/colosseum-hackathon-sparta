// Numbers, amounts, durations and dates (DESIGN.md, Numbers and indicators).
import { LOCALE } from "./i18n.js";

const integer = new Intl.NumberFormat(LOCALE, { maximumFractionDigits: 0 });
const two = new Intl.NumberFormat(LOCALE, { minimumFractionDigits: 2, maximumFractionDigits: 2 });
const six = new Intl.NumberFormat(LOCALE, { minimumFractionDigits: 6, maximumFractionDigits: 6 });
const MINUS = "−";

export function fmtInt(value) {
  return typeof value === "number" && Number.isFinite(value) ? integer.format(value) : null;
}

// Test USDC has 6 decimals; two by default, full precision on request.
export function fmtToken(units, { full = false } = {}) {
  if (typeof units !== "number" || !Number.isFinite(units)) return null;
  return (full ? six : two).format(units / 1e6).replace("-", MINUS);
}

export function fmtTokenDelta(units) {
  if (typeof units !== "number" || !Number.isFinite(units)) return null;
  const text = two.format(Math.abs(units) / 1e6);
  if (units > 0) return `+${text}`;
  if (units < 0) return `${MINUS}${text}`;
  return text;
}

export function fmtDuration(seconds) {
  if (typeof seconds !== "number" || !Number.isFinite(seconds) || seconds < 0) return null;
  const whole = Math.floor(seconds);
  const minutes = Math.floor(whole / 60);
  const rest = whole % 60;
  if (minutes === 0) return `${whole} s`;
  const hours = Math.floor(minutes / 60);
  if (hours === 0) return `${minutes} min ${rest} s`;
  return `${hours} h ${minutes % 60} min`;
}

export function fmtDecimal(value, digits = 1) {
  if (typeof value !== "number" || !Number.isFinite(value)) return null;
  return new Intl.NumberFormat(LOCALE, { minimumFractionDigits: digits, maximumFractionDigits: digits }).format(value);
}

function pad(number) {
  return String(number).padStart(2, "0");
}

// One absolute format everywhere: 2026-10-06 14:32:07 UTC.
export function fmtUtc(iso) {
  const time = typeof iso === "string" ? Date.parse(iso) : typeof iso === "number" ? iso : NaN;
  if (!Number.isFinite(time)) return null;
  const date = new Date(time);
  return `${date.getUTCFullYear()}-${pad(date.getUTCMonth() + 1)}-${pad(date.getUTCDate())} ` +
    `${pad(date.getUTCHours())}:${pad(date.getUTCMinutes())}:${pad(date.getUTCSeconds())} UTC`;
}

export function parseTime(iso) {
  const time = typeof iso === "string" ? Date.parse(iso) : NaN;
  return Number.isFinite(time) ? time : null;
}

// HashField truncation: hex 8+8, signatures 8+8, addresses 4+4.
export function truncate(value, kind) {
  if (typeof value !== "string") return "";
  const [head, tail] = kind === "address" ? [4, 4] : [8, 8];
  return value.length <= head + tail + 1 ? value : `${value.slice(0, head)}…${value.slice(-tail)}`;
}
