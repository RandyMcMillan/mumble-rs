use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Row, Table},
};

pub fn render_server_list<'a>() -> Table<'a> {
    let header = Row::new(vec!["Server", "IP", "Server Name"])
        .style(Style::default().add_modifier(Modifier::BOLD));

    // Mock Data
    let rows = vec![
        Row::new(vec!["mumble.example.com", "192.0.2.1", "Example Server 1"]),
        Row::new(vec!["voice.example.org", "198.51.100.5", "Community Voice"]),
        Row::new(vec!["gaming.server", "203.0.113.10", "Gamers United"]),
    ];

    Table::new(rows, [Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(34)])
        .header(header)
        .block(Block::default().title("Servers").borders(Borders::ALL))
        .widths([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
}
