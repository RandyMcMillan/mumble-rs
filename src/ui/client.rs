use crate::lan::ServerInfo;
use crate::ui::{local_server, servers};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use std::io::{self, stdout, Stdout};
use tokio::sync::mpsc;

#[derive(PartialEq, Eq)]
pub enum LocalServerState {
    Stopped,
    Starting,
    Running,
    Stopping,
}

pub enum ServerCommand {
    Start,
    Stop,
    Restart,
}

pub struct AppState {
    log_messages: Vec<String>,
    servers: Vec<ServerInfo>,
    pub local_server_state: LocalServerState,
}

impl AppState {
    fn new(servers: Vec<ServerInfo>) -> Self {
        let mock_logs = vec![
            "[INFO] Welcome to Mumble!".to_string(),
            "[INFO] Press 'q' to quit.".to_string(),
            "[INFO] Use 's' to start/stop and 'r' to restart the local server.".to_string(),
        ];

        Self {
            log_messages: mock_logs,
            servers,
            local_server_state: LocalServerState::Stopped,
        }
    }
}

pub struct Tui {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    pub app_state: AppState,
    command_tx: mpsc::Sender<ServerCommand>,
}

impl Tui {
    pub fn new(servers: Vec<ServerInfo>, command_tx: mpsc::Sender<ServerCommand>) -> io::Result<Self> {
        let backend = CrosstermBackend::new(stdout());
        let mut terminal = Terminal::new(backend)?;
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)?;
        terminal.clear()?;
        Ok(Self {
            terminal,
            app_state: AppState::new(servers),
            command_tx,
        })
    }

    pub fn draw(&mut self) -> io::Result<()> {
        self.terminal.draw(|frame| ui(frame, &self.app_state))?;
        Ok(())
    }

    pub fn handle_events(&mut self) -> io::Result<bool> {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Ok(true),
                        KeyCode::Char('s') => {
                            if self.app_state.local_server_state == LocalServerState::Running {
                                self.command_tx.try_send(ServerCommand::Stop).ok();
                            } else if self.app_state.local_server_state == LocalServerState::Stopped {
                                self.command_tx.try_send(ServerCommand::Start).ok();
                            }
                        }
                        KeyCode::Char('r') => {
                            if self.app_state.local_server_state == LocalServerState::Running {
                                self.command_tx.try_send(ServerCommand::Restart).ok();
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(false)
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        let _ = execute!(self.terminal.backend_mut(), LeaveAlternateScreen);
        let _ = disable_raw_mode();
    }
}

fn ui(frame: &mut Frame, app_state: &AppState) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // For the local server widget
            Constraint::Min(0),    // For the server list
            Constraint::Length(5), // For the log pane
        ])
        .split(frame.area());

    let local_server_widget = local_server::render(&app_state.local_server_state);
    frame.render_widget(local_server_widget, main_layout[0]);

    let server_list = servers::render_server_list(&app_state.servers);
    frame.render_widget(server_list, main_layout[1]);

    let log_pane = render_log_pane(app_state);
    frame.render_widget(log_pane, main_layout[2]);
}

fn render_log_pane<'a>(app_state: &'a AppState) -> Paragraph<'a> {
    let log_text = app_state.log_messages.join("\n");
    Paragraph::new(log_text).block(Block::default().title("Log").borders(Borders::ALL))
}