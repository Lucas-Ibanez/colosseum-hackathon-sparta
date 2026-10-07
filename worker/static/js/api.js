// The local worker's API (same origin). The token lives only in this module's
// memory: never in a URL, a log, storage or a cookie.

let token = null;
let clockOffset = null; // worker clock minus browser clock, from the HTTP Date header
const listeners = new Set();

export class ApiError extends Error {
  constructor(status, payload) {
    super(payload && payload.error ? `${status} ${payload.error}` : `HTTP ${status}`);
    this.status = status;
    this.payload = payload || {};
  }
}

export function setToken(value) {
  token = value || null;
}

export function hasToken() {
  return token !== null;
}

// Called with the ApiError when the worker refuses the token (401/403).
export function onTokenRefused(listener) {
  listeners.add(listener);
}

// Time on the worker's clock (Date header), so elapsed times are the worker's.
export function workerNow() {
  return Date.now() + (clockOffset || 0);
}

function noteClock(response) {
  const header = response.headers.get("Date");
  const serverTime = header ? Date.parse(header) : NaN;
  if (!Number.isFinite(serverTime)) return;
  const offset = serverTime - Date.now();
  // The header has one-second resolution; keep the first reading unless it drifts.
  if (clockOffset === null || Math.abs(offset - clockOffset) > 1500) clockOffset = offset;
}

export async function api(method, path, body) {
  if (!token) throw new ApiError(401, { error: "token_required" });
  const headers = { "X-VeriCode-Token": token };
  const init = { method, headers, cache: "no-store", credentials: "omit", redirect: "error", referrerPolicy: "no-referrer" };
  if (method === "POST") {
    headers["Content-Type"] = "application/json";
    init.body = JSON.stringify(body || {});
  }
  let response;
  try {
    response = await fetch(path, init);
  } catch (error) {
    throw new ApiError(0, { error: "unreachable", detail: String(error && error.message ? error.message : error) });
  }
  noteClock(response);
  let payload = null;
  try {
    payload = await response.json();
  } catch {
    payload = null;
  }
  if (!response.ok) {
    const error = new ApiError(response.status, payload);
    if (response.status === 401 || response.status === 403) {
      token = null;
      for (const listener of listeners) listener(error);
    }
    throw error;
  }
  return payload;
}

export const getHealth = () => api("GET", "/api/health");
export const getJobs = () => api("GET", "/api/jobs");
export const getJob = (jobId) => api("GET", `/api/jobs/${jobId}`);
export const getOp = (opId) => api("GET", `/api/ops/${opId}`);
export const postShow = (jobId) => api("POST", `/api/jobs/${jobId}/show`, {});
export const postCreate = (offset) => api("POST", "/api/jobs", { deadline_offset: offset });
export const postProve = (jobId, input, output) =>
  api("POST", `/api/jobs/${jobId}/prove`, { input, claimed_output: output });
export const postSettle = (jobId) => api("POST", `/api/jobs/${jobId}/settle`, {});
export const postRefund = (jobId) => api("POST", `/api/jobs/${jobId}/refund-timeout`, {});
export const postNegative = (jobId, kind, receiptJobId) =>
  api("POST", `/api/jobs/${jobId}/negative/${kind}`, kind === "escrow-6014" ? { receipt_job_id: receiptJobId } : {});
