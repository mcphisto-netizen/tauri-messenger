use anyhow::Result;
use std::path::PathBuf;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

pub struct WindowManager {
    app_handle: AppHandle,
}

impl WindowManager {
    pub fn new(app_handle: AppHandle) -> Self {
        Self { app_handle }
    }

    fn get_profile_path(&self, service_id: &str) -> Result<PathBuf> {
        let app_data = self
            .app_handle
            .path()
            .app_data_dir()
            .expect("No se pudo obtener app_data_dir");
        let profile_dir = app_data.join("profiles").join(service_id);
        std::fs::create_dir_all(&profile_dir)?;
        Ok(profile_dir)
    }

    pub async fn create_service_window(
        &self,
        service_id: &str,
        url: &str,
    ) -> Result<WebviewWindow> {
        let profile_path = self.get_profile_path(service_id)?;
        let url_parsed = url
            .parse()
            .map_err(|e| anyhow::anyhow!("URL inválida: {}", e))?;

        let window = WebviewWindowBuilder::new(
            &self.app_handle,
            service_id,
            WebviewUrl::External(url_parsed),
        )
        .inner_size(1200.0, 800.0)
        .data_directory(profile_path)
        .build()?;

        Ok(window)
    }

    pub fn close_service_window(&self, service_id: &str) -> Result<()> {
        if let Some(window) = self.app_handle.get_webview_window(service_id) {
            window.close()?;
        }
        Ok(())
    }

    pub fn hibernate_service(&self, service_id: &str) -> Result<()> {
        self.close_service_window(service_id)
    }

    pub async fn restore_service(&self, service_id: &str, url: &str) -> Result<()> {
        if self.app_handle.get_webview_window(service_id).is_none() {
            self.create_service_window(service_id, url).await?;
        } else {
            let window = self.app_handle.get_webview_window(service_id).unwrap();
            window.show()?;
            window.set_focus()?;
        }
        Ok(())
    }
}
