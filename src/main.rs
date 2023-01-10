mod app;
mod events;
mod ui;

use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use events::{watch_keys, Event};
use ui::draw_ui;

use std::{io, sync::mpsc};
use tui::{backend::CrosstermBackend, layout::Rect, Terminal};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;

    let (tx, rx) = mpsc::channel();
    watch_keys(tx);

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        terminal.draw(|f| draw_ui(f, &mut app))?;

        // Check channel
        match rx.recv()? {
            Event::Input(event) => match event.code {
                event::KeyCode::Esc => {
                    break;
                }
                event::KeyCode::Char('q') => {
                    break;
                }
                event::KeyCode::Char('j') => app.toggle(),
                _ => {}
            },

            Event::Resize(x, y) => terminal.resize(Rect::new(0, 0, x, y))?,
            Event::Tick => {}
        };
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    Ok(())
}
