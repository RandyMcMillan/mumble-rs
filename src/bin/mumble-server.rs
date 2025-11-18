#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use anyhow::{anyhow, Result};
use ed25519_dalek::SigningKey;
use env_logger::Builder;
use log::{info, LevelFilter};
use mumble::cli;
use mumble::config::{DbConnectionParameter, MetaParams};
use mumble::db;
use mumble::server::Meta;
use mumble::tui::Tui;
use pkcs8::EncodePrivateKey;
use rcgen::{
    generate_simple_self_signed, CertificateParams, DistinguishedName, KeyPair, PKCS_ED25519,
};
use rusqlite::Connection as SqliteConnection;
use rustls::pki_types::PrivateKeyDer;
use rustls::ServerConfig;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::signal::unix::{signal, SignalKind};
use tokio_rustls::TlsAcceptor;

fn generate_cert(cert_path: &str, key_path: &str, hash_seed: Option<&str>) -> Result<()> {
    if let Some(hash) = hash_seed {
        info!("Generating certificate from SHA256 hash...");
        let seed_bytes = hex::decode(hash)?;
        if seed_bytes.len() != 32 {
            return Err(anyhow!(
                "SHA256 hash must be 32 bytes (64 hex characters) long."
            ));
        }
        let seed_array: [u8; 32] = seed_bytes.try_into().unwrap();
        let secret_key = SigningKey::from_bytes(&seed_array);
        let pkcs8_der = secret_key
            .to_pkcs8_der()
            .map_err(|e| anyhow!("Failed to create PKCS#8 DER from secret key: {}", e))?;

        let key_pair = KeyPair::from_der_and_sign_algo(
            &PrivateKeyDer::Pkcs8(pkcs8_der.as_bytes().into()),
            &PKCS_ED25519,
        )?;
        let mut params = CertificateParams::new(vec!["localhost".to_string()])?;
        let mut dn = DistinguishedName::new();
        dn.push(rcgen::DnType::CommonName, "localhost");
        params.distinguished_name = dn;

        let cert = params.self_signed(&key_pair)?;
        let cert_pem = cert.pem();
        let key_pem = key_pair.serialize_pem();

        std::fs::write(cert_path, cert_pem)?;
        std::fs::write(key_path, key_pem)?;
        info!(
            "Certificate and key saved to {} and {}",
            cert_path, key_path
        );
        return Ok(());
    }

    info!("Generating new self-signed certificate...");
    let subject_alt_names = vec!["localhost".to_string()];
    let cert = generate_simple_self_signed(subject_alt_names)?;
    let cert_pem = cert.cert.pem();
    let key_pem = cert.signing_key.serialize_pem();
    std::fs::write(cert_path, cert_pem)?;
    std::fs::write(key_path, key_pem)?;
    info!(
        "Certificate and key saved to {} and {}",
        cert_path, key_path
    );
    Ok(())
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
    let config = cli::load_and_merge_config();
    let mut params = config.params;

    let log_messages = Arc::new(Mutex::new(Vec::new()));

    if config.tui {
        let log_messages_clone = Arc::clone(&log_messages);
        let mut builder = Builder::new();
        builder
            .format(move |buf, record| {
                let msg = format!("[{}] {}", record.level(), record.args());
                log_messages_clone.lock().unwrap().push(msg.clone());
                writeln!(buf, "{}", msg)
            })
            .filter(None, LevelFilter::Info)
            .init();
    } else {
        env_logger::init();
    }

    if config.generate_cert || config.generate_keys {
        let mut cert_file = params.ssl_cert.clone();
        let mut key_file = params.ssl_key.clone();
        if cert_file.is_empty() {
            cert_file = "mumble-server.pem".to_string();
            info!("'sslCert' not set, using default: {}", cert_file);
        }
        if key_file.is_empty() {
            key_file = "mumble-server.key".to_string();
            info!("'sslKey' not set, using default: {}", key_file);
        }
        generate_cert(&cert_file, &key_file, config.key_from_hash.as_deref())?;
        return Ok(());
    }

    if params.ssl_cert.is_empty() {
        params.ssl_cert = "mumble-server.pem".to_string();
    }
    if params.ssl_key.is_empty() {
        params.ssl_key = "mumble-server.key".to_string();
    }

    info!("Starting Mumble server...");

    let cert_file = params.ssl_cert.clone();
    let key_file = params.ssl_key.clone();


    info!(
        "Server configured with port: {} and welcome text: {}",
        params.port, params.welcome_text
    );

    // Load SSL/TLS certificate and key from files
    if cert_file.is_empty() || key_file.is_empty() {
        return Err(anyhow!(
            "'sslCert' and 'sslKey' must be set in the config file or via command line arguments."
        ));
    }

    if !std::path::Path::new(&cert_file).exists() || !std::path::Path::new(&key_file).exists() {
        return Err(anyhow!("Certificate or key file not found. Use --generate-cert or --generate-keys to create them."));
    }

    let certs = rustls_pemfile::certs(&mut std::io::BufReader::new(
        std::fs::File::open(&cert_file)
            .map_err(|e| anyhow!("Failed to open cert file '{}': {}", &cert_file, e))?,
    ))
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| anyhow!("Failed to parse cert file '{}': {}", &cert_file, e))?;

    let mut keys = rustls_pemfile::pkcs8_private_keys(&mut std::io::BufReader::new(
        std::fs::File::open(&key_file)
            .map_err(|e| anyhow!("Failed to open key file '{}': {}", &key_file, e))?,
    ))
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
    let db_connection_param =
        get_db_connection_parameter(&params).expect("Failed to get DB connection parameters");

    // Establish database connection
    let db_connection = match db_connection_param {
        DbConnectionParameter::SQLite { path, use_wal } => {
            let conn = SqliteConnection::open(&path).expect("Failed to open SQLite database");
            if use_wal {
                conn.pragma_update(None, "journal_mode", "WAL")
                    .expect("Failed to set WAL mode");
            }
            db::initialize_database(&conn)?;
            conn
        }
        DbConnectionParameter::MySQL { .. } => {
            // TODO: Implement MySQL connection
            panic!("MySQL not yet implemented");
        }
        DbConnectionParameter::PostgreSQL { .. } => {
            // TODO: Implement PostgreSQL connection
            panic!("PostgreSQL not yet implemented");
        }
    };

    // Initialize Meta
    let mut meta = Meta::new(params, db_connection);
    info!("Mumble API initialized.");

    // Boot all servers
    meta.boot_all(true).expect("Failed to boot servers");

    // All startup checks passed, now we can start the TUI if requested
    if config.tui {
        let tui_log_messages = Arc::clone(&log_messages);
        thread::spawn(move || {
            let mut tui = Tui::new(tui_log_messages).unwrap();
            tui.run().unwrap();
        });
    }

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