use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use crossterm::event::Event as CEvent;

use std::{
    io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    widgets::{Block, Borders},
    Terminal,
};

enum Event<I> {
    Input(I),
    Resize(u16, u16),
    Tick,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);

    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                let event = event::read().expect("Can read events");
                match event {
                    CEvent::Key(key) => tx.send(Event::Input(key)).unwrap_or(()),

                    CEvent::Resize(x, y) => tx.send(Event::Resize(x, y)).unwrap_or(()),

                    _ => {}
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default().title("Test Block").borders(Borders::ALL);
            f.render_widget(block, size);
        })?;

        // Check channel
        match rx.recv()? {
            Event::Input(event) => match event.code {
                event::KeyCode::Esc => {
                    break;
                }

                event::KeyCode::Char('q') => {
                    break;
                }

                _ => {}
            },

            Event::Tick => {}

            Event::Resize(x, y) => terminal.resize(Rect::new(0, 0, x, y))?,
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
