use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub users: u32,
    pub max_users: u32,
}

const PUBLIC_SERVER_LIST_URL: &str = "https://servers.mumble.info/v1/list";

pub async fn fetch_servers() -> Result<Vec<ServerInfo>, reqwest::Error> {
    let response = reqwest::get(PUBLIC_SERVER_LIST_URL).await?;
    let servers = response.json::<Vec<ServerInfo>>().await?;
    Ok(servers)
}
