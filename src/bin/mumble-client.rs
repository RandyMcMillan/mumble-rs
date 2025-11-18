use anyhow::Result;
use mumble::{lan, local};
use mumble::ui::client::Tui;

#[tokio::main]
async fn main() -> Result<()> {
    let servers = lan::fetch_servers();
    let is_local_running = if let Some(_local_server) = local::detect_local_server() {
        // We no longer add the local server to the main list,
        // but we use its detection to set the status for the dedicated widget.
        true
    } else {
        false
    };

    let mut tui = Tui::new(servers, is_local_running)?;
    tui.run()?;
    Ok(())
}