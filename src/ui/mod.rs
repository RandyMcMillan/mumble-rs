use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use std::{
    io::{self, stdout, Stdout},
    sync::{Arc, Mutex},
};

#[derive(Clone, Debug)]
pub struct User {
    name: String,
    is_talking: bool,
    is_muted: bool,
}

#[derive(Clone, Debug)]
pub struct Channel {
    name: String,
    users: Vec<User>,
    subchannels: Vec<Channel>,
}

pub struct AppState {
    channels: Vec<Channel>,
    log_messages: Arc<Mutex<Vec<String>>>, 
    selected_item: usize,
    total_items: usize,
}

impl AppState {
    fn new(log_messages: Arc<Mutex<Vec<String>>>) -> Self {
        // --- Mock Data ---
        let mock_users = vec![
            User { name: "Alice".to_string(), is_talking: true, is_muted: false },
            User { name: "Bob".to_string(), is_talking: false, is_muted: true },
        ];
        let mock_subchannel = Channel {
            name: "Sub-Channel 1".to_string(),
            users: vec![User { name: "Charlie".to_string(), is_talking: false, is_muted: false }],
            subchannels: vec![],
        };
        let mock_channels = vec![
            Channel {
                name: "Root".to_string(),
                users: mock_users,
                subchannels: vec![mock_subchannel],
            },
            Channel {
                name: "Lobby".to_string(),
                users: vec![],
                subchannels: vec![],
            },
        ];
        // --- End Mock Data ---

        let total_items = count_items(&mock_channels);

        Self {
            channels: mock_channels,
            log_messages,
            selected_item: 0,
            total_items,
        }
    }

    fn next(&mut self) {
        self.selected_item = (self.selected_item + 1) % self.total_items;
    }

    fn previous(&mut self) {
        if self.selected_item > 0 {
            self.selected_item -= 1;
        } else {
            self.selected_item = self.total_items - 1;
        }
    }
}

fn count_items(channels: &[Channel]) -> usize {
    let mut count = 0;
    for channel in channels {
        count += 1; // For the channel itself
        count += channel.users.len();
        count += count_items(&channel.subchannels);
    }
    count
}

pub struct Tui {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    app_state: AppState,
}

impl Tui {
    pub fn new(log_messages: Arc<Mutex<Vec<String>>>) -> io::Result<Self> {
        let backend = CrosstermBackend::new(stdout());
        let mut terminal = Terminal::new(backend)?;
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)?;
        terminal.clear()?;
        Ok(Self {
            terminal,
            app_state: AppState::new(log_messages),
        })
    }

    pub fn run(&mut self) -> io::Result<()> {
        loop {
            self.terminal.draw(|frame| ui(frame, &self.app_state))?;

            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') => break,
                            KeyCode::Down => self.app_state.next(),
                            KeyCode::Up => self.app_state.previous(),
                            _ => {}
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

fn ui(frame: &mut Frame, app_state: &AppState) {
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(frame.area());

    let channel_pane = render_channel_pane(app_state);
    frame.render_widget(channel_pane, main_layout[0]);

    let log_pane = render_log_pane(app_state);
    frame.render_widget(log_pane, main_layout[1]);
}

fn render_channel_pane(app_state: &AppState) -> List<'_> {
    let mut items = vec![];
    let mut current_index = 0;
    for channel in &app_state.channels {
        build_list_items(channel, 0, &mut items, &mut current_index, app_state);
    }

    List::new(items).block(Block::default().title("Channels").borders(Borders::ALL))
}

fn build_list_items<'a>(
    channel: &'a Channel,
    depth: usize,
    items: &mut Vec<ListItem<'a>>,
    current_index: &mut usize,
    app_state: &AppState,
) {
    let prefix = " ".repeat(depth * 2);
    let style = if *current_index == app_state.selected_item {
        Style::default().add_modifier(Modifier::REVERSED)
    } else {
        Style::default()
    };
    items.push(ListItem::new(format!("{}v {}", prefix, channel.name)).style(style));
    *current_index += 1;

    for user in &channel.users {
        let user_prefix = " ".repeat((depth + 1) * 2);
        let status = if user.is_talking { "S" } else if user.is_muted { "M" } else { " " };
        let user_style = if *current_index == app_state.selected_item {
            Style::default().add_modifier(Modifier::REVERSED)
        } else {
            Style::default()
        };
        items.push(ListItem::new(format!("{}[{}] {}", user_prefix, status, user.name)).style(user_style));
        *current_index += 1;
    }

    for subchannel in &channel.subchannels {
        build_list_items(subchannel, depth + 1, items, current_index, app_state);
    }
}

fn render_log_pane(app_state: &AppState) -> Paragraph<'_> {
    let log_messages = app_state.log_messages.lock().unwrap();
    let log_text = log_messages.join("\n");
    Paragraph::new(log_text).block(Block::default().title("Log").borders(Borders::ALL))
}


impl Drop for Tui {
    fn drop(&mut self) {
        let _ = execute!(self.terminal.backend_mut(), LeaveAlternateScreen);
        let _ = disable_raw_mode();
    }
}
