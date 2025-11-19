use crate::{lan, public};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Row, Table},
};

pub fn render_lan_server_list<'a>(
    servers: &'a [lan::ServerInfo],
    has_focus: bool,
    selected_index: usize,
) -> Table<'a> {
    let header = Row::new(vec!["Server Name", "Host", "IP", "Port"])
        .style(Style::default().add_modifier(Modifier::BOLD));

    let rows: Vec<Row> = servers
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let style = if i == selected_index {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };
            Row::new(vec![
                s.name.clone(),
                s.host.clone(),
                s.ip.to_string(),
                s.port.to_string(),
            ])
            .style(style)
        })
        .collect();

    Table::new(rows, [Constraint::Percentage(40), Constraint::Percentage(30), Constraint::Percentage(20), Constraint::Percentage(10)])
        .header(header)
        .block(
            Block::default()
                .title("LAN Servers")
                .borders(Borders::ALL)
                .border_type(if has_focus {
                    ratatui::widgets::BorderType::Double
                } else {
                    ratatui::widgets::BorderType::Plain
                }),
        )
}

pub fn render_public_server_list<'a>(
    servers: &'a [public::ServerInfo],
    has_focus: bool,
    selected_index: usize,
) -> Table<'a> {
    let header = Row::new(vec!["Server Name", "Address", "Users"])
        .style(Style::default().add_modifier(Modifier::BOLD));

    let rows: Vec<Row> = servers
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let style = if i == selected_index {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };
            Row::new(vec![
                s.name.clone(),
                format!("{}:{}", s.host, s.port),
                format!("{}/{}", s.users, s.max_users),
            ])
            .style(style)
        })
        .collect();

    Table::new(rows, [Constraint::Percentage(50), Constraint::Percentage(30), Constraint::Percentage(20)])
        .header(header)
        .block(
            Block::default()
                .title("Public Servers")
                .borders(Borders::ALL)
                .border_type(if has_focus {
                    ratatui::widgets::BorderType::Double
                } else {
                    ratatui::widgets::BorderType::Plain
                }),
        )
}
