use crate::lan::ServerInfo;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Row, Table},
};

pub fn render_server_list<'a>(servers: &'a [ServerInfo]) -> Table<'a> {
    let header = Row::new(vec!["Server Name", "Address", "Users"])
        .style(Style::default().add_modifier(Modifier::BOLD));

    let rows: Vec<Row> = servers
        .iter()
        .map(|s| {
            Row::new(vec![
                s.name.clone(),
                format!("{}:{}", s.host, s.port),
                format!("{}/{}", s.users, s.max_users),
            ])
        })
        .collect();

    Table::new(rows, [Constraint::Percentage(50), Constraint::Percentage(30), Constraint::Percentage(20)])
        .header(header)
        .block(Block::default().title("Public Servers").borders(Borders::ALL))
}
