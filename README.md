# terminal-slide

Terminal-based slide presentation tool supporting Markdown and HTML.

Markdown slides run in the terminal (TUI). HTML slides are served via a local HTTP server and opened in the browser with auto-injected navigation UI.

## Install

```bash
cargo install --path .
```

Requires Rust toolchain. After install, `terminal-slide` is available as a CLI command.

## Quick Start

```bash
# Present Markdown slides in the terminal
terminal-slide slides.md

# Present HTML slides in the browser
terminal-slide presentation.html
```

That's it. No config files, no build step.

## Two Modes

### Markdown Mode (terminal)

Best for quick internal talks and technical LTs. Runs entirely in the terminal.

```bash
terminal-slide slides.md
```

Write slides separated by `---`:

```markdown
# Title Slide

Welcome to my talk

---

## Second Slide

- Point 1
- Point 2
- **Bold** and *italic* supported

---

## Code

\```rust
fn main() {
    println!("Hello!");
}
\```
```

Supported elements: headings (H1-H6), bold, italic, inline code, code blocks with syntax highlighting, bullet lists, numbered lists, horizontal rules.

### HTML Mode (browser)

Best for rich presentations with custom layouts, charts, animations, and interactive content.

```bash
terminal-slide presentation.html
```

The server starts on `localhost:8234` (configurable with `--port`) and opens the default browser. A navigation toolbar is automatically injected into every HTML presentation.

HTML mode supports:
- Any CSS layout (flexbox, grid, absolute positioning)
- JavaScript libraries via CDN (Chart.js, KaTeX, Mermaid, D3, Three.js, etc.)
- Interactive elements (buttons, forms, live Python via Pyodide)
- CSS and JS animations

## CLI Reference

```
terminal-slide [OPTIONS] <FILE>
```

### Arguments

| Argument | Description |
|----------|-------------|
| `<FILE>` | Path to `.md`, `.html`, or `.htm` file |

### Options

| Option | Description | Default |
|--------|-------------|---------|
| `--port <PORT>` | Port for the local HTTP server (HTML mode) | `8234` |
| `--export <FORMAT>` | Export instead of presenting (`pdf`, `pptx`, `md`) | — |
| `-o, --output <PATH>` | Output file path for export | auto-generated |
| `-h, --help` | Print help | — |
| `-V, --version` | Print version | — |

### CLI Examples

```bash
# Present
terminal-slide talk.md                  # Terminal TUI
terminal-slide slides.html              # Browser
terminal-slide slides.html --port 9000  # Custom port

# Export
terminal-slide talk.md --export pdf             # MD -> PDF (pandoc beamer)
terminal-slide talk.md --export pptx            # MD -> PPTX (pandoc)
terminal-slide slides.html --export pdf         # HTML -> PDF (headless Chrome)
terminal-slide slides.html --export pptx        # HTML -> PPTX (pandoc)
terminal-slide slides.html --export md          # HTML -> Markdown (pandoc)
terminal-slide talk.md --export pdf -o out.pdf  # Custom output path
```

## Keyboard Shortcuts

Works in both Markdown (terminal) and HTML (browser) modes.

| Action | Keys |
|--------|------|
| Next slide | `Right` `l` `j` `n` `Space` |
| Previous slide | `Left` `h` `k` `p` |
| First slide | `g` |
| Last slide | `G` |
| Quit | `q` `Esc` `Ctrl+C` |

## Navigation UI (HTML Mode)

HTML presentations get an auto-injected toolbar at the bottom-left with these controls:

| Button | Function |
|--------|----------|
| `1 / N` | **Slide jump** — click to open a numbered grid, click any number to jump |
| `▦` | **Gallery** — full-screen thumbnail view of all slides, click to jump |
| `⏸` | **Pause** — freeze all CSS animations and transitions. Click again to resume |
| `⏭` | **Skip** — disable all animations entirely. Click again to restore |
| `⇓` | **Export** — download as PDF, PPTX, or Markdown directly from the browser |

## Export

Export requires external tools:

