use std::{
    io::{self, stdout, Stdout},
    sync::{Arc, Mutex},
};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

pub struct Tui {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    log_messages: Arc<Mutex<Vec<String>>>,
}

impl Tui {
    pub fn new(log_messages: Arc<Mutex<Vec<String>>>) -> io::Result<Self> {
        let backend = CrosstermBackend::new(stdout());
        let mut terminal = Terminal::new(backend)?;
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)?;
        terminal.clear()?;
        Ok(Self { terminal, log_messages })
    }

    pub fn run(&mut self) -> io::Result<()> {
        loop {
            self.terminal.draw(|frame| {
                let log_messages = self.log_messages.lock().unwrap();
                let log_text = log_messages.join("\n");
                let log_paragraph = Paragraph::new(log_text)
                    .block(Block::default().title("Log").borders(Borders::ALL));
                frame.render_widget(log_paragraph, frame.area());
            })?;

            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if let KeyCode::Char('q') = key.code {
                        break;
                    }
                }
            }
        }
        Ok(())
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        // It's good practice to ignore errors here, as we don't want to panic in a drop.
        let _ = execute!(self.terminal.backend_mut(), LeaveAlternateScreen);
        let _ = disable_raw_mode();
    }
}