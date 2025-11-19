use crate::lan::ServerInfo;
use std::net::{IpAddr, Ipv4Addr, TcpStream};

const LOCAL_SERVER_ADDRESS: &str = "127.0.0.1:64738";

/// Attempts to detect a Mumble server running on the local machine.
///
/// This function tries to establish a TCP connection to the default Mumble port.
/// If successful, it returns a ServerInfo struct for the local server.
pub fn detect_local_server() -> Option<ServerInfo> {
    if TcpStream::connect(LOCAL_SERVER_ADDRESS).is_ok() {
        Some(ServerInfo {
            name: "Local Server".to_string(),
            host: "127.0.0.1".to_string(),
            ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 64738,
        })
    } else {
        None
    }
}