| Conversion | Requires |
|------------|----------|
| MD -> PDF | [pandoc](https://pandoc.org/installing.html) + LaTeX (`brew install pandoc basictex`) |
| MD -> PPTX | [pandoc](https://pandoc.org/installing.html) (`brew install pandoc`) |
| HTML -> PDF | Chrome or Chromium (set `CHROME_PATH` env var if not auto-detected) |
| HTML -> MD | [pandoc](https://pandoc.org/installing.html) |
| HTML -> PPTX | [pandoc](https://pandoc.org/installing.html) |

Export is available both from the CLI (`--export`) and from the browser toolbar (⇓ button).

## Templates

Ready-to-use templates are in the `templates/` directory.

### Starter Templates

Copy and start editing:

```bash
cp templates/starter.html my-talk.html       # Dark theme
cp templates/starter-light.html my-talk.html  # Light theme
terminal-slide my-talk.html
```

Each starter includes 3 slides (Title, Content, Thank You) with all base CSS and keyboard navigation.

### Layout Patterns

`templates/layouts.html` contains 14 copy-paste layout patterns:

| Layout | Description |
|--------|-------------|
| Title | Centered title + subtitle + author |
| Section Divider | Big heading for section breaks |
| Text + Bullets | Standard content slide |
| Two Column | Equal 1:1 flex layout |
| Wide + Narrow | 2:1 ratio with sidebar |
| Three Column | Three equal cards |
| Code Showcase | Large syntax-highlighted code block |
| Image + Text | Side-by-side media layout |
| Quote | Centered blockquote with attribution |
| Comparison | Before/After with colored headers |
| Timeline | Vertical timeline with accent dots |
| Stats | Big numbers with labels |
| Grid Cards | 3x2 CSS grid |
| Hero + Details | Full-width banner + bottom row |

Preview them: `terminal-slide templates/layouts.html`

### Color Themes

Six CSS theme files in `templates/themes/`:

| Theme | File | Style |
|-------|------|-------|
| Dark | `dark.css` | Dark background, cyan accent (default) |
| Light | `light.css` | White background, blue accent |
| Corporate | `corporate.css` | Navy, professional |
| Neon | `neon.css` | Black, green + magenta glow |
| Paper | `paper.css` | Warm sepia, serif typography |
| Minimal | `minimal.css` | Black and white, clean |

All themes use the same CSS custom properties (`--ts-bg`, `--ts-text`, `--ts-accent`, etc.) so they're swappable. To use a theme, copy its properties into your HTML's `<style>` block or link the CSS file.

## Examples

```bash
terminal-slide examples/demo.md           # Markdown TUI demo
terminal-slide examples/demo.html         # HTML browser demo
terminal-slide examples/gallery.html      # Layout pattern gallery
terminal-slide examples/interactive.html  # Charts, animations, Python
terminal-slide examples/math-algo.html    # KaTeX math + Mermaid diagrams
terminal-slide templates/layouts.html     # 14 layout templates
```

### What Each Example Demonstrates

**`demo.md`** / **`demo.html`** — Basic features: headings, bullets, code blocks, text formatting, keyboard shortcuts table.

**`gallery.html`** — Layout patterns: 2-column, 3-column, 2x2 grid, free grid with row/column spans, hero sections, absolute positioning with overlapping elements.

**`interactive.html`** — Dynamic content: Chart.js bar/line/doughnut/radar charts with live data updates, CSS animations (bounce, spin, pulse), animated bar charts, counter animations, typing effects, Pyodide Python execution, Canvas particle animation.

**`math-algo.html`** — Academic content: KaTeX-rendered formulas (quadratic formula, Euler's identity, matrix operations), algorithm pseudocode (binary search, quicksort), Big-O complexity comparison table, Mermaid flowcharts and sequence diagrams.

## Use Cases

**Quick internal talk** — Write a `.md` file, run `terminal-slide talk.md`, present from the terminal. No browser, no setup.

**Technical LT / lightning talk** — Markdown mode with syntax-highlighted code blocks. Stay in the terminal where your audience expects you.

**Conference presentation** — Use HTML mode with custom layouts, Chart.js graphs, and KaTeX math. Export to PDF for backup.

**Lecture / tutorial** — Math formulas via KaTeX, algorithm flowcharts via Mermaid, live Python execution via Pyodide.

**Team demo** — Interactive charts with live data, split layouts for before/after comparisons. Export to PPTX for stakeholders who want slides.

**Documentation walkthrough** — Code showcase layouts with syntax highlighting. Present directly, then export to PDF for archival.

## Markdown vs HTML

| | Markdown (`.md`) | HTML (`.html`) |
|---|---|---|
| Display | Terminal (TUI) | Browser |
| Layout | Single column, centered | Fully customizable (CSS) |
| Code | Syntax highlighted | Syntax highlighted |
| Charts | No | Yes (Chart.js, D3, etc.) |
| Math | No | Yes (KaTeX) |
| Diagrams | No | Yes (Mermaid) |
| Animations | Slide transitions only | CSS/JS, pause/skip controls |
| Interactive | No | Yes (JS, Pyodide, etc.) |
| Speed to create | Fast | Moderate |
| Best for | Quick talks, internal | Rich presentations, external |

## Project Structure

```
src/
  main.rs           # Entry point, CLI routing
  cli.rs            # CLI argument parsing (clap)
  export.rs         # Export to PDF/PPTX/MD
  slide.rs          # Slide data model
  markdown/
    parser.rs       # Markdown -> Slide elements
    renderer.rs     # Slide -> ratatui widgets
  html/
    server.rs       # HTTP server with nav injection
    nav_snippet.rs  # Injected navigation UI (JS+CSS)
  tui/
    terminal.rs     # Terminal setup/teardown
    event.rs        # Keyboard event handling
    transitions.rs  # tachyonfx slide animations
  app.rs            # Application state
examples/           # Demo presentations
templates/          # Starter templates and themes
```

## Built With

- [ratatui](https://github.com/ratatui/ratatui) — Terminal UI framework
- [tachyonfx](https://github.com/junkdog/tachyonfx) — Terminal effects and animations
- [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) — Markdown parser
- [syntect](https://github.com/trishume/syntect) — Syntax highlighting
- [tiny_http](https://github.com/tiny-http/tiny-http) — Lightweight HTTP server
- [clap](https://github.com/clap-rs/clap) — CLI argument parsing

## License

MIT
