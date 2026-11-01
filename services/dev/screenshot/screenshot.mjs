// Capture screenshots of the dev stack, see `just screenshot --help`.
import { mkdir } from "node:fs/promises";
import path from "node:path";
import { parseArgs } from "node:util";
import { chromium } from "playwright";

const USAGE = `Usage: just screenshot [options] <path>...

Capture pages of the dev stack (default host: portal.localhost).

Options:
  --host <host>      Host to capture (default: portal.localhost)
  --dark             Use the dark color scheme
  --mobile           Use a phone viewport (390x844) instead of 1280x800
  --full             Capture the full page instead of the viewport
  --click <selector> Click an element before capturing (repeatable, in order).
                     Playwright selectors, e.g. 'text=Concerts' or
                     '[aria-label="Modifier le concert"] >> nth=0'
  --wait <ms>        Extra delay before capturing (default: 500)
  --name <name>      Output file name (single path only)
  -h, --help         Show this help

Examples:
  just screenshot /
  just screenshot --dark --mobile / /locations
  just screenshot --click '[aria-label="Modifier le concert"] >> nth=0' /
`;

const { values: opts, positionals: paths } = parseArgs({
  allowPositionals: true,
  options: {
    host: { type: "string", default: "portal.localhost" },
    dark: { type: "boolean", default: false },
    mobile: { type: "boolean", default: false },
    full: { type: "boolean", default: false },
    click: { type: "string", multiple: true, default: [] },
    wait: { type: "string", default: "500" },
    name: { type: "string" },
    help: { type: "boolean", short: "h", default: false },
  },
});

if (opts.help || paths.length === 0) {
  console.log(USAGE);
  process.exit(opts.help ? 0 : 1);
}
if (opts.name && paths.length > 1) {
  console.error("--name can only be used with a single path");
  process.exit(1);
}

const OUT_DIR = "/out";
// Where OUT_DIR is mounted on the host, to print usable paths.
const HOST_OUT_DIR = process.env.SCREENSHOT_HOST_DIR ?? OUT_DIR;

const fileName = (pagePath) => {
  if (opts.name) return opts.name.endsWith(".png") ? opts.name : `${opts.name}.png`;
  const slug =
    pagePath
      .replace(/^\/+|\/+$/g, "")
      .replace(/[^a-zA-Z0-9]+/g, "-")
      .toLowerCase() || "index";
  const suffix = [
    opts.mobile && "mobile",
    opts.dark && "dark",
    opts.click.length > 0 && "clicked",
  ]
    .filter(Boolean)
    .join("-");
  return `${opts.host.split(".")[0]}-${slug}${suffix ? `-${suffix}` : ""}.png`;
};

await mkdir(OUT_DIR, { recursive: true });

const browser = await chromium.launch({
  // Chromium always resolves *.localhost to loopback; send it to Traefik
  // instead, which routes it exactly like on the host.
  args: ["--host-resolver-rules=MAP *.localhost traefik, MAP localhost traefik"],
});
const context = await browser.newContext({
  ignoreHTTPSErrors: true, // mkcert CA is not installed in this container
  colorScheme: opts.dark ? "dark" : "light",
  viewport: opts.mobile ? { width: 390, height: 844 } : { width: 1280, height: 800 },
  deviceScaleFactor: opts.mobile ? 2 : 1,
  isMobile: opts.mobile,
  hasTouch: opts.mobile,
  locale: "fr-FR",
  timezoneId: "Europe/Paris",
});

let failed = false;

for (const pagePath of paths) {
  const url = `https://${opts.host}${pagePath.startsWith("/") ? "" : "/"}${pagePath}`;
  const page = await context.newPage();
  // Problem first line → occurrences, to keep the report short.
  const problems = new Map();
  const report = (problem) => {
    const line = problem.split("\n")[0].trim();
    problems.set(line, (problems.get(line) ?? 0) + 1);
  };
  page.on("console", (msg) => {
    if (msg.type() === "error" || msg.type() === "warning") {
      report(`console.${msg.type()}: ${msg.text()}`);
    }
  });
  page.on("pageerror", (err) => report(`page error: ${err.message}`));
  page.on("requestfailed", (req) =>
    report(`request failed: ${req.method()} ${req.url()} (${req.failure()?.errorText})`),
  );
  page.on("response", (res) => {
    if (res.status() >= 400) {
      report(`HTTP ${res.status()}: ${res.request().method()} ${res.url()}`);
    }
  });

  try {
    await page.goto(url, { waitUntil: "networkidle", timeout: 60_000 });
    for (const selector of opts.click) {
      await page.locator(selector).first().click({ timeout: 10_000 });
      await page.waitForLoadState("networkidle");
    }
    // Hide the Nuxt devtools badge, it overlaps the page content.
    await page.addStyleTag({
      content: "#nuxt-devtools-container { display: none !important; }",
    });
    await page.evaluate(() => document.fonts.ready);
    await page.waitForTimeout(Number(opts.wait));

    const file = fileName(pagePath);
    await page.screenshot({ path: path.join(OUT_DIR, file), fullPage: opts.full });
    console.log(`✔ ${url} → ${path.join(HOST_OUT_DIR, file)}`);
  } catch (err) {
    failed = true;
    console.error(`✘ ${url}: ${err.message.split("\n")[0]}`);
  }

  for (const [problem, count] of problems) {
    console.log(`  ⚠ ${count > 1 ? `(x${count}) ` : ""}${problem}`);
  }
  await page.close();
}

await browser.close();
process.exit(failed ? 1 : 0);
