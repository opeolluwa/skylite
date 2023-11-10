use std::net::{IpAddr, Ipv4Addr};

use local_ip_address::local_ip;
use utils::pkg::CommandData;

#[tauri::command]
pub fn server() -> CommandData<String> {
    let default_ip_address = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let local_ip = local_ip().unwrap_or(default_ip_address);
    let port = 2105;
    let server_address = format!("http://{}:{}", local_ip, port);
    CommandData::ok("server address", server_address)
}
