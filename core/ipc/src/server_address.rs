use local_ip_address::local_ip;
use utils::pkg::CommandData;

#[tauri::command]
pub async fn server() -> Result<CommandData<String>, ()> {
    let ip = local_ip().unwrap();
    let port = 2105;
    let server_address = format!("http://{}:{}", ip, port);
    Ok(CommandData::ok("server address", server_address))
}
