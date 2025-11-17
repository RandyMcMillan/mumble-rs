#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use configparser::ini::Ini;
use tokio_rustls::TlsAcceptor;
use rustls::ServerConfig;
use rustls::pki_types::PrivateKeyDer;
use std::sync::Arc;
use anyhow::{Result, anyhow};
use rusqlite::Connection as SqliteConnection;
use std::collections::HashMap;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::fs::File;
use std::io::BufReader;
use log::{info, error};
use env_logger;
use tokio::signal::unix::{signal, SignalKind};

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

// Enum to represent different database connection parameters
pub enum DbConnectionParameter {
    SQLite { path: String, use_wal: bool },
    MySQL { db_name: String, username: String, password: String, host: String, port: u16, prefix: String, opts: String },
    PostgreSQL { db_name: String, username: String, password: String, host: String, port: u16, prefix: String, opts: String },
}

// Equivalent to C++ MetaParams
pub struct MetaParams {
    pub base_path: String,
    pub bind_addresses: Vec<String>,
    pub port: u16,
    pub timeout: i32,
    pub max_bandwidth: i32,
    pub max_users: u32,
    pub max_users_per_channel: u32,
    pub max_listeners_per_channel: i32,
    pub max_listener_proxies_per_user: i32,
    pub default_channel: u32,
    pub remember_channel: bool,
    pub remember_channel_duration: i32,
    pub max_text_message_length: i32,
    pub max_image_message_length: i32,
    pub opus_threshold: i32,
    pub channel_nesting_limit: i32,
    pub channel_count_limit: i32,
    pub legacy_password_hash: bool,
    pub kdf_iterations: i32,
    pub allow_html: bool,
    pub password: String,
    pub welcome_text: String,
    pub welcome_text_file: String,
    pub cert_required: bool,
    pub force_external_auth: bool,
    pub ban_tries: i32,
    pub ban_timeframe: i32,
    pub ban_time: i32,
    pub ban_successful: bool,
    pub database: String,
    pub sqlite_wal: i32,
    pub db_driver: String,
    pub db_username: String,
    pub db_password: String,
    pub db_hostname: String,
    pub db_prefix: String,
    pub db_opts: String,
    pub db_port: i32,
    pub log_days: i32,
    pub obfuscate: i32,
    pub send_version: bool,
    pub allow_ping: bool,
    pub logfile: String,
    pub pid_file: String,
    pub ice_endpoint: String,
    pub ice_secret_read: String,
    pub ice_secret_write: String,
    pub reg_name: String,
    pub reg_password: String,
    pub reg_host: String,
    pub reg_location: String,
    pub reg_web_url: String,
    pub bonjour: bool,
    // qrUserName: QRegularExpression,
    // qrChannelName: QRegularExpression,
    pub message_limit: u32,
    pub message_burst: u32,
    pub plugin_message_limit: u32,
    pub plugin_message_burst: u32,
    pub broadcast_listener_volume_adjustments: bool,
    // qscCert: QSslCertificate,
    // qskKey: QSslKey,
    // qlIntermediates: QList<QSslCertificate>,
    // qlCA: QList<QSslCertificate>,
    // qlCiphers: QList<QSslCipher>,
    // qbaDHParams: QByteArray,
    // qbaPassPhrase: QByteArray,
    pub ciphers: String,
    // qmConfig: QMap<QString, QString>,
    // uiUid, uiGid: unsigned int,
    // qsHome: QString,
    // qsName: QString,
    // m_suggestVersion: Version::full_t,
    pub suggest_positional: Option<bool>,
    pub suggest_push_to_talk: Option<bool>,
    pub log_group_changes: bool,
    pub log_acl_changes: bool,
    pub allow_recording: bool,
    pub rolling_stats_window: u32,
    pub abs_settings_file_path: String,
    pub ssl_cert: String,
    pub ssl_key: String,
    // qsSettings: QSettings,
}

