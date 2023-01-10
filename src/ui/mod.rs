use tui::{
    backend::Backend,
    widgets::{Block, Borders},
    Frame,
};

use crate::app::App;

pub fn draw_ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();
    let title = match app.is_clicked {
        true => "Clicked".to_string(),
        false => "Not Clicked".to_string(),
    };

    let block = Block::default().title(title).borders(Borders::ALL);
    f.render_widget(block, size);
}
