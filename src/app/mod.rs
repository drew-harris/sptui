use rspotify::model::FullTrack;

pub struct App {
    pub is_clicked: bool,
    pub currently_playing: Option<FullTrack>,
}

impl App {
    pub fn new() -> Self {
        App {
            is_clicked: false,
            currently_playing: None,
        }
    }
}