impl Default for MetaParams {
    fn default() -> Self {
        Self {
            base_path: "".to_string(),
            bind_addresses: Vec::new(),
            port: 64738,
            timeout: 5000,
            max_bandwidth: 100000,
            max_users: 100,
            max_users_per_channel: 0,
            max_listeners_per_channel: 0,
            max_listener_proxies_per_user: 0,
            default_channel: 0,
            remember_channel: true,
            remember_channel_duration: 100,
            max_text_message_length: 5000,
            max_image_message_length: 131072,
            opus_threshold: 100,
            channel_nesting_limit: 10,
            channel_count_limit: 50,
            legacy_password_hash: false,
            kdf_iterations: 0,
            allow_html: true,
            password: "".to_string(),
            welcome_text: "<br />Welcome to this Mumble server!<br />".to_string(),
            welcome_text_file: "".to_string(),
            cert_required: false,
            force_external_auth: false,
            ban_tries: 10,
            ban_timeframe: 120,
            ban_time: 300,
            ban_successful: false,
            database: "murmur.sqlite".to_string(),
            sqlite_wal: 1,
            db_driver: "QSQLITE".to_string(),
            db_username: "".to_string(),
            db_password: "".to_string(),
            db_hostname: "".to_string(),
            db_prefix: "".to_string(),
            db_opts: "".to_string(),
            db_port: 0,
            log_days: 31,
            obfuscate: 0,
            send_version: true,
            allow_ping: true,
            logfile: "".to_string(),
            pid_file: "".to_string(),
            ice_endpoint: "tcp -h 127.0.0.1 -p 6502".to_string(),
            ice_secret_read: "".to_string(),
            ice_secret_write: "".to_string(),
            reg_name: "".to_string(),
            reg_password: "".to_string(),
            reg_host: "".to_string(),
            reg_location: "".to_string(),
            reg_web_url: "".to_string(),
            bonjour: false,
            message_limit: 5,
            message_burst: 10,
            plugin_message_limit: 5,
            plugin_message_burst: 10,
            broadcast_listener_volume_adjustments: false,
            ciphers: "TLS_AES_256_GCM_SHA384:TLS_CHACHA20_POLY1305_SHA256:TLS_AES_128_GCM_SHA256:ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384:ECDHE-ECDSA-CHACHA20-POLY1305:ECDHE-RSA-CHACHA20-POLY1305:ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES256-SHA384:ECDHE-RSA-AES256-SHA384:ECDHE-ECDSA-AES128-SHA256:ECDHE-RSA-AES128-SHA256:AES256-GCM-SHA384:AES128-GCM-SHA256:AES256-SHA256:AES128-SHA256".to_string(),
            suggest_positional: None,
            suggest_push_to_talk: None,
            log_group_changes: false,
            log_acl_changes: false,
            allow_recording: true,
            rolling_stats_window: 60,
            abs_settings_file_path: "".to_string(),
            ssl_cert: "".to_string(),
            ssl_key: "".to_string(),
        }
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

fn read_config(file_path: &str) -> Result<MetaParams> {
    let mut config_parser = Ini::new();
    config_parser.read(file_path.to_string()).map_err(|e| anyhow!("Failed to read INI file: {}", e))?;

    let mut params = MetaParams::default();

    let port_result_option_i64 = config_parser.getint("General", "port");
    params.port = match port_result_option_i64 {
        Ok(Some(i)) => i as u16,
        _ => params.port,
    };

    params.welcome_text = config_parser.get("General", "welcometext")
        .unwrap_or(params.welcome_text.clone());

    params.ssl_cert = config_parser.get("General", "sslCert")
        .unwrap_or(params.ssl_cert.clone());
    params.ssl_key = config_parser.get("General", "sslKey")
        .unwrap_or(params.ssl_key.clone());

    // Parse other fields as needed

    Ok(params)
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    info!("Starting Mumble server...");

    // Load configuration from murmur.ini
    let params = read_config("murmur.ini").expect("Failed to load server configuration");
    info!("Server configured with port: {} and welcome text: {}", params.port, params.welcome_text);

    // Load SSL/TLS certificate and key from files
    let cert_file = params.base_path.clone() + "/cert.pem";
    let key_file = params.base_path.clone() + "/key.pem";

    let certs = rustls_pemfile::certs(&mut std::io::BufReader::new(std::fs::File::open(&cert_file)?))
        .collect::<Result<Vec<_>, _>>()?;
    let mut keys = rustls_pemfile::pkcs8_private_keys(&mut std::io::BufReader::new(std::fs::File::open(&key_file)?))
        .collect::<Result<Vec<_>, _>>()?;

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

    info!("SSL/TLS initialized with placeholder certificate.");

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
    info!("Mumble API initialized (placeholder).");

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
