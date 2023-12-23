use crate::connection::session::Session;
use crossterm::event::KeyEvent;
use tui_textarea::TextArea;

pub struct App {
    pub session: Session,
    pub event_buffer: Vec<KeyEvent>,
    pub text_area: TextArea<'static>,
}

impl App {
    pub fn new(session: Session) -> App {
        App {
            session,
            event_buffer: vec![],
            text_area: TextArea::default(),
        }
    }

    pub fn send_message(&mut self) {
        let message = self.text_area.lines();
        self.session.send_message(message.join("\n"));
        self.text_area.select_all();
        self.text_area.cut();
    }
}
