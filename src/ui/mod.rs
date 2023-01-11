use rspotify::model::CurrentlyPlayingContext;
use tui::{
    backend::Backend,
    layout::Margin,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::App;

pub fn draw_ui<B: Backend>(
    f: &mut Frame<B>,
    app: &App,
    currently_playing: &Option<CurrentlyPlayingContext>,
) {
    let size = f.size();

    let title = match app.is_clicked {
        true => "Clicked".to_string(),
        false => "Not Clicked".to_string(),
    };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    match *&currently_playing {
        Some(playing) => match &playing.item {
            Some(item) => match item.to_owned() {
                rspotify::model::PlayableItem::Track(track) => {
                    let name = Paragraph::new(track.name);
                    f.render_widget(
                        name,
                        size.inner(&Margin {
                            vertical: 2,
                            horizontal: 4,
                        }),
                    );
                }

                _ => {}
            },

            None => {}
        },

        _ => {}
    }

    f.render_widget(block, size);
}
