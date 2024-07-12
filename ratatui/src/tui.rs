use std::io::{self, stdout, Stdout};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        execute,
        terminal::{
            disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
            LeaveAlternateScreen,
        },
    },
    Terminal,
};

/// A type alias for the terminal type used in this application
pub type Tui = Terminal<CrosstermBackend<Stdout>>;

/// Initialize the terminal
pub fn init() -> io::Result<Tui> {
    // Enter alternate screen mode,  which is a secondary screen which allows
    // your application to render whatever it needs to, without disturbing the
    // normal output of terminal apps in your shell.
    execute!(stdout(), EnterAlternateScreen)?;

    // Enables raw mode, which turns off input and output processing by the terminal.
    // This gives your application control over when to print characters to the screen.
    enable_raw_mode()?;

    // creates a backend and Terminal
    Terminal::new(CrosstermBackend::new(stdout()))
}

/// Restore the terminal to its original state
pub fn restore() -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
