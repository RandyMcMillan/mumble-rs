use anyhow::Result;
use mumble::{lan, local};
use mumble::ui::client::Tui;

#[tokio::main]
async fn main() -> Result<()> {
    let mut servers = lan::fetch_servers();

    if let Some(local_server) = local::detect_local_server() {
        servers.insert(0, local_server);
    }

    let mut tui = Tui::new(servers)?;
    tui.run()?;
    Ok(())
}