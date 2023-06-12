use std::{
    sync::mpsc::Sender,
    thread,
    time::{Duration, Instant},
};

use crossterm::event::Event as CEvent;
use crossterm::event::{self, KeyEvent};

pub enum Event<I> {
    Input(I),
    Resize(u16, u16),
    Tick,
}

pub fn watch_keys(tx: Sender<Event<KeyEvent>>) {
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

            if last_tick.elapsed() >= tick_rate && tx.send(Event::Tick).is_ok() {
                last_tick = Instant::now();
            }
        }
    });
}
