use rspotify::model::FullTrack;

pub struct App {
    pub currently_playing: Option<FullTrack>,
}

impl App {
    pub fn new() -> Self {
        App {
            currently_playing: None,
        }
    }
}
