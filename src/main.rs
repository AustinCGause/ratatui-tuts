use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
use std::io::{stdout, Result};

// Main function that returns a Result from standard library
fn main() -> Result<()> {

    // Application enters the alternate screen, which gives you a clean slate without affecting
    // normal terminal output
    stdout().execute(EnterAlternateScreen)?;

    // Turns off input and output processing by the terminal. Gives app control over when to print
    // characters to the screen
    enable_raw_mode()?;

    // Creates a backend and a Terminal object
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    // Clears the terminal
    terminal.clear()?;

    // Initializes the main loop for the application to run in
    loop {

        // The draw method on the terminal object is the main interaction point for the
        // application. The draw method always accepts the |frame| closure and renders the entire
        // screen.
        terminal.draw(|frame| {

            // Creates an area that is the full size of the terminal window
            let area = frame.size();

            // Renders a paragraph widget with white text and blue background on the entire area
            // provided in the render widget argument
            frame.render_widget(
                Paragraph::new("Hello Ratatui! (press 'q' or 'Q' to quit)")
                    .white()
                    .on_blue(),
                area,
            );
        })?;

        // Checks if any events have occurred every 16 milliseconds, which is ~60fps
        if event::poll(std::time::Duration::from_millis(16))? {

            // Checks event type for a key press, press being important, as otherwise it would see
            // the key twice, adn then executes the break command if the key pressed is a q. This
            // exits the for loop and calls the exit code below.
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && ( key.code == KeyCode::Char('q') || key.code == KeyCode::Char('Q') )
                {
                    break;
                }
            }
        }
    }

    // Restores the terminal back to its previous state by leaving the alternate screen and
    // returning keystroks back to the terminal
    // MAKE SURE disable_raw_mode IS ALWAYS CALLED
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
