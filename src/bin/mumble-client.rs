use anyhow::Result;
use mumble::lan;
use mumble::ui::client::Tui;

#[tokio::main]
async fn main() -> Result<()> {
    let servers = lan::fetch_servers();
    let mut tui = Tui::new(servers)?;
    tui.run()?;
    Ok(())
}