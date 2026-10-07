// Screenshots of the Hive interface for the visual check DESIGN.md asks for (development
// tool; never served). Reads only the screen; it never clicks an action that writes.
//
//   VU_WORKER_TOKEN=<token> node capture.mjs <out-dir> name=#/route [name=#/route ...]
//     [--dark] [--reduced-motion] [--blocks] [--width=1440] [--height=900] [--wait=1500]
//
// The token comes from the environment and is typed into the password field; it is never
// printed. Console errors, page errors and CSP violations are reported (exit 1 if any).

import { chromium } from "playwright";
import { mkdirSync } from "node:fs";
import { join } from "node:path";

const args = process.argv.slice(2);
const outDir = args.find((arg) => !arg.startsWith("--") && !arg.includes("="));
const shots = args.filter((arg) => !arg.startsWith("--") && arg.includes("=")).map((arg) => arg.split(/=(.*)/s).slice(0, 2));
const flag = (name) => args.includes(`--${name}`);
const option = (name, fallback) => Number((args.find((arg) => arg.startsWith(`--${name}=`)) || "").split("=")[1] || fallback);
const url = process.env.VU_UI_URL || "http://127.0.0.1:8710/ui/";
const token = process.env.VU_WORKER_TOKEN;
if (!outDir || !shots.length || !token) {
  console.error("usage: VU_WORKER_TOKEN=... node capture.mjs <out-dir> name=#/route ... [--dark] [--reduced-motion]");
  process.exit(2);
}
mkdirSync(outDir, { recursive: true });

const browser = await chromium.launch();
const context = await browser.newContext({
  viewport: { width: option("width", 1440), height: option("height", 900) },
  reducedMotion: flag("reduced-motion") ? "reduce" : "no-preference",
  locale: "pt-BR",
});
const page = await context.newPage();
const problems = [];
page.on("console", (message) => {
  if (["error", "warning"].includes(message.type())) problems.push(`console.${message.type()}: ${message.text()}`);
});
page.on("pageerror", (error) => problems.push(`pageerror: ${error.message}`));
await page.goto(url);
await page.fill("#token", token);
await page.keyboard.press("Enter");
await page.waitForSelector("#token", { state: "detached" });
await page.waitForSelector(".page__title");
if (flag("dark")) await page.evaluate(() => (document.documentElement.dataset.theme = "dark"));
for (const [name, hash] of shots) {
  await page.evaluate((target) => (location.hash = target), hash);
  await page.waitForTimeout(option("wait", 1500));
  await page.waitForLoadState("networkidle");
  const suffix = flag("dark") ? "-dark" : "";
  const file = join(outDir, `${name}${suffix}.png`);
  await page.screenshot({ path: file, fullPage: true });
  console.log(`shot ${file}`);
  if (flag("blocks")) {
    // One image per block of the page, for a close reading of each component.
    // Fixed bars would overlay the element images; hidden by CSSOM (allowed by the CSP).
    await page.evaluate(() => document.querySelectorAll(".status-bar, .top-bar").forEach((bar) => (bar.style.visibility = "hidden")));
    const blocks = await page.$$(".content > .page > *");
    for (let index = 0; index < blocks.length; index++) {
      const part = join(outDir, `${name}${suffix}-${String(index).padStart(2, "0")}.png`);
      await blocks[index].screenshot({ path: part });
      console.log(`block ${part}`);
    }
    await page.evaluate(() => document.querySelectorAll(".status-bar, .top-bar").forEach((bar) => (bar.style.visibility = "")));
  }
}
await browser.close();
for (const problem of problems) console.log(problem);
process.exit(problems.length ? 1 : 0);
