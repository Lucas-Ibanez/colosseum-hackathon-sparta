// Generates worker/static/css/tokens.css from DESIGN.md (development tool; never served).
//
//   node gen-tokens.mjs           write ../static/css/tokens.css
//   node gen-tokens.mjs --check   regenerate in memory and compare byte for byte (exit 1 on drift)
//
// Layers, all read from DESIGN.md:
//   1. `design.md export --format css-vars --prefix hive`, verbatim (colors, spacing, radii);
//   2. typography and component dimensions from the model resolved by the package's linter;
//   3. the light/dark roles of "Mapa de papéis" (Colors);
//   4. values that DESIGN.md states in prose (elevation, scrim, interaction mixes, motion,
//      reading width, icon and PartyMark sizes, table row height, focus offset), each with
//      the section it comes from.
// Components read only the role and type variables; nothing here invents a value.

import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { fileURLToPath } from "node:url";
import { lint } from "@google/design.md/linter";

const here = dirname(fileURLToPath(import.meta.url));
const repo = join(here, "..", "..");
const designPath = join(repo, "DESIGN.md");
const outPath = join(here, "..", "static", "css", "tokens.css");
const cli = join(here, "node_modules", "@google", "design.md", "dist", "index.js");

const FAMILIES = {
  Manrope: ['--hive-font-sans', '"Manrope", sans-serif'],
  "IBM Plex Mono": ['--hive-font-mono', '"IBM Plex Mono", monospace'],
};

// "Mapa de papéis" (DESIGN.md, Colors): role -> [light color, dark color].
const ROLES = [
  ["bg-page", "neutral", "dark-background"],
  ["bg-nav", "primary", "dark-nav"],
  ["bg-topbar", "surface", "dark-topbar"],
  ["bg-panel", "surface", "dark-panel"],
  ["bg-control", "surface", "dark-control"],
  // The map allows dark-background or dark-control; dark-background is the one that
  // sits below dark-panel.
  ["bg-sunken", "neutral", "dark-background"],
  ["bg-selected", "surface-variant", "dark-selected"],
  ["bg-chip", "surface-variant", "dark-control"],
  ["text-primary", "primary", "dark-on-surface"],
  ["text-secondary", "secondary", "dark-on-surface-muted"],
  ["text-on-nav", "surface", "dark-on-surface"],
  ["text-on-accent", "on-tertiary", "dark-on-tertiary"],
  ["accent", "tertiary", "tertiary"],
  ["border-subtle", "outline", "dark-outline"],
  ["border-strong", "outline-strong", "dark-outline-strong"],
  ["focus", "focus-ring", "dark-focus-ring"],
  ["link", "primary", "tertiary"],
  ["status-success", "success", "dark-success"],
  ["status-success-bg", "success-container", "dark-success-container"],
  ["status-danger", "danger", "dark-danger"],
  ["status-danger-bg", "danger-container", "dark-danger-container"],
  ["status-caution", "caution", "dark-caution"],
  ["status-caution-bg", "caution-container", "dark-caution-container"],
  ["status-pending", "pending", "dark-pending"],
  ["status-pending-bg", "pending-container", "dark-pending-container"],
];

// Values that DESIGN.md gives in prose: [variable, light, dark or null, source section].
const PROSE = [
  ["shadow-2", "0 8px 24px rgba(57, 45, 64, 0.12)", "0 8px 24px rgba(0, 0, 0, 0.32)", "Elevation & Depth, nível 2"],
  ["scrim", "rgba(57, 45, 64, 0.48)", null, "Elevation & Depth, scrim de modal"],
  ["mix-hover", "6%", null, "Motion and interaction states, hover"],
  ["mix-pressed", "12%", null, "Motion and interaction states, pressionado"],
  ["mix-accent-hover", "88%", null, "Colors, regras de uso do âmbar (hover do primário)"],
  ["mix-accent-pressed", "78%", null, "Colors, regras de uso do âmbar (pressionado)"],
  ["motion-fast", "120ms", null, "Motion: hover, pressionado, foco"],
  ["motion-base", "200ms", null, "Motion: expandir/recolher, toast, troca de estado"],
  ["ease", "ease-out", null, "Motion: curva"],
  ["measure", "68ch", null, "Typography, regras: largura de leitura"],
  ["focus-offset", "2px", null, "Components, campos: anel afastado 2 px"],
  ["row-height", "48px", null, "Components, tabelas: linhas de 48 px"],
  ["icon-sm", "16px", null, "Shapes, ícones: 16 px"],
  ["icon-md", "20px", null, "Shapes, ícones: 20 px"],
  ["icon-stroke", "1.5px", null, "Shapes, ícones: traço de 1,5 px"],
  ["partymark-sm", "24px", null, "Shapes, hexágono: 24 px"],
  ["partymark-md", "32px", null, "Shapes, hexágono: 32 px"],
  ["spin-period", "calc(var(--hive-motion-base) * 5)", null, "Motion: indicador de atividade em Proving (múltiplo do tempo base)"],
  ["brand-signature-min", "128px", null, "Brand assets (brand/MANIFEST.md): assinatura horizontal, largura mínima"],
];

