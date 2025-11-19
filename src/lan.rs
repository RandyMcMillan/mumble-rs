use futures_util::{pin_mut, stream::StreamExt};
use mdns::RecordKind;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct ServerInfo {
    pub name: String,
    pub host: String,
    pub ip: IpAddr,
    pub port: u16,
}

const SERVICE_TYPE: &str = "_mumble._tcp.local.";

pub async fn discover_servers() -> Vec<ServerInfo> {
    let mut servers = HashMap::new();

    if let Ok(stream) = mdns::discover::all(SERVICE_TYPE, Duration::from_secs(1)) {
        let stream = stream.listen();
        pin_mut!(stream);

        let discovery_duration = Duration::from_secs(2);

        let stream_with_timeout = stream.take_until(tokio::time::sleep(discovery_duration));
        pin_mut!(stream_with_timeout);

        while let Some(Ok(response)) = stream_with_timeout.next().await {
            let mut host = None;
            let mut port = None;
            let mut ip = None;
            let mut name = None;

            for record in response.records() {
                match &record.kind {
                    RecordKind::A(addr) => ip = Some(IpAddr::V4(*addr)),
                    RecordKind::AAAA(addr) => ip = Some(IpAddr::V6(*addr)),
                    RecordKind::SRV {
                        port: srv_port,
                        target,
                        ..
                    } => {
                        port = Some(*srv_port);
                        host = Some(target.to_string());
                    }
                    RecordKind::PTR(ptr_name) => {
                        name = Some(ptr_name.to_string());
                    }
                    _ => {}
                }
            }

            if let (Some(name), Some(host), Some(ip), Some(port)) = (name, host, ip, port) {
                let server_info = ServerInfo {
                    name,
                    host,
                    ip,
                    port,
                };
                servers.insert(server_info.host.clone(), server_info);
            }
        }
    }

    servers.into_values().collect()
}
