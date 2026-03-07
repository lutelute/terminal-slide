use std::io::{stdout, Stdout};
use std::panic;

use anyhow::Result;
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::{
    cursor,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::Terminal;

/// Type alias for the terminal type used throughout the application.
pub type Tui = Terminal<CrosstermBackend<Stdout>>;

/// Initialize the terminal for TUI rendering.
///
/// Enables raw mode, enters the alternate screen, and creates a
/// [`Terminal`] with a [`CrosstermBackend`].
pub fn init() -> Result<Tui> {
    terminal::enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

/// Restore the terminal to its original state.
///
/// Disables raw mode, leaves the alternate screen, and shows the cursor.
/// This must be called before the application exits to avoid leaving the
/// terminal in a broken state.
pub fn restore() -> Result<()> {
    terminal::disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    stdout().execute(cursor::Show)?;
    Ok(())
}

/// Install a panic hook that restores the terminal before printing the panic.
///
/// Without this hook, a panic would leave the terminal in raw mode with the
/// alternate screen still active, making the terminal unusable until `reset`
/// is run.
pub fn install_panic_hook() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        // Best-effort restore — ignore errors since we're already panicking.
        let _ = restore();
        original_hook(panic_info);
    }));
}
