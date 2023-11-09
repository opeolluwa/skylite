use utils::pkg::CommandData;

#[tauri::command]
pub fn is_connected_to_wifi() -> Result<CommandData<bool>, ()> {
    // do business logic to see if device is connected to wifi
    Ok(CommandData::ok("server address", true))
}
