use anyhow::Result;
use mumble::{
    cli, embed, lan,
    ui::client::{LocalServerState, ServerCommand, Tui},
};
use tokio::{
    sync::{mpsc, oneshot},
    task,
};

async fn start_server_task(
) -> (task::JoinHandle<Result<()>>, oneshot::Sender<()>) {
    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    let handle = task::spawn(async move {
        let config = cli::load_and_merge_config();
        embed::run_embedded_server(config, shutdown_rx).await
    });
    (handle, shutdown_tx)
}

#[tokio::main]
async fn main() -> Result<()> {
    let servers = lan::fetch_servers();
    let (command_tx, mut command_rx) = mpsc::channel(10);

    let mut tui = Tui::new(servers, command_tx)?;

    let mut server_handle: Option<task::JoinHandle<Result<()>>> = None;
    let mut shutdown_tx: Option<oneshot::Sender<()>> = None;

    loop {
        tui.draw()?;
        if tui.handle_events()? {
            break;
        }

        if let Ok(command) = command_rx.try_recv() {
            match command {
                ServerCommand::Start => {
                    if server_handle.is_none() {
                        tui.app_state.local_server_state = LocalServerState::Starting;
                        tui.draw()?;
                        let (handle, tx) = start_server_task().await;
                        server_handle = Some(handle);
                        shutdown_tx = Some(tx);
                        tui.app_state.local_server_state = LocalServerState::Running;
                    }
                }
                ServerCommand::Stop => {
                    if let Some(tx) = shutdown_tx.take() {
                        tui.app_state.local_server_state = LocalServerState::Stopping;
                        tui.draw()?;
                        tx.send(()).ok();
                        if let Some(handle) = server_handle.take() {
                            handle.await??;
                        }
                        tui.app_state.local_server_state = LocalServerState::Stopped;
                    }
                }
                ServerCommand::Restart => {
                    if let Some(tx) = shutdown_tx.take() {
                        tui.app_state.local_server_state = LocalServerState::Stopping;
                        tui.draw()?;
                        tx.send(()).ok();
                        if let Some(handle) = server_handle.take() {
                            handle.await??;
                        }
                        
                        tui.app_state.local_server_state = LocalServerState::Starting;
                        tui.draw()?;
                        let (handle, tx_new) = start_server_task().await;
                        server_handle = Some(handle);
                        shutdown_tx = Some(tx_new);
                        tui.app_state.local_server_state = LocalServerState::Running;
                    }
                }
            }
        }
    }

    // Ensure server is stopped on exit
    if let Some(tx) = shutdown_tx.take() {
        tx.send(()).ok();
        if let Some(handle) = server_handle.take() {
            handle.await??;
        }
    }

    Ok(())
}
