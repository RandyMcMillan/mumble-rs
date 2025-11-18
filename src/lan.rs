use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub users: u32,
    pub max_users: u32,
}

// This is a placeholder function. In the future, this will call into mumble-sys
// to discover servers on the network. For now, it returns a fixed list of mock servers.
pub fn fetch_servers() -> Vec<ServerInfo> {
    vec![
        ServerInfo {
            name: "Mumble Public US".to_string(),
            host: "us.mumble.com".to_string(),
            port: 64738,
            users: 128,
            max_users: 200,
        },
        ServerInfo {
            name: "Mumble Public EU".to_string(),
            host: "eu.mumble.com".to_string(),
            port: 64738,
            users: 215,
            max_users: 250,
        },
        ServerInfo {
            name: "Community Gaming Server".to_string(),
            host: "voice.example.org".to_string(),
            port: 10020,
            users: 12,
            max_users: 50,
        },
    ]
}
