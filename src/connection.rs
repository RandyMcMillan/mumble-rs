use crate::ui::client::ConnectionInfo;
use anyhow::Result;

pub async fn connect_to_server(info: ConnectionInfo) -> Result<()> {
    println!("[CONNECTION] Attempting to connect to {}:{}", info.host, info.port);
    // In the future, this will contain the actual Mumble connection logic.
    Ok(())
}
