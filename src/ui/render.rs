use crate::connection::message::Message;
use crate::ui::App;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;
use std::rc::Rc;

pub fn render(f: &mut Frame, app: &mut App) {
    let chunks = create_main_layout(f.size());
    render_messages(f, app, &chunks[0]);
    render_text_area(f, app, &chunks[1]);
}

fn create_main_layout(size: Rect) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(size)
}

fn render_messages(f: &mut Frame, app: &App, area: &Rect) {
    let messages = app.session.get_messages();
    let max_size = (f.size().height as f32 * 0.8 / 3.0).floor() as usize;

    let skip_amount = if messages.len() > max_size {
        messages.len() - max_size
    } else {
        0
    };
    let last_messages: Vec<_> = messages.iter().skip(skip_amount).cloned().collect();

    let message_layout = create_message_layout(last_messages.len(), area);

    for (i, message) in last_messages.iter().enumerate() {
        let inner_message_layout = create_inner_message_layout(message, &message_layout[i]);
        let block_color = if message.is_sent {
            Color::Magenta
        } else {
            Color::LightCyan
        };
        let block = Block::default()
            .border_style(Style::default().fg(block_color))
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL);
        let message_paragraph = Paragraph::new(format!(" {} ", message.body.clone()));
        let inner_block = block.inner(inner_message_layout[get_message_area_index(message)]);

        f.render_widget(block, inner_message_layout[get_message_area_index(message)]);
        f.render_widget(message_paragraph, inner_block);
    }
}

fn create_message_layout(message_count: usize, area: &Rect) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                vec![Constraint::Length(3); message_count],
                vec![Constraint::Min(0); 1],
            ]
            .concat(),
        )
        .split(*area)
}

fn create_inner_message_layout(message: &Message, area: &Rect) -> Rc<[Rect]> {
    let constraints = if message.is_sent {
        [
            Constraint::Min(0),
            Constraint::Max(message.body.len() as u16 + 4),
        ]
    } else {
        [
            Constraint::Max(message.body.len() as u16 + 4),
            Constraint::Min(0),
        ]
    };

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(*area)
}

fn get_message_area_index(message: &Message) -> usize {
    if message.is_sent {
        1
    } else {
        0
    }
}

fn render_text_area(f: &mut Frame, app: &mut App, area: &Rect) {
    for k in app.event_buffer.drain(..) {
        app.text_area.input(k);
    }
    let block = Block::default().borders(Borders::ALL);
    let inner_block = block.inner(*area);
    f.render_widget(block, *area);
    f.render_widget(app.text_area.widget(), inner_block);
}
