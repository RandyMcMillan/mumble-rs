use serde::Deserialize;

include!(concat!(env!("OUT_DIR"), "/public_server_list.rs"));

#[derive(Debug, Clone, Deserialize)]
pub struct ServerInfo {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@ip")]
    pub host: String,
    #[serde(rename = "@port")]
    pub port: u16,
    // These fields are not in the XML, but we keep them for compatibility with the UI
    #[serde(default)]
    pub users: u32,
    #[serde(default)]
    pub max_users: u32,
}

#[derive(Debug, Deserialize)]
struct Servers {
    #[serde(rename = "server", default)]
    servers: Vec<ServerInfo>,
}

pub async fn fetch_servers() -> Result<Vec<ServerInfo>, anyhow::Error> {
    let servers: Servers = quick_xml::de::from_str(PUBLIC_SERVER_LIST_XML)?;
    Ok(servers.servers)
}