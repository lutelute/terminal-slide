use std::fs;
use std::path::Path;
use std::time::Instant;

use anyhow::{bail, Context, Result};
use clap::Parser;
use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

mod app;
mod cli;
mod export;
mod html;
mod markdown;
mod slide;
mod tui;

use app::App;
use cli::{detect_format, Cli, PresentationFormat};
use markdown::{parse_presentation, SlideWidget, SyntaxHighlighter};

fn main() -> Result<()> {
    let args = Cli::parse();

    // Check format first so unsupported extensions get a specific error
    // even if the file doesn't exist
    let format = detect_format(&args.file)?;

    let path = Path::new(&args.file);
    if !path.exists() {
        bail!("File not found: {}", args.file);
    }

    // Export mode: convert and exit
    if let Some(export_format) = args.export {
        return export::export(path, format, export_format, args.output.as_deref());
    }

    match format {
        PresentationFormat::Markdown => {
            run_markdown_presentation(&args.file)?;
        }
        PresentationFormat::Html => {
            html::serve_html(path, args.port)?;
        }
    }

    Ok(())
}

/// Runs an interactive markdown slide presentation in the terminal.
///
/// Reads the file, parses it into slides, initializes the TUI, and enters
/// the main rendering/event loop until the user quits.
fn run_markdown_presentation(file_path: &str) -> Result<()> {
    // 1. Read file content (with user-friendly error for binary/corrupted files)
    let content = fs::read_to_string(file_path).with_context(|| {
        format!("Failed to read '{file_path}' (is it a binary or corrupted file?)")
    })?;

    // 2. Parse into Presentation via markdown::parser
    let presentation = parse_presentation(&content);

    // 3. Create App with slide count
    let mut app = App::new(presentation.slide_count());

    // Create syntax highlighter (expensive — create once at startup)
    let highlighter = SyntaxHighlighter::new();

    // 4. Init terminal with panic hook
    tui::install_panic_hook();
    let mut terminal = tui::init()?;

    // Create transition manager for slide animations
    let mut transition_manager = tui::TransitionManager::new();

    // Track elapsed time between frames for animation timing
    let mut last_frame_time = Instant::now();

    // 5. Enter main loop
    loop {
        // Calculate elapsed time since last frame for animation processing
        let elapsed = last_frame_time.elapsed();
        last_frame_time = Instant::now();

        // Draw current slide via SlideWidget + render progress indicator
        terminal.draw(|frame| {
            let area = frame.area();

            // Edge case: terminal too small shows warning message
            if area.width < 40 || area.height < 10 {
                let warning = Paragraph::new("Terminal too small (minimum 40x10)")
                    .style(Style::default().fg(Color::Red))
                    .alignment(Alignment::Center);
                frame.render_widget(warning, area);
                return;
            }

            // Layout::vertical split for [main content area, footer bar]
            let chunks = Layout::vertical([Constraint::Min(1), Constraint::Length(1)]).split(area);

            let main_area = chunks[0];
            let footer_area = chunks[1];

            // Render current slide
            if let Some(current_slide) = presentation.get_slide(app.current_slide_index) {
                if current_slide.is_empty() {
                    // Edge case: empty file shows 'No slides found'
                    let msg = Paragraph::new("No slides found")
                        .style(Style::default().fg(Color::DarkGray))
                        .alignment(Alignment::Center);
                    frame.render_widget(msg, main_area);
                } else {
                    let slide_widget = SlideWidget::new(current_slide, &highlighter);
                    frame.render_widget(slide_widget, main_area);
                }
            }

            // Render progress indicator (slide N / M) in footer area
            // Hide for single-slide presentations since navigation is disabled
            if app.is_single_slide() {
                // Show a subtle help hint instead of navigation indicator
                let hint = Paragraph::new(Line::from(vec![Span::styled(
                    "  q to quit  ",
                    Style::default().fg(Color::DarkGray),
                )]))
                .alignment(Alignment::Right);
                frame.render_widget(hint, footer_area);
            } else {
                let progress = format!("  {}  ", app.progress_text());
                let footer = Paragraph::new(Line::from(vec![Span::styled(
                    progress,
                    Style::default().fg(Color::DarkGray),
                )]))
                .alignment(Alignment::Right);
                frame.render_widget(footer, footer_area);
            }

            // Apply transition effects AFTER widget rendering
            transition_manager.process(elapsed, frame.buffer_mut(), main_area);
        })?;

        // 6. Poll events and dispatch actions to App
        let prev_index = app.current_slide_index;
        let action = tui::poll_event()?;
        app.handle_action(action);

        // Trigger transition effect when slide index actually changes
        if app.current_slide_index != prev_index {
            transition_manager.trigger(action);
        }

        // 7. Break on Quit
        if app.should_quit {
            break;
        }
    }

    // 8. Restore terminal
    tui::restore()?;

    Ok(())
}
