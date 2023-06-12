pub struct App {
    pub is_clicked: bool,
    pub currently_playing: Option<String>,
}

impl App {
    pub fn new() -> Self {
        App {
            is_clicked: false,
            currently_playing: None,
        }
    }

    pub fn set_currently_playing(&mut self, currently_playing: String) {
        self.currently_playing = Some(currently_playing);
    }
}
