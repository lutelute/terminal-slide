#!/usr/bin/env node
// Record category-specific GIFs: math, code, interactive
// Uses examples/math-algo.html, templates/gallery.html, examples/interactive.html

import { chromium } from "playwright";
import { spawn } from "child_process";
import { execSync } from "child_process";
import { setTimeout as sleep } from "timers/promises";
import path from "path";
import { fileURLToPath } from "url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const ROOT = path.resolve(__dirname, "..");
const ASSETS = path.join(ROOT, "assets");
const BIN = path.join(ROOT, "target", "release", "terminal-slide");
const VP = { width: 1280, height: 800 };

function startServer(file, port) {
  return spawn(BIN, [file, "--port", String(port)], {
    cwd: ROOT,
    stdio: "ignore",
  });
}

async function waitForServer(port, retries = 30) {
  for (let i = 0; i < retries; i++) {
    try {
      const res = await fetch(`http://localhost:${port}`);
      if (res.ok) return;
    } catch {}
    await sleep(300);
  }
  throw new Error(`Server on port ${port} not ready`);
}

function videoToGif(videoPath, gifPath, fps = 12, width = 720) {
  execSync(
    `ffmpeg -y -i "${videoPath}" -vf "fps=${fps},scale=${width}:-1:flags=lanczos,split[s0][s1];[s0]palettegen=max_colors=128[p];[s1][p]paletteuse=dither=bayer:bayer_scale=3" "${gifPath}"`,
    { stdio: "inherit" }
  );
}

// -------------------------------------------------------------------
// Math & Algorithms (math-algo.html): slides 1-9
// Show: formulas, proof, matrix, Big-O table, algorithm, flowchart
// -------------------------------------------------------------------
async function recordMath() {
  const port = 19001;
  const server = startServer("examples/math-algo.html", port);
  try {
    await waitForServer(port);
    const browser = await chromium.launch();
    const ctx = await browser.newContext({
      viewport: VP,
      recordVideo: { dir: ASSETS, size: VP },
    });
    const page = await ctx.newPage();
    await page.goto(`http://localhost:${port}`);
    await sleep(2500);

    // Navigate: Title → Formulas → Proof → Matrix → Big-O → Binary Search → Quicksort → Flowchart
    const slides = [1, 2, 3, 4, 5, 6, 7];
    for (const _ of slides) {
      await page.keyboard.press("ArrowRight");
      await sleep(2200);
    }

    await ctx.close();
    await browser.close();

    const video = await page.video().path();
    videoToGif(video, path.join(ASSETS, "math.gif"));
    console.log("✓ math.gif");
  } finally {
    server.kill();
  }
}

// -------------------------------------------------------------------
// Code (gallery.html): slides 8-9 are code slides
// Jump to syntax highlight slide and diff view slide
// -------------------------------------------------------------------
async function recordCode() {
  const port = 19002;
  const server = startServer("templates/gallery.html", port);
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

    // Jump to slide 8 (Syntax Highlight) via jump menu
    await page.click("._ts-counter-btn");
    await sleep(800);
    const items = await page.$$("._ts-jump-item");
    if (items.length > 7) await items[7].click(); // 0-indexed → slide 8
    await sleep(2500);

    // Next: Diff View (slide 9)
    await page.keyboard.press("ArrowRight");
    await sleep(2500);

    // Next: Python execution (slide 10)
    await page.keyboard.press("ArrowRight");
    await sleep(2500);

    // Next: Interactive I/O (slide 11)
    await page.keyboard.press("ArrowRight");
    await sleep(2500);

    await ctx.close();
    await browser.close();

    const video = await page.video().path();
    videoToGif(video, path.join(ASSETS, "code.gif"));
    console.log("✓ code.gif");
  } finally {
    server.kill();
  }
}

// -------------------------------------------------------------------
// Interactive (interactive.html): charts, animations, Python, particles
// -------------------------------------------------------------------
async function recordInteractive() {
  const port = 19003;
  const server = startServer("examples/interactive.html", port);
  try {
    await waitForServer(port);
    const browser = await chromium.launch();
    const ctx = await browser.newContext({
      viewport: VP,
      recordVideo: { dir: ASSETS, size: VP },
    });
    const page = await ctx.newPage();
    await page.goto(`http://localhost:${port}`);
    await sleep(2500);

    // Navigate: Title → Bar Chart → Line Chart → Doughnut/Radar → CSS Animations → Bar CSS → Counter → Typing → Python → Particles
    for (let i = 0; i < 9; i++) {
      await page.keyboard.press("ArrowRight");
      await sleep(2200);
    }

    await ctx.close();
    await browser.close();

    const video = await page.video().path();
    videoToGif(video, path.join(ASSETS, "interactive.gif"));
    console.log("✓ interactive.gif");
  } finally {
    server.kill();
  }
}

// -------------------------------------------------------------------

async function main() {
  console.log("Recording category GIFs...\n");

  await recordMath();
  await recordCode();
  await recordInteractive();

  execSync(`rm -f ${ASSETS}/*.webm`);
  console.log("\nAll category GIFs recorded!");
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
