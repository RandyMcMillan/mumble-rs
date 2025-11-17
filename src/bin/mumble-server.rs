#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use tokio_rustls::TlsAcceptor;
use rustls::ServerConfig;
use rustls::pki_types::PrivateKeyDer;
use std::sync::Arc;
use anyhow::{Result, anyhow};
use rusqlite::Connection as SqliteConnection;
use std::collections::HashMap;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use log::{info, error};
use env_logger;
use tokio::signal::unix::{signal, SignalKind};
use mumble::config::{MetaParams, DbConnectionParameter};
use mumble::cli;

// Placeholder for the Server struct
pub struct Server {
    pub id: u32,
    pub is_valid: bool,
    // Add other server-specific fields here
}

impl Server {
    pub fn new(id: u32) -> Self {
        // In a real implementation, this would load server configuration from DB
        // and perform validation.
        Self { id, is_valid: true }
    }

    pub fn initialize_cert(&self) {
        info!("Server {}: Initializing certificates (placeholder).", self.id);
    }

    pub fn log(&self, message: &str) {
        info!("Server {}: {}", self.id, message);
    }
}

// Equivalent to C++ Meta
pub struct Meta {
    pub params: MetaParams,
    pub db_connection: SqliteConnection, // Using rusqlite for now
    pub servers: HashMap<u32, Server>,
}

impl Meta {
    pub fn new(params: MetaParams, db_connection: SqliteConnection) -> Self {
        Self {
            params,
            db_connection,
            servers: HashMap::new(),
        }
    }

    // Placeholder for dbWrapper.getBootServers()
    fn get_boot_servers(&self) -> Result<Vec<u32>> {
        // For now, return an empty vector, or a default server ID if no servers exist
        let mut stmt = self.db_connection.prepare("SELECT server_id FROM servers WHERE boot = 1")?;
        let server_ids = stmt.query_map([], |row| row.get(0))?.filter_map(|id| id.ok()).collect();
        Ok(server_ids)
    }

    // Placeholder for dbWrapper.addServer()
    fn add_server(&mut self) -> Result<u32> {
        self.db_connection.execute("INSERT INTO servers (boot) VALUES (0)", [])?;
        Ok(self.db_connection.last_insert_rowid() as u32)
    }

    // Placeholder for dbWrapper.setServerBootProperty()
    fn set_server_boot_property(&self, server_id: u32, boot: bool) -> Result<()> {
        let boot_val = if boot { 1 } else { 0 };
        self.db_connection.execute("UPDATE servers SET boot = ?1 WHERE server_id = ?2", &[&boot_val, &(server_id as i64)])?;
        Ok(())
    }

    // Placeholder for dbWrapper.serverExists()
    fn server_exists(&self, server_id: u32) -> Result<bool> {
        let mut stmt = self.db_connection.prepare("SELECT COUNT(*) FROM servers WHERE server_id = ?1")?;
        let count: i64 = stmt.query_row([server_id as i64], |row| row.get(0))?;
        Ok(count > 0)
    }

    pub fn boot_all(&mut self, create_default_instance: bool) -> Result<()> {
        let mut boot_server_ids = self.get_boot_servers()?;

        if boot_server_ids.is_empty() && create_default_instance {
            let new_server_id = self.add_server()?;
            self.set_server_boot_property(new_server_id, true)?;
            boot_server_ids.push(new_server_id);
            info!("Created new server default instance: {}", new_server_id);
        }

        for current_server_id in boot_server_ids {
            self.boot(current_server_id)?;
        }
        Ok(())
    }

    pub fn boot(&mut self, srvnum: u32) -> Result<()> {
        if self.servers.contains_key(&srvnum) {
            info!("Server {} already running.", srvnum);
            return Ok(());
        }

        if !self.server_exists(srvnum)? {
            return Err(anyhow!("Server {} does not exist in database.", srvnum));
        }

        let s = Server::new(srvnum);
        if !s.is_valid {
            return Err(anyhow!("Server {} is not valid.", srvnum));
        }

        self.servers.insert(srvnum, s);
        info!("Server {} started.", srvnum);
        // TODO: Emit started signal

        // TODO: Handle rlimit for file descriptors on Unix-like systems

        Ok(())
    }

