use crate::config::Config;

pub struct App {
    pub is_clicked: bool,
}

impl App {
    pub fn new() -> Self {
        App { is_clicked: false }
    }

    pub fn toggle(&mut self) {
        self.is_clicked = !self.is_clicked;
    }
}
