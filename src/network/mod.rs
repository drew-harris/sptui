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
            Ok(opt) => return opt,
            Err(err) => {
                println!("Error getting currently playing: {:?}", err);
                return None;
            }
        }
    }

    pub async fn set_currently_playing(&self) {
        let playing = self.get_currently_playing().await;
        let mut app = self.app.lock().await;

        match playing {
            Some(playing) => match playing.item {
                Some(item) => match item.to_owned() {
                    rspotify::model::PlayableItem::Track(track) => {
                        app.currently_playing = Some(track);
                    }
                    rspotify::model::PlayableItem::Episode(_) => app.currently_playing = None,
                },

                None => {
                    app.currently_playing = None;
                }
            },
            None => {
                {
                    app.currently_playing = None;
                };
            }
        }
    }
}
