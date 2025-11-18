use crate::ui::client::LocalServerState;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

pub fn render(state: &LocalServerState) -> Paragraph<'static> {
    let (status_text, button_text) = match state {
        LocalServerState::Running => (
            Line::from(vec!["Status: ".into(), "Running".green().bold()]),
            " [S]top [R]estart ",
        ),
        LocalServerState::Stopped => (
            Line::from(vec!["Status: ".into(), "Stopped".red().bold()]),
            " [S]tart ",
        ),
        LocalServerState::Starting => (
            Line::from(vec!["Status: ".into(), "Starting...".yellow().bold()]),
            " ".into(),
        ),
        LocalServerState::Stopping => (
            Line::from(vec!["Status: ".into(), "Stopping...".yellow().bold()]),
            " ".into(),
        ),
        LocalServerState::Restarting => (
            Line::from(vec!["Status: ".into(), "Restarting...".yellow().bold()]),
            " ".into(),
        ),
    };

    let text = Text::from(vec![
        status_text,
        Line::from(""), // Spacer
        Line::from(button_text).alignment(Alignment::Center),
    ]);

    Paragraph::new(text)
        .block(Block::default().title("Local Server").borders(Borders::ALL))
        .alignment(Alignment::Center)
}
