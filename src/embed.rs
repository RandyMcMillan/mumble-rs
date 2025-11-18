use crate::cli;
use crate::config::MetaParams;
use crate::db;
use crate::server::Meta;
use anyhow::{anyhow, Result};
use env_logger::Builder;
use log::{info, LevelFilter};
use rustls::pki_types::PrivateKeyDer;
use rustls::ServerConfig;
use std::io::Write;
use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;
use tokio_rusqlite::Connection;
use tokio_rustls::TlsAcceptor;

async fn get_db_connection(params: &MetaParams) -> Result<Connection> {
    match params.db_driver.as_str() {
        "QSQLITE" => {
            let conn = Connection::open(&params.database).await?;
            if params.sqlite_wal == 1 {
                conn.call(|conn| conn.pragma_update(None, "journal_mode", "WAL").map_err(Into::into)).await?;
            }
            Ok(conn)
        }
        _ => Err(anyhow!("Unsupported database driver for embedded server: {}", params.db_driver)),
    }
}

/// Runs an embedded Mumble server instance.
pub async fn run_embedded_server(
    log_buffer: Arc<Mutex<Vec<String>>>,
    mut shutdown_rx: oneshot::Receiver<()>,
) -> Result<()> {
    let config = cli::load_and_merge_config();
    let mut params = config.params;

    // Configure logger to write to the shared buffer
    let log_buffer_clone = Arc::clone(&log_buffer);
    let mut builder = Builder::new();
    builder
        .format(move |_buf, record| {
            let msg = format!("[{}] {}", record.level(), record.args());
            log_buffer_clone.lock().unwrap().push(msg);
            Ok(())
        })
        .filter(None, LevelFilter::Info) // Or use config.logging
        .try_init()
        .ok(); // Ignore error if logger is already set (e.g. in tests)

    if params.ssl_cert.is_empty() {
        params.ssl_cert = "mumble-server.pem".to_string();
    }
    if params.ssl_key.is_empty() {
        params.ssl_key = "mumble-server.key".to_string();
    }

    info!("Starting embedded Mumble server...");

    let cert_file = params.ssl_cert.clone();
    let key_file = params.ssl_key.clone();

    if !std::path::Path::new(&cert_file).exists() || !std::path::Path::new(&key_file).exists() {
        return Err(anyhow!("Certificate or key file not found. Please generate them first."));
    }

    let certs = rustls_pemfile::certs(&mut std::io::BufReader::new(std::fs::File::open(&cert_file)?))
        .collect::<Result<Vec<_>, _>>()?;
    let mut keys = rustls_pemfile::pkcs8_private_keys(&mut std::io::BufReader::new(std::fs::File::open(&key_file)?))
        .collect::<Result<Vec<_>, _>>()?;
    let key = keys.pop().ok_or_else(|| anyhow!("No private key found"))?;

    let tls_config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, PrivateKeyDer::Pkcs8(key))?;
    let acceptor = TlsAcceptor::from(Arc::new(tls_config));

    info!("SSL/TLS initialized for embedded server.");

    let db_connection = get_db_connection(&params).await?;
    db::initialize_database(&db_connection).await?;

    let mut meta = Meta::new(params, db_connection);
    info!("Mumble API initialized for embedded server.");
    meta.boot_all(true).await?;

    info!("Embedded server booted.");

    tokio::select! {
        res = meta.start_server(acceptor) => {
            if let Err(e) = res {
                log::error!("Embedded server stopped with error: {}", e);
            } else {
                info!("Embedded server stopped.");
            }
        }
        _ = &mut shutdown_rx => {
            info!("Shutdown signal received, stopping embedded server.");
        }
    }

    Ok(())
}