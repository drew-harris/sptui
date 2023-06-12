use std::sync::Arc;

use rspotify::{
    model::{AdditionalType, CurrentlyPlayingContext, Market},
    prelude::OAuthClient,
    AuthCodeSpotify,
};
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
            spotify,
            client_config: config,
            app,
        }
    }

    pub async fn get_currently_playing(&self) -> Option<CurrentlyPlayingContext> {
        let additional_type = [AdditionalType::Track];
        let currently_playing = self
            .spotify
            .current_playing(Some(Market::FromToken), Some(&additional_type))
            .await;

        match currently_playing {
            Ok(opt) => opt,
            Err(err) => {
                println!("Error getting currently playing: {:?}", err);
                None
            }
        }
    }

    pub async fn set_currently_playing(&self) {
        let playing = self.get_currently_playing().await;

        match playing {
            Some(playing) => match playing.item {
                Some(item) => match item.to_owned() {
                    rspotify::model::PlayableItem::Track(track) => {
                        let mut app = self.app.lock().await;
                        println!("Playing track: {}", track.name);
                        app.set_currently_playing(track.name);
                    }

                    _ => {}
                },

                None => {}
            },
            None => {
                let mut app = self.app.lock().await;
                app.set_currently_playing("".to_string());
            }
        }
    }
}