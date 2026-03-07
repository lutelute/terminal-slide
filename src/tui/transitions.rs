use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use tachyonfx::{fx, EffectManager, Interpolation, Motion};

use crate::tui::event::Action;

/// Manages slide transition animations using tachyonfx effects.
///
/// Wraps tachyonfx's [`EffectManager`] to create and process transition effects
/// when navigating between slides. Effects are applied post-render, modifying
/// the terminal buffer to create smooth visual transitions.
///
/// Uses unique effects keyed by `"slide_transition"` so that rapid navigation
/// cancels any in-progress transition before starting a new one.
pub struct TransitionManager {
    effects: EffectManager<&'static str>,
}

impl TransitionManager {
    /// Creates a new `TransitionManager` with no active effects.
    pub fn new() -> Self {
        Self {
            effects: EffectManager::default(),
        }
    }

    /// Trigger a transition effect for the given navigation action.
    ///
    /// Creates the appropriate visual effect based on the action type:
    /// - **NextSlide**: slide-in from right with CubicOut interpolation (~300ms)
    /// - **PrevSlide**: slide-in from left with CubicOut interpolation (~300ms)
    /// - **FirstSlide / LastSlide**: coalesce (dissolve-style) effect (~200ms)
    ///
    /// Other actions (Quit, Tick, None) are ignored.
    pub fn trigger(&mut self, action: Action) {
        let effect = match action {
            // Forward navigation: new slide sweeps in from the right
            Action::NextSlide => Some(fx::slide_in(
                Motion::RightToLeft,
                10,
                0,
                Color::Reset,
                (300u32, Interpolation::CubicOut),
            )),
            // Backward navigation: new slide sweeps in from the left
            Action::PrevSlide => Some(fx::slide_in(
                Motion::LeftToRight,
                10,
                0,
                Color::Reset,
                (300u32, Interpolation::CubicOut),
            )),
            // Jump navigation: content materializes with a dissolve-style effect
            Action::FirstSlide | Action::LastSlide => Some(fx::coalesce(
                (200u32, Interpolation::CubicOut),
            )),
            _ => None,
        };

        if let Some(fx) = effect {
            self.effects.add_unique_effect("slide_transition", fx);
        }
    }

    /// Process active effects for the elapsed duration.
    ///
    /// Should be called each frame inside the render closure, after all
    /// widgets have been rendered to the buffer. Effects modify the buffer
    /// in-place to create visual transitions.
    pub fn process(&mut self, elapsed: std::time::Duration, buf: &mut Buffer, area: Rect) {
        self.effects.process_effects(elapsed.into(), buf, area);
    }
}
