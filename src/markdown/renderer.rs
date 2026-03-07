use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Padding, Paragraph, Widget, Wrap};
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

use crate::slide::{Slide, SlideElement, TextStyle};

/// Background color for code blocks (base16-ocean dark).
const CODE_BG: Color = Color::Rgb(43, 48, 59);

/// Holds preloaded syntax highlighting assets for reuse across slides.
///
/// Creating `SyntaxSet` and `ThemeSet` is expensive, so this struct should be
/// created once at startup and passed by reference to each `SlideWidget`.
pub struct SyntaxHighlighter {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
}

impl SyntaxHighlighter {
    /// Creates a new syntax highlighter with default syntax definitions and themes.
    pub fn new() -> Self {
        Self {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
        }
    }
}

impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

/// A ratatui widget that renders a single slide with formatted content.
///
/// Converts [`SlideElement`] variants into ratatui primitives:
/// - `Heading` -> Bold `Line` with level-based color (H1 brightest)
/// - `Paragraph` -> `Paragraph` with styled spans and word wrap
/// - `CodeBlock` -> Syntax-highlighted lines with distinct background
/// - `BulletList` -> Lines prefixed with bullet character
/// - `NumberedList` -> Lines prefixed with sequential numbers
/// - `HorizontalRule` -> Single-line separator
///
/// Content is rendered inside a bordered block with padding and
/// vertically centered within the terminal area.
pub struct SlideWidget<'a> {
    slide: &'a Slide,
    highlighter: &'a SyntaxHighlighter,
}

impl<'a> SlideWidget<'a> {
    /// Creates a new `SlideWidget` for the given slide.
    pub fn new(slide: &'a Slide, highlighter: &'a SyntaxHighlighter) -> Self {
        Self { slide, highlighter }
    }

    /// Builds all content lines from the slide elements.
    fn build_lines(&self, width: usize) -> Vec<Line<'static>> {
        let mut lines: Vec<Line<'static>> = Vec::new();

        for element in &self.slide.elements {
            match element {
                SlideElement::Heading(level, text) => {
                    let style = heading_style(*level);
                    lines.push(Line::from(Span::styled(text.clone(), style)));
                    lines.push(Line::default());
                }
                SlideElement::Paragraph(spans) => {
                    let ratatui_spans: Vec<Span<'static>> =
                        spans.iter().map(styled_span_to_ratatui).collect();
                    lines.push(Line::from(ratatui_spans));
                    lines.push(Line::default());
                }
                SlideElement::CodeBlock(lang, code) => {
                    lines.extend(self.highlight_code(lang.as_deref(), code, width));
                    lines.push(Line::default());
                }
                SlideElement::BulletList(items) => {
                    for item in items {
                        let mut spans: Vec<Span<'static>> =
                            vec![Span::styled("  \u{2022} ", Style::default().fg(Color::Cyan))];
                        spans.extend(item.iter().map(styled_span_to_ratatui));
                        lines.push(Line::from(spans));
                    }
                    lines.push(Line::default());
                }
                SlideElement::NumberedList(items) => {
                    for (i, item) in items.iter().enumerate() {
                        let prefix = format!("  {}. ", i + 1);
                        let mut spans: Vec<Span<'static>> =
                            vec![Span::styled(prefix, Style::default().fg(Color::Cyan))];
                        spans.extend(item.iter().map(styled_span_to_ratatui));
                        lines.push(Line::from(spans));
                    }
                    lines.push(Line::default());
                }
                SlideElement::HorizontalRule => {
                    let rule_width = width.min(60);
                    let rule = "\u{2500}".repeat(rule_width);
                    lines.push(Line::from(Span::styled(
                        rule,
                        Style::default().fg(Color::DarkGray),
                    )));
                    lines.push(Line::default());
                }
                SlideElement::BlankLine => {
                    lines.push(Line::default());
                }
            }
        }

        // Remove trailing blank line if present
        if lines.last().is_some_and(|l| l.width() == 0) {
            lines.pop();
        }

