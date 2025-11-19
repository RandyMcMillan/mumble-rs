use crate::{lan, public};
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

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum FocusedWidget {
    LocalServer,
    LanServerList,
    PublicServerList,
    Content,
}

impl FocusedWidget {
    pub fn next(&self) -> Self {
        match self {
            Self::LocalServer => Self::LanServerList,
            Self::LanServerList => Self::PublicServerList,
            Self::PublicServerList => Self::Content,
            Self::Content => Self::LocalServer,
        }
    }
}

pub struct AppState {
    log_messages: Vec<String>,
    lan_servers: Vec<lan::ServerInfo>,
    public_servers: Vec<public::ServerInfo>,
    pub local_server_state: LocalServerState,
    pub current_view: CurrentView,
    pub local_server_logs: Arc<Mutex<Vec<String>>>,
    pub focused_widget: FocusedWidget,
    pub selected_lan_server: usize,
    pub selected_public_server: usize,
    pub content_scroll: usize,
}

impl AppState {
    fn new(
        lan_servers: Vec<lan::ServerInfo>,
        public_servers: Vec<public::ServerInfo>,
        local_server_logs: Arc<Mutex<Vec<String>>>,
    ) -> Self {
        let log_messages = vec![
            "[INFO] Welcome to Mumble!".to_string(),
            "[INFO] Press 'q' to quit, 'Tab' to navigate.".to_string(),
            "[INFO] Use 's' to start/stop and 'r' to restart the local server.".to_string(),
            r"[INFO] Press '\\' to toggle server log view.".to_string(),
        ];

        Self {
            log_messages,
            lan_servers,
            public_servers,
            local_server_state: LocalServerState::Stopped,
            current_view: CurrentView::Chat,
            local_server_logs,
            focused_widget: FocusedWidget::LanServerList,
            selected_lan_server: 0,
            selected_public_server: 0,
            content_scroll: 0,
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
        lan_servers: Vec<lan::ServerInfo>,
        public_servers: Vec<public::ServerInfo>,
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
            app_state: AppState::new(lan_servers, public_servers, local_server_logs),
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
                        KeyCode::Tab => {
                            self.app_state.focused_widget = self.app_state.focused_widget.next();
                        }
                        KeyCode::Char('\\') => {
                            self.app_state.current_view = match self.app_state.current_view {
                                CurrentView::Chat => CurrentView::LocalServerLog,
                                CurrentView::LocalServerLog => CurrentView::Chat,
                            };
                        }
                        KeyCode::Up => match self.app_state.focused_widget {
                            FocusedWidget::LanServerList => {
                                if self.app_state.selected_lan_server > 0 {
                                    self.app_state.selected_lan_server -= 1;
                                }
                            }
                            FocusedWidget::PublicServerList => {
                                if self.app_state.selected_public_server > 0 {
                                    self.app_state.selected_public_server -= 1;
                                }
                            }
                            FocusedWidget::Content => {
                                if self.app_state.content_scroll > 0 {
                                    self.app_state.content_scroll -= 1;
                                }
                            }
                            _ => {}
                        },
                        KeyCode::Down => match self.app_state.focused_widget {
                            FocusedWidget::LanServerList => {
                                if self.app_state.selected_lan_server < self.app_state.lan_servers.len() - 1 {
                                    self.app_state.selected_lan_server += 1;
                                }
                            }
                            FocusedWidget::PublicServerList => {
                                if self.app_state.selected_public_server < self.app_state.public_servers.len() - 1 {
                                    self.app_state.selected_public_server += 1;
                                }
                            }
                            FocusedWidget::Content => {
                                self.app_state.content_scroll += 1;
                            }
                            _ => {}
                        },
                        KeyCode::Char('s') if self.app_state.focused_widget == FocusedWidget::LocalServer => {
                            if self.app_state.local_server_state == LocalServerState::Running {
                                self.command_tx.try_send(ServerCommand::Stop).ok();
                            } else if self.app_state.local_server_state == LocalServerState::Stopped {
                                self.command_tx.try_send(ServerCommand::Start).ok();
                            }
                        }
                        KeyCode::Char('r') if self.app_state.focused_widget == FocusedWidget::LocalServer => {
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
    // Create a top-level horizontal layout (Left 30%, Right 70%)
    let top_level_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(frame.area());

    let left_pane = top_level_layout[0];
    let right_pane = top_level_layout[1];

    // Split the left pane vertically for the local server and server list
    let left_pane_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // Fixed height for local server widget
            Constraint::Percentage(50),
            Constraint::Min(0),    // Remaining space for server list
        ])
        .split(left_pane);

    let local_server_widget = local_server::render(
        &app_state.local_server_state,
        app_state.focused_widget == FocusedWidget::LocalServer,
    );
    frame.render_widget(local_server_widget, left_pane_layout[0]);

    let lan_server_list = servers::render_lan_server_list(
        &app_state.lan_servers,
        app_state.focused_widget == FocusedWidget::LanServerList,
        app_state.selected_lan_server,
    );
    frame.render_widget(lan_server_list, left_pane_layout[1]);

    let public_server_list = servers::render_public_server_list(
        &app_state.public_servers,
        app_state.focused_widget == FocusedWidget::PublicServerList,
        app_state.selected_public_server,
    );
    frame.render_widget(public_server_list, left_pane_layout[2]);

    // Split the right pane for content and client log
    let right_pane_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // Main content view
            Constraint::Length(5), // Client log at the bottom
        ])
        .split(right_pane);

    match app_state.current_view {
        CurrentView::Chat => {
            let chat_widget = Paragraph::new("Chat view placeholder...")
                .block(
                    Block::default()
                        .title("Chat")
                        .borders(Borders::ALL)
                        .border_style(if app_state.focused_widget == FocusedWidget::Content {
                            Style::default().fg(Color::Yellow)
                        } else {
                            Style::default()
                        }),
                )
                .scroll((app_state.content_scroll as u16, 0));
            frame.render_widget(chat_widget, right_pane_layout[0]);
        }
        CurrentView::LocalServerLog => {
            let log_view = log_view::render(
                &app_state.local_server_logs,
                app_state.focused_widget == FocusedWidget::Content,
                app_state.content_scroll,
            );
            frame.render_widget(log_view, right_pane_layout[0]);
        }
    }

    let log_pane = render_log_pane(app_state);
    frame.render_widget(log_pane, right_pane_layout[1]);
}

fn render_log_pane<'a>(app_state: &'a AppState) -> Paragraph<'a> {
    let log_text = app_state.log_messages.join("\n");
    Paragraph::new(log_text).block(Block::default().title("Client Log").borders(Borders::ALL))
}