#[tauri::command]
fn server_address(name: &str) -> Result<CommandData> {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
