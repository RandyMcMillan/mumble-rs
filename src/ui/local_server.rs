use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

pub fn render(is_running: bool) -> Paragraph<'static> {
    let (status_text, button_text) = if is_running {
        (
            Line::from(vec!["Status: ".into(), "Running".green().bold()]),
            " [ Stop ] [ Restart ] ",
        )
    } else {
        (
            Line::from(vec!["Status: ".into(), "Stopped".red().bold()]),
            " [ Start ] ",
        )
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