    pub async fn start_server(&self, acceptor: TlsAcceptor) -> Result<()> {
        let addr = format!("0.0.0.0:{}", self.params.port);
        let listener = TcpListener::bind(&addr).await?;
        info!("Listening on {}", addr);

        loop {
            let (stream, peer_addr) = listener.accept().await?;
            info!("New connection from: {}", peer_addr);

            let acceptor = acceptor.clone();
            tokio::spawn(async move {
                match acceptor.accept(stream).await {
                    Ok(mut stream) => {
                        info!("TLS handshake successful with {}", peer_addr);
                        // TODO: Handle Mumble protocol communication
                        let mut buf = vec![0; 1024];
                        loop {
                            match stream.read(&mut buf).await {
                                Ok(0) => {
                                    info!("Client {} disconnected.", peer_addr);
                                    break;
                                },
                                Ok(n) => {
                                    let msg = String::from_utf8_lossy(&buf[..n]);
                                    info!("Received from {}: {}", peer_addr, msg);
                                    stream.write_all(b"ACK").await.expect("Failed to write to stream");
                                },
                                Err(e) => {
                                    error!("Error reading from {}: {}", peer_addr, e);
                                    break;
                                }
                            }
                        }
                    },
                    Err(e) => {
                        error!("TLS handshake failed with {}: {}", peer_addr, e);
                    }
                }
            });
        }
    }
}

fn get_db_connection_parameter(params: &MetaParams) -> Result<DbConnectionParameter> {
    match params.db_driver.as_str() {
        "QSQLITE" => Ok(DbConnectionParameter::SQLite {
            path: params.database.clone(),
            use_wal: params.sqlite_wal == 1,
        }),
        "QMYSQL" => Ok(DbConnectionParameter::MySQL {
            db_name: params.database.clone(),
            username: params.db_username.clone(),
            password: params.db_password.clone(),
            host: params.db_hostname.clone(),
            port: params.db_port as u16,
            prefix: params.db_prefix.clone(),
            opts: params.db_opts.clone(),
        }),
        "QPSQL" => Ok(DbConnectionParameter::PostgreSQL {
            db_name: params.database.clone(),
            username: params.db_username.clone(),
            password: params.db_password.clone(),
            host: params.db_hostname.clone(),
            port: params.db_port as u16,
            prefix: params.db_prefix.clone(),
            opts: params.db_opts.clone(),
        }),
        _ => Err(anyhow!("Unsupported database driver: {}", params.db_driver)),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    info!("Starting Mumble server...");

    let config = cli::load_and_merge_config();
    let params = config.params;

    info!("Server configured with port: {} and welcome text: {}", params.port, params.welcome_text);

    // Load SSL/TLS certificate and key from files
    let cert_file = params.ssl_cert.clone();
    let key_file = params.ssl_key.clone();

    if cert_file.is_empty() || key_file.is_empty() {
        return Err(anyhow!("'sslCert' and 'sslKey' must be set in the config file or via command line arguments."));
    }

    let certs = rustls_pemfile::certs(&mut std::io::BufReader::new(std::fs::File::open(&cert_file)
        .map_err(|e| anyhow!("Failed to open cert file '{}': {}", &cert_file, e))?))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| anyhow!("Failed to parse cert file '{}': {}", &cert_file, e))?;

    let mut keys = rustls_pemfile::pkcs8_private_keys(&mut std::io::BufReader::new(std::fs::File::open(&key_file)
        .map_err(|e| anyhow!("Failed to open key file '{}': {}", &key_file, e))?))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| anyhow!("Failed to parse key file '{}': {}", &key_file, e))?;

    let key = if let Some(k) = keys.pop() {
        k
    } else {
        return Err(anyhow!("No private key found in {}", key_file));
    };

    // Create a `ServerConfig` for `rustls`
    let tls_config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, PrivateKeyDer::Pkcs8(key))
        .expect("Failed to create TLS config");

    let acceptor = TlsAcceptor::from(Arc::new(tls_config));

    info!("SSL/TLS initialized.");

    // Get database connection parameters
    let db_connection_param = get_db_connection_parameter(&params).expect("Failed to get DB connection parameters");

    // Establish database connection
    let db_connection = match db_connection_param {
        DbConnectionParameter::SQLite { path, use_wal } => {
            let conn = SqliteConnection::open(&path).expect("Failed to open SQLite database");
            if use_wal {
                conn.execute("PRAGMA journal_mode = WAL", []).expect("Failed to set WAL mode");
            }
            conn
        },
        DbConnectionParameter::MySQL { .. } => {
            // TODO: Implement MySQL connection
            panic!("MySQL not yet implemented");
        },
        DbConnectionParameter::PostgreSQL { .. } => {
            // TODO: Implement PostgreSQL connection
            panic!("PostgreSQL not yet implemented");
        },
    };

    // Initialize Meta
    let mut meta = Meta::new(params, db_connection);
    info!("Mumble API initialized.");

    // Boot all servers
    meta.boot_all(true).expect("Failed to boot servers");

    // Start listening for client connections
    let mut sigint = signal(SignalKind::interrupt())?;
    let mut sigterm = signal(SignalKind::terminate())?;

    tokio::select! {
        _ = meta.start_server(acceptor) => {
            info!("Server stopped unexpectedly.");
        }
        _ = sigint.recv() => {
            info!("SIGINT received, shutting down gracefully...");
        }
        _ = sigterm.recv() => {
            info!("SIGTERM received, shutting down gracefully...");
        }
    }

    info!("Mumble server stopped.");

    Ok(())
}
