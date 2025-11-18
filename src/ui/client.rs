use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph}
};
use std::io::{self, stdout, Stdout};
use crate::ui::servers;

pub struct AppState {
    log_messages: Vec<String>,
}

impl AppState {
    fn new() -> Self {
        let mock_logs = vec![
            "[INFO] Welcome to Mumble!".to_string(),
            "[INFO] Use arrow keys to navigate.".to_string(),
        ];

        Self {
            log_messages: mock_logs,
        }
    }
}

pub struct Tui {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    app_state: AppState,
}

impl Tui {
    pub fn new() -> io::Result<Self> {
        let backend = CrosstermBackend::new(stdout());
        let mut terminal = Terminal::new(backend)?;
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)?;
        terminal.clear()?;
        Ok(Self {
            terminal,
            app_state: AppState::new(),
        })
    }

    pub fn run(&mut self) -> io::Result<()> {
        loop {
            self.terminal.draw(|frame| ui(frame, &self.app_state))?;

            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        if let KeyCode::Char('q') = key.code {
                            break;
                        }
                    }
                }
            }
        }
        Ok(())
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
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(frame.area());

    let server_list = servers::render_server_list();
    frame.render_widget(server_list, main_layout[0]);

    let log_pane = render_log_pane(app_state);
    frame.render_widget(log_pane, main_layout[1]);
}

fn render_log_pane<'a>(app_state: &'a AppState) -> Paragraph<'a> {
    let log_text = app_state.log_messages.join("\n");
    Paragraph::new(log_text).block(Block::default().title("Log").borders(Borders::ALL))
}
