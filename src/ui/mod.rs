use tui::{
    backend::Backend,
    layout::Margin,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::App;

pub fn draw_ui<B: Backend>(f: &mut Frame<'_, B>, app: &App) {
    let size = f.size();

    let title = match &app.currently_playing {
        Some(track_name) => format!("Now Playing: {}", track_name),
        None => "Nothing Playing".to_string(),
    };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    match &app.currently_playing {
        Some(title) => {
            let name = Paragraph::new(title.to_owned());
            f.render_widget(
                name,
                size.inner(&Margin {
                    vertical: 2,
                    horizontal: 4,
                }),
            );
        }
        None => (),
    }

    f.render_widget(block, size);
}