        lines
    }

    /// Highlights a code block using syntect and returns styled lines.
    ///
    /// Falls back to plain text rendering if the language is not recognized
    /// or if syntax highlighting fails for any line.
    fn highlight_code(
        &self,
        lang: Option<&str>,
        code: &str,
        width: usize,
    ) -> Vec<Line<'static>> {
        let syntax = lang
            .and_then(|l| self.highlighter.syntax_set.find_syntax_by_token(l))
            .unwrap_or_else(|| self.highlighter.syntax_set.find_syntax_plain_text());

        let theme = &self.highlighter.theme_set.themes["base16-ocean.dark"];
        let mut h = HighlightLines::new(syntax, theme);

        let mut lines = Vec::new();

        // Top padding line with background
        let pad_line = " ".repeat(width);
        lines.push(Line::from(Span::styled(
            pad_line.clone(),
            Style::default().bg(CODE_BG),
        )));

        // Language label if present
        if let Some(lang) = lang {
            let label = format!("  {lang}");
            let remaining = width.saturating_sub(label.len());
            let padded = format!("{label}{}", " ".repeat(remaining));
            lines.push(Line::from(Span::styled(
                padded,
                Style::default()
                    .fg(Color::DarkGray)
                    .bg(CODE_BG)
                    .add_modifier(Modifier::ITALIC),
            )));
        }

        // Highlighted code lines
        for line_str in code.lines() {
            let line_with_ending = format!("{line_str}\n");
            let ranges = h
                .highlight_line(&line_with_ending, &self.highlighter.syntax_set)
                .unwrap_or_default();

            let mut spans: Vec<Span<'static>> = Vec::new();

            // Indent code with 2 spaces
            spans.push(Span::styled("  ", Style::default().bg(CODE_BG)));
            let mut line_len: usize = 2;

            if ranges.is_empty() {
                // Fallback: render as plain text if highlighting failed
                spans.push(Span::styled(
                    line_str.to_string(),
                    Style::default().fg(Color::White).bg(CODE_BG),
                ));
                line_len += line_str.len();
            } else {
                for (style, text) in &ranges {
                    let text = text.trim_end_matches('\n');
                    if text.is_empty() {
                        continue;
                    }
                    let fg =
                        Color::Rgb(style.foreground.r, style.foreground.g, style.foreground.b);
                    line_len += text.len();
                    spans.push(Span::styled(
                        text.to_string(),
                        Style::default().fg(fg).bg(CODE_BG),
                    ));
                }
            }

            // Pad to full width for consistent background
            if line_len < width {
                spans.push(Span::styled(
                    " ".repeat(width - line_len),
                    Style::default().bg(CODE_BG),
                ));
            }

            lines.push(Line::from(spans));
        }

        // Bottom padding line with background
        lines.push(Line::from(Span::styled(
            pad_line,
            Style::default().bg(CODE_BG),
        )));

        lines
    }
}

impl Widget for SlideWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Create bordered block with padding
        let block = Block::bordered()
            .padding(Padding::new(2, 2, 1, 1))
            .border_style(Style::default().fg(Color::DarkGray));

        let inner_area = block.inner(area);
        block.render(area, buf);

        // Build content lines
        let lines = self.build_lines(inner_area.width as usize);

        // Calculate vertical centering offset
        let content_height = lines.len() as u16;
        let available_height = inner_area.height;
        let y_offset = if content_height < available_height {
            (available_height - content_height) / 2
        } else {
            0
        };

        // Create vertically centered area
        let render_area = Rect::new(
            inner_area.x,
            inner_area.y + y_offset,
            inner_area.width,
            available_height.saturating_sub(y_offset),
        );

        let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });
        paragraph.render(render_area, buf);
    }
}

/// Returns the style for a heading based on its level (1-6).
///
/// H1 is the brightest and most prominent, while H6 is the most subdued.
fn heading_style(level: u8) -> Style {
    let color = match level {
        1 => Color::Cyan,
        2 => Color::Green,
        3 => Color::Yellow,
        4 => Color::Magenta,
        5 => Color::Blue,
        _ => Color::White,
    };
    Style::default().fg(color).add_modifier(Modifier::BOLD)
}

/// Converts a [`StyledSpan`](crate::slide::StyledSpan) from the slide data model
/// into a ratatui [`Span`].
fn styled_span_to_ratatui(span: &crate::slide::StyledSpan) -> Span<'static> {
    let style = match span.style {
        TextStyle::Plain => Style::default(),
        TextStyle::Bold => Style::default().add_modifier(Modifier::BOLD),
        TextStyle::Italic => Style::default().add_modifier(Modifier::ITALIC),
        TextStyle::BoldItalic => Style::default()
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::ITALIC),
        TextStyle::Code => Style::default()
            .fg(Color::Yellow)
            .bg(Color::Rgb(50, 50, 50)),
    };

    Span::styled(span.text.clone(), style)
}
