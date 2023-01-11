use crate::config::Config;

pub struct App {
    pub is_clicked: bool,
    pub config: Config,
}

impl App {
    pub fn new(config: Config) -> Self {
        App {
            is_clicked: false,
            config,
        }
    }

    pub fn toggle(&mut self) {
        self.is_clicked = !self.is_clicked;
    }
}
