use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use std::sync::{Arc, Mutex};

pub fn render<'a>(log_buffer: &'a Arc<Mutex<Vec<String>>>) -> Paragraph<'a> {
    let logs = log_buffer.lock().unwrap();
    let log_text: Vec<Line> = logs.iter().map(|s| Line::from(s.clone())).collect();

    Paragraph::new(log_text)
        .block(Block::default().title("Local Server Log").borders(Borders::ALL))
}
