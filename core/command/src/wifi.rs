use local_ip_address::local_ip;
use utils::pkg::CommandData;

#[tauri::command]
pub fn is_connected_to_wifi() -> CommandData<bool> {
    // the app would have a local ip address if it is connected to a network
    // else it would crash, this is leveraged to check the network status
    let has_ip_addr = local_ip().ok();
    if has_ip_addr.is_none() {
        return CommandData::ok("wifi status", false);
    }
    CommandData::ok("server address", true)
}
