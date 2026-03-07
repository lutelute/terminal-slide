use std::time::Duration;

use anyhow::Result;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

/// Actions that can be triggered by user input or system events.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// Quit the application.
    Quit,
    /// Navigate to the next slide.
    NextSlide,
    /// Navigate to the previous slide.
    PrevSlide,
    /// Jump to the first slide.
    FirstSlide,
    /// Jump to the last slide.
    LastSlide,
    /// A tick event (used for animation timing).
    Tick,
    /// No action to take.
    None,
}

/// Poll for the next event and map it to an [`Action`].
///
/// Polls crossterm events with a 16ms timeout (~60fps) to enable smooth
/// animation ticks. Only processes `KeyEventKind::Press` events to avoid
/// double-navigation on platforms that emit Release/Repeat events.
///
/// Returns `Action::Tick` if no event is available within the timeout.
pub fn poll_event() -> Result<Action> {
    if event::poll(Duration::from_millis(16))? {
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                return Ok(Action::None);
            }
            return Ok(map_key_event(key));
        }
        // Non-key events (mouse, resize, etc.) are ignored.
        return Ok(Action::None);
    }
    Ok(Action::Tick)
}

/// Map a key event to an [`Action`].
///
/// Key mapping:
/// - Right / l / j / n / Space → NextSlide
/// - Left / h / k / p → PrevSlide
/// - q / Esc → Quit
/// - Ctrl+C → Quit
/// - g → FirstSlide
/// - G (Shift+g) → LastSlide
fn map_key_event(key: KeyEvent) -> Action {
    // Ctrl+C is a quit action (in raw mode, it does not generate SIGINT).
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        return Action::Quit;
    }

    match key.code {
        // Quit
        KeyCode::Char('q') => Action::Quit,
        KeyCode::Esc => Action::Quit,

        // Next slide
        KeyCode::Right => Action::NextSlide,
        KeyCode::Char('l') => Action::NextSlide,
        KeyCode::Char('j') => Action::NextSlide,
        KeyCode::Char('n') => Action::NextSlide,
        KeyCode::Char(' ') => Action::NextSlide,

        // Previous slide
        KeyCode::Left => Action::PrevSlide,
        KeyCode::Char('h') => Action::PrevSlide,
        KeyCode::Char('k') => Action::PrevSlide,
        KeyCode::Char('p') => Action::PrevSlide,

        // First slide
        KeyCode::Char('g') => Action::FirstSlide,

        // Last slide (Shift+g produces uppercase 'G')
        KeyCode::Char('G') => Action::LastSlide,

        _ => Action::None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to create a key press event with no modifiers.
    fn key_press(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    /// Helper to create a key press event with modifiers.
    fn key_press_with(code: KeyCode, modifiers: KeyModifiers) -> KeyEvent {
        KeyEvent::new(code, modifiers)
    }

    #[test]
    fn test_quit_actions() {
        assert_eq!(map_key_event(key_press(KeyCode::Char('q'))), Action::Quit);
        assert_eq!(map_key_event(key_press(KeyCode::Esc)), Action::Quit);
        assert_eq!(
            map_key_event(key_press_with(KeyCode::Char('c'), KeyModifiers::CONTROL)),
            Action::Quit
        );
    }

    #[test]
    fn test_next_slide_actions() {
        assert_eq!(
            map_key_event(key_press(KeyCode::Right)),
            Action::NextSlide
        );
        assert_eq!(
            map_key_event(key_press(KeyCode::Char('l'))),
            Action::NextSlide
        );
        assert_eq!(
            map_key_event(key_press(KeyCode::Char('j'))),
            Action::NextSlide
        );
        assert_eq!(
            map_key_event(key_press(KeyCode::Char('n'))),
            Action::NextSlide
        );
        assert_eq!(
            map_key_event(key_press(KeyCode::Char(' '))),
            Action::NextSlide
        );
    }

    #[test]
    fn test_prev_slide_actions() {
        assert_eq!(
            map_key_event(key_press(KeyCode::Left)),
            Action::PrevSlide
        );
        assert_eq!(
            map_key_event(key_press(KeyCode::Char('h'))),
            Action::PrevSlide
        );
        assert_eq!(
            map_key_event(key_press(KeyCode::Char('k'))),
            Action::PrevSlide
        );
        assert_eq!(
            map_key_event(key_press(KeyCode::Char('p'))),
            Action::PrevSlide
        );
    }

    #[test]
    fn test_first_last_slide_actions() {
        assert_eq!(
            map_key_event(key_press(KeyCode::Char('g'))),
            Action::FirstSlide
        );
        assert_eq!(
            map_key_event(key_press(KeyCode::Char('G'))),
            Action::LastSlide
        );
    }

    #[test]
    fn test_unmapped_keys_return_none() {
        assert_eq!(
            map_key_event(key_press(KeyCode::Char('x'))),
            Action::None
        );
        assert_eq!(
            map_key_event(key_press(KeyCode::Char('z'))),
            Action::None
        );
        assert_eq!(map_key_event(key_press(KeyCode::Tab)), Action::None);
        assert_eq!(map_key_event(key_press(KeyCode::Enter)), Action::None);
    }
}
