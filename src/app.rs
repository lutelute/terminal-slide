use crate::tui::event::Action;

/// The main application state for slide navigation.
///
/// Tracks the current slide index, total number of slides, and whether
/// the application should quit. The navigation methods clamp the index
/// to valid bounds to prevent out-of-range access.
#[derive(Debug)]
pub struct App {
    /// The index of the currently displayed slide (0-based).
    pub current_slide_index: usize,
    /// The total number of slides in the presentation.
    pub total_slides: usize,
    /// Whether the application should quit on the next loop iteration.
    pub should_quit: bool,
}

impl App {
    /// Creates a new `App` with the given number of slides, starting at slide 0.
    pub fn new(total_slides: usize) -> Self {
        Self {
            current_slide_index: 0,
            total_slides,
            should_quit: false,
        }
    }

    /// Advance to the next slide, clamping at the last slide.
    pub fn next_slide(&mut self) {
        if self.total_slides > 0 && self.current_slide_index < self.total_slides - 1 {
            self.current_slide_index += 1;
        }
    }

    /// Go back to the previous slide, clamping at the first slide.
    pub fn prev_slide(&mut self) {
        if self.current_slide_index > 0 {
            self.current_slide_index -= 1;
        }
    }

    /// Jump to the first slide.
    pub fn first_slide(&mut self) {
        self.current_slide_index = 0;
    }

    /// Jump to the last slide.
    pub fn last_slide(&mut self) {
        if self.total_slides > 0 {
            self.current_slide_index = self.total_slides - 1;
        }
    }

    /// Handle an [`Action`] by dispatching to the appropriate navigation method.
    ///
    /// Returns the action that was handled, which callers can use to trigger
    /// transition effects or other side effects.
    pub fn handle_action(&mut self, action: Action) -> Action {
        match action {
            Action::Quit => {
                self.should_quit = true;
            }
            Action::NextSlide => {
                self.next_slide();
            }
            Action::PrevSlide => {
                self.prev_slide();
            }
            Action::FirstSlide => {
                self.first_slide();
            }
            Action::LastSlide => {
                self.last_slide();
            }
            Action::Tick | Action::None => {}
        }
        action
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_new() {
        let app = App::new(5);
        assert_eq!(app.current_slide_index, 0);
        assert_eq!(app.total_slides, 5);
        assert!(!app.should_quit);
    }

    #[test]
    fn test_next_slide() {
        let mut app = App::new(3);
        assert_eq!(app.current_slide_index, 0);

        app.next_slide();
        assert_eq!(app.current_slide_index, 1);

        app.next_slide();
        assert_eq!(app.current_slide_index, 2);
    }

    #[test]
    fn test_next_slide_clamps_at_last() {
        let mut app = App::new(3);
        app.current_slide_index = 2; // last slide

        app.next_slide();
        assert_eq!(app.current_slide_index, 2, "should not go past last slide");

        // Multiple attempts should not change the index.
        app.next_slide();
        app.next_slide();
        assert_eq!(app.current_slide_index, 2);
    }

    #[test]
    fn test_prev_slide() {
        let mut app = App::new(3);
        app.current_slide_index = 2;

        app.prev_slide();
        assert_eq!(app.current_slide_index, 1);

        app.prev_slide();
        assert_eq!(app.current_slide_index, 0);
    }

    #[test]
    fn test_prev_slide_clamps_at_first() {
        let mut app = App::new(3);
        assert_eq!(app.current_slide_index, 0);

        app.prev_slide();
        assert_eq!(app.current_slide_index, 0, "should not go below 0");

        // Multiple attempts should not underflow.
        app.prev_slide();
        app.prev_slide();
        assert_eq!(app.current_slide_index, 0);
    }

    #[test]
    fn test_first_slide() {
        let mut app = App::new(5);
        app.current_slide_index = 3;

        app.first_slide();
        assert_eq!(app.current_slide_index, 0);
    }

    #[test]
    fn test_last_slide() {
        let mut app = App::new(5);
        assert_eq!(app.current_slide_index, 0);

        app.last_slide();
        assert_eq!(app.current_slide_index, 4);
    }

    #[test]
    fn test_first_slide_when_already_first() {
        let mut app = App::new(5);
        assert_eq!(app.current_slide_index, 0);

        app.first_slide();
        assert_eq!(app.current_slide_index, 0);
    }

    #[test]
    fn test_last_slide_when_already_last() {
        let mut app = App::new(5);
        app.current_slide_index = 4;

        app.last_slide();
        assert_eq!(app.current_slide_index, 4);
    }

    #[test]
    fn test_handle_action_quit() {
        let mut app = App::new(3);
        assert!(!app.should_quit);

        app.handle_action(Action::Quit);
        assert!(app.should_quit);
    }

    #[test]
    fn test_handle_action_navigation() {
        let mut app = App::new(5);

        app.handle_action(Action::NextSlide);
        assert_eq!(app.current_slide_index, 1);

        app.handle_action(Action::NextSlide);
        assert_eq!(app.current_slide_index, 2);

        app.handle_action(Action::PrevSlide);
        assert_eq!(app.current_slide_index, 1);

        app.handle_action(Action::LastSlide);
        assert_eq!(app.current_slide_index, 4);

        app.handle_action(Action::FirstSlide);
        assert_eq!(app.current_slide_index, 0);
    }

    #[test]
    fn test_handle_action_tick_and_none() {
        let mut app = App::new(3);
        app.current_slide_index = 1;

        app.handle_action(Action::Tick);
        assert_eq!(app.current_slide_index, 1);
        assert!(!app.should_quit);

        app.handle_action(Action::None);
        assert_eq!(app.current_slide_index, 1);
        assert!(!app.should_quit);
    }

    #[test]
    fn test_single_slide_presentation() {
        let mut app = App::new(1);
        assert_eq!(app.current_slide_index, 0);

        // Next should not advance past the only slide.
        app.next_slide();
        assert_eq!(app.current_slide_index, 0);

        // Prev should stay at 0.
        app.prev_slide();
        assert_eq!(app.current_slide_index, 0);

        // First and last should both be 0.
        app.first_slide();
        assert_eq!(app.current_slide_index, 0);
        app.last_slide();
        assert_eq!(app.current_slide_index, 0);
    }

    #[test]
    fn test_zero_slides_presentation() {
        let mut app = App::new(0);
        assert_eq!(app.current_slide_index, 0);

        // All navigation should be safe with zero slides.
        app.next_slide();
        assert_eq!(app.current_slide_index, 0);

        app.prev_slide();
        assert_eq!(app.current_slide_index, 0);

        app.first_slide();
        assert_eq!(app.current_slide_index, 0);

        app.last_slide();
        assert_eq!(app.current_slide_index, 0);
    }

    #[test]
    fn test_handle_action_returns_action() {
        let mut app = App::new(3);

        assert_eq!(app.handle_action(Action::NextSlide), Action::NextSlide);
        assert_eq!(app.handle_action(Action::PrevSlide), Action::PrevSlide);
        assert_eq!(app.handle_action(Action::Quit), Action::Quit);
        assert_eq!(app.handle_action(Action::Tick), Action::Tick);
        assert_eq!(app.handle_action(Action::None), Action::None);
    }
}
