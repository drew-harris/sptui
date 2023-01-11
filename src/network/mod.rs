use std::sync::Arc;

use rspotify::AuthCodeSpotify;
use tokio::sync::Mutex;

use crate::{app::App, config::Config};

pub struct Network<'a> {
    pub spotify: AuthCodeSpotify,
    pub client_config: Config,
    pub app: &'a Arc<Mutex<App>>,
}

impl<'a> Network<'a> {
    pub fn new(spotify: AuthCodeSpotify, app: &'a Arc<Mutex<App>>, config: Config) -> Self {
        Network {
            spotify: spotify,
            client_config: config,
            app: app,
        }
    }
}
