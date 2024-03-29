mod app;
mod config;
mod events;
mod network;
mod ui;

use app::App;
use config::Config;
use events::{watch_keys, Event};
use network::Network;
use tokio::sync::Mutex;
use ui::draw_ui;

use rspotify::Config as RConfig;
use rspotify::{
    prelude::{BaseClient, OAuthClient},
    scopes, AuthCodeSpotify, Credentials, OAuth,
};

use std::sync::Arc;
use std::{io, sync::mpsc};

use anyhow::{Context, Result};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{backend::CrosstermBackend, layout::Rect, Terminal};

#[tokio::main]
async fn main() -> Result<()> {
    let mut config = Config::new();
    match config.load_config_from_file() {
        Ok(()) => {}
        Err(err) => {
            println!("Error loading config{:?}", err);
            config.prompt_for_config()?;
            config.save_config()?;
        }
    }

    let creds = Credentials::new(&config.client_id, &config.client_secret);
    let oauth = OAuth {
        redirect_uri: "http://localhost:8080".to_string(),
        scopes: scopes!("user-read-playback-state", "user-modify-playback-state"),
        ..Default::default()
    };

    let spotify = AuthCodeSpotify::with_config(
        creds,
        oauth,
        RConfig {
            token_cached: true,
            ..Default::default()
        },
    );

    let url = spotify.get_authorize_url(false)?;
    spotify.prompt_for_token(&url).await?;
    spotify.write_token_cache().await?;

    let app = Arc::new(Mutex::new(App::new()));
    let app_clone = Arc::clone(&app);
    let network = Network::new(spotify, &app_clone, config);

    network.set_currently_playing().await;

    // Start watching keys
    let (tx, rx) = mpsc::channel();
    watch_keys(tx);

    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
        .context("Failed to execute crossterm features")?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).context("Failed to create terminal")?;

    // Start network
    loop {
        // Draw the UI
        let mut app_ref = app.lock().await;
        terminal.draw(|f| draw_ui(f, &mut app_ref))?;

        // Check for keys and resizes
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
            Event::Resize(x, y) => terminal.resize(Rect::new(0, 0, x, y))?,
            Event::Tick => {}
        };
    }

    // Shutdown
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    Ok(())
}
