# terminal-slide

A terminal-based slide presentation tool

Navigate with **arrow keys**, `j`/`k`, or `n`/`p`

Press `q` to quit

---

## Features

- Render **Markdown** slides in the terminal
- Present *HTML* slides in the browser
- Smooth slide transition animations
- Syntax-highlighted code blocks
- Progress indicator in the footer

---

## Code Examples

Rust:

```rust
fn main() {
    let slides = parse_presentation("demo.md");
    for slide in &slides {
        render(slide);
    }
}
```

Python:

```python
def fibonacci(n):
    a, b = 0, 1
    for _ in range(n):
        a, b = b, a + b
    return a
```

---

### Getting Started

1. Install with `cargo install --path .`
2. Create a `.md` file with slides separated by `---`
3. Run `terminal-slide your-slides.md`
4. Present with ***confidence***

---

### Text Formatting

You can use **bold text** for emphasis, *italic text* for subtle highlights,
and `inline code` for technical terms.

Combine them for ***bold italic*** when you really need attention.

---

### Keyboard Shortcuts

| Action          | Keys                    |
|-----------------|-------------------------|
| Next slide      | Right, l, j, n, Space   |
| Previous slide  | Left, h, k, p           |
| First slide     | g                       |
| Last slide      | G                       |
| Quit            | q, Esc, Ctrl+C          |

---

## Thank You

Built with **Rust**, *ratatui*, and `tachyonfx`

Happy presenting!
