// Copies the fixed table static-assets.json into worker/static/ (development tool; never served).
//
//   node sync-assets.mjs          copy each source to its destination; every SHA-256 must match the table
//   node sync-assets.mjs --pin    first copy only: record the SHA-256 of entries that have none
//   node sync-assets.mjs --check  copy nothing; destinations and sources must match the table
//
// Sources: `brand/...` from the repository root, `lucide-static/...` from node_modules
// (ISC). No directory listing, no globbing: only the table.

import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, normalize } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const repo = join(here, "..", "..");
const staticDir = join(here, "..", "static");
const tablePath = join(here, "static-assets.json");
const table = JSON.parse(readFileSync(tablePath, "utf8"));
const pin = process.argv.includes("--pin");
const check = process.argv.includes("--check");

const sha256 = (bytes) => createHash("sha256").update(bytes).digest("hex");

function sourcePath(source) {
  if (source.startsWith("brand/")) return join(repo, source);
  if (source.startsWith("lucide-static/")) return join(here, "node_modules", source);
  throw new Error(`unknown source root: ${source}`);
}

function inside(base, relativePath) {
  const full = normalize(join(base, relativePath));
  if (!full.startsWith(base + "/")) throw new Error(`path escapes ${base}: ${relativePath}`);
  return full;
}

let changed = false;
let failures = 0;
for (const entry of table.files) {
  const dest = inside(staticDir, entry.dest);
  const src = sourcePath(entry.source);
  const bytes = readFileSync(src);
  const digest = sha256(bytes);
  if (!entry.sha256) {
    if (!pin) throw new Error(`${entry.dest}: no pinned SHA-256; run with --pin once`);
    entry.sha256 = digest;
    changed = true;
  }
  if (digest !== entry.sha256) {
    console.error(`MISMATCH source ${entry.source}: ${digest} != table ${entry.sha256}`);
    failures++;
    continue;
  }
  if (check) {
    const current = existsSync(dest) ? sha256(readFileSync(dest)) : "missing";
    if (current !== entry.sha256) {
      console.error(`MISMATCH dest ${entry.dest}: ${current} != table ${entry.sha256}`);
      failures++;
    }
    continue;
  }
  mkdirSync(dirname(dest), { recursive: true });
  writeFileSync(dest, bytes);
  if (sha256(readFileSync(dest)) !== entry.sha256) {
    console.error(`MISMATCH after copy ${entry.dest}`);
    failures++;
  }
}
if (changed) writeFileSync(tablePath, JSON.stringify(table, null, 2) + "\n");
if (failures) {
  console.error(`${failures} mismatch(es)`);
  process.exit(1);
}
console.log(`${table.files.length} assets ${check ? "match the table" : "copied and verified"}`);
