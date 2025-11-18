use anyhow::Result;
use mumble::{
    embed, lan,
    ui::client::{LocalServerState, ServerCommand, Tui},
};
use std::sync::{Arc, Mutex};
use tokio::{
    sync::{mpsc, oneshot},
    task,
};

async fn start_server_task(
    log_buffer: Arc<Mutex<Vec<String>>>,
) -> (task::JoinHandle<Result<()>>, oneshot::Sender<()>) {
    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    let handle = task::spawn(embed::run_embedded_server(log_buffer, shutdown_rx));
    (handle, shutdown_tx)
}

#[tokio::main]
async fn main() -> Result<()> {
    let servers = lan::fetch_servers();
    let (command_tx, mut command_rx) = mpsc::channel(10);
    let server_log_buffer = Arc::new(Mutex::new(Vec::new()));

    let mut tui = Tui::new(servers, Arc::clone(&server_log_buffer), command_tx)?;

    let mut server_handle: Option<task::JoinHandle<Result<()>>> = None;
    let mut shutdown_tx: Option<oneshot::Sender<()>> = None;
    let mut stopping_handle: Option<task::JoinHandle<Result<()>>> = None;

    loop {
        // --- Step 1: Check for and finalize any pending shutdowns ---
        if let Some(handle) = &stopping_handle {
            if handle.is_finished() {
                let handle = stopping_handle.take().unwrap();
                handle.await??; // Await is now non-blocking and cleans up the task

                if tui.app_state.local_server_state == LocalServerState::Stopping {
                    tui.app_state.local_server_state = LocalServerState::Stopped;
                    tui.app_state.log("[INFO] Server stopped.".to_string());
                } else if tui.app_state.local_server_state == LocalServerState::Restarting {
                    tui.app_state.log("[INFO] Server stopped. Starting again...".to_string());
                    tui.app_state.local_server_state = LocalServerState::Starting;
                    tui.draw()?; // Redraw to show "Starting"

                    let (handle_new, tx_new) = start_server_task(Arc::clone(&server_log_buffer)).await;
                    server_handle = Some(handle_new);
                    shutdown_tx = Some(tx_new);
                    tui.app_state.local_server_state = LocalServerState::Running;
                    tui.app_state.log("[INFO] Server restarted successfully.".to_string());
                }
            }
        }

        // --- Step 2: Handle commands from the TUI ---
        if stopping_handle.is_none() { // Don't process new commands while a stop is pending
            if let Ok(command) = command_rx.try_recv() {
                match command {
                    ServerCommand::Start => {
                        if server_handle.is_none() {
                            tui.app_state.local_server_state = LocalServerState::Starting;
                            tui.app_state.log("[CMD] Starting server...".to_string());
                            tui.draw()?; // Redraw to show "Starting"

                            let (handle, tx) = start_server_task(Arc::clone(&server_log_buffer)).await;
                            server_handle = Some(handle);
                            shutdown_tx = Some(tx);
                            tui.app_state.local_server_state = LocalServerState::Running;
                            tui.app_state.log("[INFO] Server started successfully.".to_string());
                        }
                    }
                    ServerCommand::Stop => {
                        if let Some(tx) = shutdown_tx.take() {
                            tui.app_state.local_server_state = LocalServerState::Stopping;
                            tui.app_state.log("[CMD] Stopping server...".to_string());
                            tx.send(()).ok();
                            stopping_handle = server_handle.take();
                        }
                    }
                    ServerCommand::Restart => {
                        if let Some(tx) = shutdown_tx.take() {
                            tui.app_state.local_server_state = LocalServerState::Restarting;
                            tui.app_state.log("[CMD] Restarting server...".to_string());
                            tx.send(()).ok();
                            stopping_handle = server_handle.take();
                        }
                    }
                }
            }
        }

        // --- Step 3: Draw the UI and handle input ---
        tui.draw()?;
        if tui.handle_events()? {
            break; // Exit loop if 'q' is pressed
        }

        // Give the OS a little time to breathe
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }

    // --- Final cleanup on exit ---
    if let Some(tx) = shutdown_tx.take() {
        tx.send(()).ok();
        if let Some(handle) = server_handle.take() {
            handle.await??;
        }
    }

    Ok(())
}