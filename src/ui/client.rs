use crate::lan::ServerInfo;
use crate::ui::{local_server, log_view, servers};
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
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum LocalServerState {
    Stopped,
    Starting,
    Running,
    Stopping,
    Restarting,
}

pub enum ServerCommand {
    Start,
    Stop,
    Restart,
}

pub enum CurrentView {
    Chat,
    LocalServerLog,
}

pub struct AppState {
    log_messages: Vec<String>,
    servers: Vec<ServerInfo>,
    pub local_server_state: LocalServerState,
    pub current_view: CurrentView,
    pub local_server_logs: Arc<Mutex<Vec<String>>>
}

impl AppState {
    fn new(servers: Vec<ServerInfo>, local_server_logs: Arc<Mutex<Vec<String>>>) -> Self {
        let log_messages = vec![
            "[INFO] Welcome to Mumble!".to_string(),
            "[INFO] Press 'q' to quit.".to_string(),
            "[INFO] Use 's' to start/stop and 'r' to restart the local server.".to_string(),
            r"[INFO] Press '\\' to toggle server log view.".to_string(),
        ];

        Self {
            log_messages,
            servers,
            local_server_state: LocalServerState::Stopped,
            current_view: CurrentView::Chat,
            local_server_logs,
        }
    }

    pub fn log(&mut self, message: String) {
        self.log_messages.push(message);
    }
}

pub struct Tui {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    pub app_state: AppState,
    command_tx: mpsc::Sender<ServerCommand>,
}

impl Tui {
    pub fn new(
        servers: Vec<ServerInfo>,
        local_server_logs: Arc<Mutex<Vec<String>>>,
        command_tx: mpsc::Sender<ServerCommand>,
    ) -> io::Result<Self> {
        let backend = CrosstermBackend::new(stdout());
        let mut terminal = Terminal::new(backend)?;
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)?;
        terminal.clear()?;
        Ok(Self {
            terminal,
            app_state: AppState::new(servers, local_server_logs),
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
                        KeyCode::Char('\\') => {
                            self.app_state.current_view = match self.app_state.current_view {
                                CurrentView::Chat => CurrentView::LocalServerLog,
                                CurrentView::LocalServerLog => CurrentView::Chat,
                            };
                        }
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
            Constraint::Min(0),    // For the main content area
            Constraint::Length(5), // For the client log pane
        ])
        .split(frame.area());

    let local_server_widget = local_server::render(&app_state.local_server_state);
    frame.render_widget(local_server_widget, main_layout[0]);

    let main_content_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(main_layout[1]);

    let server_list = servers::render_server_list(&app_state.servers);
    frame.render_widget(server_list, main_content_layout[0]);

    match app_state.current_view {
        CurrentView::Chat => {
            let chat_widget = Paragraph::new("Chat view placeholder...")
                .block(Block::default().title("Chat").borders(Borders::ALL));
            frame.render_widget(chat_widget, main_content_layout[1]);
        }
        CurrentView::LocalServerLog => {
            let log_view = log_view::render(&app_state.local_server_logs);
            frame.render_widget(log_view, main_content_layout[1]);
        }
    }

    let log_pane = render_log_pane(app_state);
    frame.render_widget(log_pane, main_layout[2]);
}

fn render_log_pane<'a>(app_state: &'a AppState) -> Paragraph<'a> {
    let log_text = app_state.log_messages.join("\n");
    Paragraph::new(log_text).block(Block::default().title("Client Log").borders(Borders::ALL))
}
