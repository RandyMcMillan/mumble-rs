use crate::config::MetaParams;
use anyhow::Result;
use log::{info, warn};
use std::collections::HashMap;
use tokio::net::{TcpListener, TcpStream};
use tokio_rustls::TlsAcceptor;
use tokio_rusqlite::Connection as TokioConnection;

#[derive(Clone)]
pub struct ServerParams {
    pub port: u16,
}

pub struct Server {
    pub params: ServerParams,
}

impl Server {
    pub fn new(params: ServerParams) -> Self {
        Self { params }
    }

    pub async fn run(&self, acceptor: TlsAcceptor) -> Result<()> {
        let addr = format!("0.0.0.0:{}", self.params.port);
        let listener = TcpListener::bind(&addr).await?;
        info!("Server listening on {}", addr);

        loop {
            let (stream, _peer_addr) = listener.accept().await?;
            let acceptor = acceptor.clone();
            tokio::spawn(async move {
                if let Err(err) = handle_connection(stream, acceptor).await {
                    warn!("Connection error: {:?}", err);
                }
            });
        }
    }
}

async fn handle_connection(stream: TcpStream, acceptor: TlsAcceptor) -> Result<()> {
    let _tls_stream = acceptor.accept(stream).await?;
    // TODO: Handle Mumble protocol
    Ok(())
}

pub struct Meta {
    pub params: MetaParams,
    pub db_connection: TokioConnection,
    pub servers: HashMap<u32, Server>,
}

impl Meta {
    pub fn new(params: MetaParams, db_connection: TokioConnection) -> Self {
        Self {
            params,
            db_connection,
            servers: HashMap::new(),
        }
    }

    pub async fn boot_all(&mut self, boot: bool) -> Result<()> {
        if boot {
            let server_ids: Vec<u32> = self.db_connection.call(|conn| {
                let mut stmt = conn.prepare("SELECT server_id FROM servers WHERE boot = 1")?;
                let ids = stmt.query_map([], |row| row.get(0))?.collect::<Result<Vec<u32>, _>>()?;
                Ok(ids)
            }).await?;

            for id in server_ids {
                self.boot(id).await?;
            }
        }
        Ok(())
    }

    pub async fn add_server(&self) -> Result<u32> {
        let id = self.db_connection.call(|conn| {
            conn.execute("INSERT INTO servers (boot) VALUES (0)", [])?;
            Ok(conn.last_insert_rowid() as u32)
        }).await?;
        Ok(id)
    }

    pub async fn is_booted(&self, srvnum: u32) -> Result<bool> {
        let count: i64 = self.db_connection.call(move |conn| {
            let mut stmt = conn.prepare("SELECT COUNT(*) FROM servers WHERE server_id = ?1")?;
            let count = stmt.query_row([srvnum], |row| row.get(0))?;
            Ok(count)
        }).await?;
        Ok(count > 0 && self.servers.contains_key(&srvnum))
    }

    pub async fn boot(&mut self, srvnum: u32) -> Result<()> {
        if self.is_booted(srvnum).await? {
            return Ok(());
        }

        let params = ServerParams {
            port: self.params.port,
            // ... other params ...
        };
        let s = Server::new(params);
        self.servers.insert(srvnum, s);
        Ok(())
    }

    pub async fn start_server(&self, acceptor: TlsAcceptor) -> Result<()> {
        // For now, we'll just run the first server.
        // A real implementation would manage multiple servers.
        if let Some(server) = self.servers.values().next() {
            server.run(acceptor).await?;
        } else {
            warn!("No servers booted, nothing to start.");
        }
        Ok(())
    }
}