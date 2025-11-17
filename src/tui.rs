use std::{
    io::{self, stdout},
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
    log_messages: Arc<Mutex<Vec<String>>>,
}

impl Tui {
    pub fn new(log_messages: Arc<Mutex<Vec<String>>>) -> Self {
        Self { log_messages }
    }

    pub fn run(&mut self) -> io::Result<()> {
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)?;

        loop {
            terminal.draw(|frame| {
                let log_messages = self.log_messages.lock().unwrap();
                let log_text = log_messages.join("\n");
                let log_paragraph = Paragraph::new(log_text)
                    .block(Block::default().title("Log").borders(Borders::ALL));
                frame.render_widget(log_paragraph, frame.size());
            })?;

            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if let KeyCode::Char('q') = key.code {
                        break;
                    }
                }
            }
        }

        execute!(stdout(), LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }
}
