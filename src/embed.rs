use anyhow::{anyhow, Result};
use log::info;
use rusqlite::Connection as SqliteConnection;
use rustls::pki_types::PrivateKeyDer;
use rustls::ServerConfig;
use std::sync::Arc;
use tokio::sync::oneshot;
use tokio_rustls::TlsAcceptor;

use crate::cli::Config;
use crate::config::{DbConnectionParameter, MetaParams};
use crate::db;
use crate::server::Meta;

fn get_db_connection_parameter(params: &MetaParams) -> Result<DbConnectionParameter> {
    match params.db_driver.as_str() {
        "QSQLITE" => Ok(DbConnectionParameter::SQLite {
            path: params.database.clone(),
            use_wal: params.sqlite_wal == 1,
        }),
        _ => Err(anyhow!("Unsupported database driver for embedded server: {}", params.db_driver)),
    }
}

/// Runs an embedded Mumble server instance.
///
/// This function encapsulates the entire server startup and execution logic.
/// It takes a `shutdown_rx` channel to allow for graceful termination.
pub async fn run_embedded_server(
    config: Config,
    mut shutdown_rx: oneshot::Receiver<()>,
) -> Result<()> {
    let mut params = config.params;

    // Default certificate paths for embedded server
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

    let db_connection_param = get_db_connection_parameter(&params)?;
    let db_connection = match db_connection_param {
        DbConnectionParameter::SQLite { path, use_wal } => {
            let conn = SqliteConnection::open(&path)?;
            if use_wal {
                conn.pragma_update(None, "journal_mode", "WAL")?;
            }
            db::initialize_database(&conn)?;
            conn
        }
        _ => unreachable!(),
    };

    let mut meta = Meta::new(params, db_connection);
    info!("Mumble API initialized for embedded server.");
    meta.boot_all(true)?;

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