function dim(value) {
  if (value && typeof value === "object" && value.type === "dimension") return `${value.value}${value.unit}`;
  if (typeof value === "string") return value;
  return null;
}

function build() {
  const source = readFileSync(designPath);
  const digest = createHash("sha256").update(source).digest("hex");
  const exported = execFileSync(process.execPath, [cli, "export", "--format", "css-vars", "--prefix", "hive", designPath], {
    encoding: "utf8",
  });
  const report = lint(source.toString("utf8"));
  if (report.summary.errors) throw new Error(`DESIGN.md has ${report.summary.errors} lint errors`);
  const ds = report.designSystem;
  const out = [];
  out.push("/* GERADO por worker/ui-tools/gen-tokens.mjs a partir do DESIGN.md. Não editar à mão.");
  out.push(` * DESIGN.md sha256=${digest}`);
  out.push(" * Camadas: 1 export css-vars; 2 tipografia e componentes; 3 papéis claro/escuro; 4 prosa do DESIGN.md. */");
  out.push("");
  out.push("/* Camada 1: design.md export --format css-vars --prefix hive */");
  out.push(exported.trimEnd());
  out.push("");

  out.push("/* Camada 2: tipografia (Typography) e dimensões dos componentes (Components) */");
  out.push(":root {");
  for (const [, [name, stack]] of Object.entries(FAMILIES)) out.push(`  ${name}: ${stack};`);
  for (const [name, type] of ds.typography) {
    const family = FAMILIES[type.fontFamily];
    if (!family) throw new Error(`typography ${name}: unknown family ${type.fontFamily}`);
    out.push(`  --hive-type-${name}: ${type.fontWeight} ${dim(type.fontSize)}/${dim(type.lineHeight)} var(${family[0]});`);
    out.push(`  --hive-type-${name}-tracking: ${type.letterSpacing ? dim(type.letterSpacing) : "normal"};`);
    out.push(`  --hive-type-${name}-numeric: ${type.fontFeature === "tnum" ? "tabular-nums" : "normal"};`);
  }
  for (const [name, component] of ds.components) {
    if (name.endsWith("-dark")) continue; // the dark variants repeat the same dimensions
    for (const prop of ["rounded", "padding", "height", "width"]) {
      const props = component.properties;
      const value = props instanceof Map ? props.get(prop) : props[prop];
      if (value === undefined) continue;
      const text = dim(value);
      if (text === null) throw new Error(`component ${name}.${prop}: unresolved`);
      out.push(`  --hive-comp-${name}-${prop}: ${text};`);
    }
  }
  out.push("}");
  out.push("");

  const color = (name) => {
    if (!ds.colors.has(name)) throw new Error(`role map names unknown color ${name}`);
    return `var(--hive-color-${name})`;
  };
  out.push("/* Camada 3: papéis (Colors, Mapa de papéis). Claro é o canônico; escuro preparado. */");
  out.push(":root {");
  out.push("  color-scheme: light;");
  for (const [role, light] of ROLES) out.push(`  --hive-${role}: ${color(light)};`);
  out.push("}");
  out.push('[data-theme="dark"] {');
  out.push("  color-scheme: dark;");
  for (const [role, , dark] of ROLES) out.push(`  --hive-${role}: ${color(dark)};`);
  out.push("}");
  out.push("");

  out.push("/* Camada 4: valores dados na prosa do DESIGN.md (seção de origem ao lado) */");
  out.push(":root {");
  for (const [name, light, , section] of PROSE) out.push(`  --hive-${name}: ${light}; /* ${section} */`);
  out.push("}");
  out.push('[data-theme="dark"] {');
  for (const [name, , dark, section] of PROSE) if (dark) out.push(`  --hive-${name}: ${dark}; /* ${section}, escuro */`);
  out.push("}");
  return out.join("\n") + "\n";
}

const text = build();
if (process.argv.includes("--check")) {
  let current = "";
  try {
    current = readFileSync(outPath, "utf8");
  } catch {
    current = "";
  }
  if (current !== text) {
    console.error(`tokens drift: ${relative(repo, outPath)} differs from what DESIGN.md generates; run npm run gen:tokens`);
    process.exit(1);
  }
  console.log(`tokens ok: ${relative(repo, outPath)} equals the DESIGN.md output`);
} else {
  writeFileSync(outPath, text);
  console.log(`wrote ${relative(repo, outPath)} (${text.length} bytes)`);
}
