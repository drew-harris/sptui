mod app;
mod events;
mod ui;

use app::App;
use events::{watch_keys, Event};
use ui::draw_ui;

use std::{
    io,
    sync::mpsc::{self, Receiver, Sender},
};

use anyhow::{Context, Result};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{backend::CrosstermBackend, layout::Rect, Terminal};

fn main() -> Result<()> {
    enable_raw_mode()?;

    let (tx, rx) = mpsc::channel();
    let (kill_keys_tx, kill_keys_rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();
    let key_thread = watch_keys(tx, kill_keys_rx);

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
        .context("Failed to execute crossterm features")?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).context("Failed to create terminal")?;

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

    kill_keys_tx.send(true)?;
    key_thread.join().unwrap_or(());

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    Ok(())
}
