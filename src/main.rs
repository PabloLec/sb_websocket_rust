use crate::connection::session::Session;
use crate::ui::{render, App};
use crossterm::event::{DisableMouseCapture, Event, KeyCode};
use crossterm::{
    event::{self},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::backend::Backend;
use ratatui::prelude::{CrosstermBackend, Terminal};

use std::io::{stdout, Result};
use std::{env, io};

mod client;
mod connection;
mod server;
mod ui;
mod websocket;

fn main() -> Result<()> {
    let session = match start_session() {
        Ok(session) => session,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
    let mut terminal = prepare_terminal()?;
    terminal.clear()?;

    let mut app = App::App::new(session);
    match run_app(&mut terminal, &mut app) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    teardown_terminal(terminal)?;
    Ok(())
}

fn prepare_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    stdout().execute(EnterAlternateScreen).unwrap();
    enable_raw_mode().unwrap();
    Terminal::new(CrosstermBackend::new(stdout()))
}

fn teardown_terminal(mut terminal: Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    terminal.clear()?;
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App::App) -> Result<()> {
    loop {
        terminal.draw(|f| render(f, app))?;
        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    continue;
                }
                match key.code {
                    KeyCode::Esc => {
                        return Ok(());
                    }
                    KeyCode::Enter => {
                        app.event_buffer.clear();
                        app.send_message();
                    }
                    _ => {
                        app.event_buffer.push(key);
                    }
                }
            }
        }
    }
}

fn start_session() -> std::result::Result<Session, io::Error> {
    let args: Vec<String> = env::args().collect();
    let port = if args.len() > 1 { &args[1] } else { "36363" };

    match client::run_client(port) {
        Ok(session) => Ok(session),
        Err(_) => match server::run_server(port) {
            Ok(session) => Ok(session),
            Err(e) => Err(e),
        },
    }
}
