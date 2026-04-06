use crate::managers::window_manager::WindowManager;
use tauri::State;

#[tauri::command]
pub async fn create_service_window(
    manager: State<'_, WindowManager>,
    service_id: String,
    url: String,
) -> Result<(), String> {
    manager
        .create_service_window(&service_id, &url)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn close_service_window(
    manager: State<'_, WindowManager>,
    service_id: String,
) -> Result<(), String> {
    manager
        .close_service_window(&service_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn hibernate_service(
    manager: State<'_, WindowManager>,
    service_id: String,
) -> Result<(), String> {
    manager
        .hibernate_service(&service_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn restore_service(
    manager: State<'_, WindowManager>,
    service_id: String,
    url: String,
) -> Result<(), String> {
    manager
        .restore_service(&service_id, &url)
        .await
        .map_err(|e| e.to_string())
}
