use crate::managers::service_manager::{Service, ServiceManager};
use tauri::State;

#[tauri::command]
pub async fn get_services(manager: State<'_, ServiceManager>) -> Result<Vec<Service>, String> {
    manager.get_all_services().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_service(
    manager: State<'_, ServiceManager>,
    service: Service,
) -> Result<Service, String> {
    manager
        .add_service(service)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_service(
    manager: State<'_, ServiceManager>,
    service: Service,
) -> Result<(), String> {
    manager
        .update_service(service)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_service(manager: State<'_, ServiceManager>, id: String) -> Result<(), String> {
    manager.delete_service(&id).await.map_err(|e| e.to_string())
}
