use std::io::{stdout, Result};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{
            disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
            LeaveAlternateScreen,
        },
        ExecutableCommand,
    },
    style::Stylize,
    widgets::Paragraph,
    Terminal,
};

fn main() -> Result<()> {
    //-- Startup

    /*
    Enter alternate screen mode,  which is a secondary screen which allows
    your application to render whatever it needs to, without disturbing the
    normal output of terminal apps in your shell.
    */
    stdout().execute(EnterAlternateScreen)?;

    // Enables raw mode, which turns off input and output processing by the terminal.
    // This gives your application control over when to print characters to the screen.
    enable_raw_mode()?;

    // creates a backend and Terminal
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    // Clears the screen
    terminal.clear()?;

    //-- Main lop
    loop {
        // Draw the UI
        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                    .white()
                    .on_blue(),
                area,
            );
        })?;

        // Handle events
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q')
                {
                    break;
                }
            }
        }
    }

    //-- Exiting
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
