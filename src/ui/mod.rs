use tui::{
    backend::Backend,
    layout::Margin,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;

pub fn draw_ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();

    let title = match app.is_clicked {
        true => "Clicked".to_string(),
        false => "Not Clicked".to_string(),
    };

    let block = Block::default().title(title).borders(Borders::ALL);
    let hint = Paragraph::new("Press \"J\" to toggle");
    f.render_widget(
        hint,
        size.inner(&Margin {
            vertical: 2,
            horizontal: 4,
        }),
    );
    f.render_widget(block, size);
}
