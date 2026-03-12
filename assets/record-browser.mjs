#!/usr/bin/env node
// Record browser demo GIFs for terminal-slide features
// Usage: node assets/record-browser.mjs
// Requires: playwright, ffmpeg, terminal-slide binary in PATH or target/release/

import { chromium } from "playwright";
import { spawn, execSync } from "child_process";
import { setTimeout as sleep } from "timers/promises";
import path from "path";
import { fileURLToPath } from "url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const ROOT = path.resolve(__dirname, "..");
const ASSETS = path.join(ROOT, "assets");
const BIN =
  process.env.TERMINAL_SLIDE ||
  path.join(ROOT, "target", "release", "terminal-slide");

const VP = { width: 1280, height: 800 };

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

function startServer(file, port) {
  const proc = spawn(BIN, [file, "--port", String(port)], {
    cwd: ROOT,
    stdio: "ignore",
  });
  return proc;
}

async function waitForServer(port, retries = 30) {
  for (let i = 0; i < retries; i++) {
    try {
      const res = await fetch(`http://localhost:${port}`);
      if (res.ok) return;
    } catch {}
    await sleep(300);
  }
  throw new Error(`Server on port ${port} did not start`);
}

function videoToGif(videoPath, gifPath, fps = 12, width = 720) {
  execSync(
    `ffmpeg -y -i "${videoPath}" -vf "fps=${fps},scale=${width}:-1:flags=lanczos,split[s0][s1];[s0]palettegen=max_colors=128[p];[s1][p]paletteuse=dither=bayer:bayer_scale=3" "${gifPath}"`,
    { stdio: "inherit" }
  );
}

// ---------------------------------------------------------------------------
// Recording functions
// ---------------------------------------------------------------------------

async function recordWebMode() {
  const port = 18001;
  const server = startServer("examples/demo.html", port);
  try {
    await waitForServer(port);
    const browser = await chromium.launch();
    const ctx = await browser.newContext({
      viewport: VP,
      recordVideo: { dir: ASSETS, size: VP },
    });
    const page = await ctx.newPage();
    await page.goto(`http://localhost:${port}`);
    await sleep(2000);

    // Navigate through slides
    for (let i = 0; i < 5; i++) {
      await page.keyboard.press("ArrowRight");
      await sleep(1500);
    }
    // Jump back to slide 1 using keyboard
    await page.keyboard.press("g");
    await sleep(1500);

    await ctx.close();
    await browser.close();

    const video = await page.video().path();
    videoToGif(video, path.join(ASSETS, "web-mode.gif"));
    console.log("✓ web-mode.gif");
  } finally {
    server.kill();
  }
}

async function recordGallery() {
  const port = 18002;
  const server = startServer("examples/gallery.html", port);
  try {
    await waitForServer(port);
    const browser = await chromium.launch();
    const ctx = await browser.newContext({
      viewport: VP,
      recordVideo: { dir: ASSETS, size: VP },
    });
    const page = await ctx.newPage();
    await page.goto(`http://localhost:${port}`);
    await sleep(2000);

    // Open gallery overlay
    await page.click("._ts-grid-btn");
    await sleep(2500);

    // Scroll down in gallery to show more cards
    await page.evaluate(() => {
      const overlay = document.querySelector("._ts-gallery-overlay");
      if (overlay) overlay.scrollBy({ top: 400, behavior: "smooth" });
    });
    await sleep(1500);

    // Click a slide card to jump
    const cards = await page.$$("._ts-gallery-card");
    if (cards.length > 4) await cards[4].click();
    await sleep(2000);

    // Open jump menu
    await page.click("._ts-counter-btn");
    await sleep(1500);

    // Click slide 2 from jump menu
    const items = await page.$$("._ts-jump-item");
    if (items.length > 1) await items[1].click();
    await sleep(1500);

    await ctx.close();
    await browser.close();

    const video = await page.video().path();
    videoToGif(video, path.join(ASSETS, "gallery.gif"));
    console.log("✓ gallery.gif");
  } finally {
    server.kill();
  }
}

async function recordTemplates() {
  const port = 18003;
  const server = startServer("templates/layouts.html", port);
  try {
    await waitForServer(port);
    const browser = await chromium.launch();
    const ctx = await browser.newContext({
      viewport: VP,
      recordVideo: { dir: ASSETS, size: VP },
    });
    const page = await ctx.newPage();
    await page.goto(`http://localhost:${port}`);
    await sleep(2000);

    // Navigate through layout templates
    for (let i = 0; i < 8; i++) {
      await page.keyboard.press("ArrowRight");
      await sleep(1800);
    }

    await ctx.close();
    await browser.close();

    const video = await page.video().path();
    videoToGif(video, path.join(ASSETS, "templates.gif"));
    console.log("✓ templates.gif");
  } finally {
    server.kill();
  }
}

async function recordThemes() {
  const port = 18004;
  const server = startServer("templates/theme-preview.html", port);
  try {
    await waitForServer(port);
    const browser = await chromium.launch();
    const ctx = await browser.newContext({
      viewport: VP,
      recordVideo: { dir: ASSETS, size: VP },
    });
    const page = await ctx.newPage();
    await page.goto(`http://localhost:${port}`);
    await sleep(2000);

    // Navigate through themes (each slide is a different theme)
    for (let i = 0; i < 5; i++) {
      await page.keyboard.press("ArrowRight");
      await sleep(2000);
    }

    // Go back to first
    await page.keyboard.press("g");
    await sleep(1500);

    await ctx.close();
    await browser.close();

    const video = await page.video().path();
    videoToGif(video, path.join(ASSETS, "themes.gif"));
    console.log("✓ themes.gif");
  } finally {
    server.kill();
  }
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

async function main() {
  console.log("Recording browser demos...\n");

  await recordWebMode();
  await recordGallery();
  await recordTemplates();
  await recordThemes();

  // Clean up webm files
  execSync(`rm -f ${ASSETS}/*.webm`);

  console.log("\nAll GIFs recorded!");
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
