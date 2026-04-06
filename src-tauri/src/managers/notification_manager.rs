use tauri::{AppHandle, Manager};
use tauri_plugin_notification::NotificationExt;

pub struct NotificationManager {
    app_handle: AppHandle,
}

impl NotificationManager {
    pub fn new(app_handle: AppHandle) -> Self {
        Self { app_handle }
    }

    pub fn show_notification(
        &self,
        title: &str,
        body: &str,
        service_id: Option<&str>,
    ) -> Result<(), String> {
        let mut notification = self
            .app_handle
            .notification()
            .builder()
            .title(title)
            .body(body);

        // El método id() espera i32, convertimos usando un hash simple (no crítico)
        if let Some(id) = service_id {
            let num_id = id.len() as i32; // alternativo: usar un hash
            notification = notification.id(num_id);
        }

        notification.show().map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn set_badge_count(&self, count: u32) -> Result<(), String> {
        if let Some(window) = self.app_handle.get_webview_window("main") {
            // set_badge_count espera Option<i64>
            window
                .set_badge_count(Some(count as i64))
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}
