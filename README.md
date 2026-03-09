# terminal-slide

Terminal-based slide presentation tool supporting Markdown and HTML.

## Features

- **Markdown mode** — Render slides as an interactive TUI in the terminal (ratatui + tachyonfx)
- **HTML mode** — Serve slides via a local HTTP server and open in the browser
- Smooth slide transition animations
- Syntax-highlighted code blocks (syntect)
- Keyboard navigation (arrow keys, vim-style, etc.)

## Install

```bash
cargo install --path .
```

## Usage

### Markdown (terminal)

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

---

## Code

\```rust
fn main() {
    println!("Hello!");
}
\```
```

### HTML (browser)

```bash
terminal-slide presentation.html
```

HTML files are served on `localhost:8234` and opened in the default browser. Use any CSS layout you want — flexbox, grid, absolute positioning.

See `examples/gallery.html` for a layout pattern gallery (2-column, 3-column, grid, free positioning).

HTML mode supports any JavaScript library via CDN — charts, animations, even Python execution in the browser. See `examples/interactive.html` for demos of Chart.js, CSS animations, canvas particles, and Pyodide.

## Keyboard Shortcuts

| Action         | Keys                   |
|----------------|------------------------|
| Next slide     | Right, l, j, n, Space  |
| Previous slide | Left, h, k, p          |
| First slide    | g                      |
| Last slide     | G                      |
| Quit           | q, Esc, Ctrl+C         |

## Examples

```bash
terminal-slide examples/demo.md       # TUI presentation
terminal-slide examples/demo.html     # Browser presentation
terminal-slide examples/gallery.html      # Layout pattern gallery
terminal-slide examples/interactive.html  # Interactive demo (charts, Python, animations)
```

## Built With

- [ratatui](https://github.com/ratatui/ratatui) — Terminal UI framework
- [tachyonfx](https://github.com/junkdog/tachyonfx) — Terminal effects & animations
- [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) — Markdown parser
- [syntect](https://github.com/trishume/syntect) — Syntax highlighting
- [tiny_http](https://github.com/tiny-http/tiny-http) — Lightweight HTTP server
