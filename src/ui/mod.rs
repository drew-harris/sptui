use tui::{
    backend::Backend,
    widgets::{Block, Borders},
    Frame,
};

pub fn draw_ui<B: Backend>(f: &mut Frame<B>) {
    let size = f.size();
    let block = Block::default().title("Test Block").borders(Borders::ALL);
    f.render_widget(block, size);
}
