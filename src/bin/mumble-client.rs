use anyhow::Result;
use mumble::ui::Tui;

#[tokio::main]
async fn main() -> Result<()> {
    let mut tui = Tui::new()?;
    tui.run()?;
    Ok(())
}