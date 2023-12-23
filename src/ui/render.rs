use crate::ui::App;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub fn render(f: &mut Frame, app: &mut App::App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(f.size());

    let mut message_block_constraints = Vec::new();
    let messages = app.session.get_messages();

    for _ in 0..messages.len() {
        message_block_constraints.push(Constraint::Percentage(100 / messages.len() as u16));
    }

    let message_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(message_block_constraints)
        .split(chunks[0]);

    for (i, message) in messages.iter().enumerate() {
        let inner_message_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(message_layout[i]);
        let block = Block::default().borders(Borders::ALL);
        let message_paragraph = Paragraph::new(message.body.clone());
        let inner_block = block.inner(inner_message_layout[1]);
        f.render_widget(block, inner_message_layout[1]);
        f.render_widget(message_paragraph, inner_block);
    }

    for k in app.event_buffer.drain(..) {
        app.text_area.input(k);
    }
    f.render_widget(app.text_area.widget(), chunks[1]);
}
