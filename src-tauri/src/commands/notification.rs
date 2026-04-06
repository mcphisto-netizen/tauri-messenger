use crate::managers::notification_manager::NotificationManager;
use tauri::State;

#[tauri::command]
pub async fn show_notification(
    manager: State<'_, NotificationManager>,
    title: String,
    body: String,
    service_id: Option<String>,
) -> Result<(), String> {
    manager.show_notification(&title, &body, service_id.as_deref())
}

#[tauri::command]
pub async fn set_badge_count(
    manager: State<'_, NotificationManager>,
    count: u32,
) -> Result<(), String> {
    manager.set_badge_count(count)
}
