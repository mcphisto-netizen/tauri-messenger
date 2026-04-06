use anyhow::Result;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Service {
    pub id: String,
    pub name: String,
    pub url: String,
    pub icon: Option<String>,
    pub order: i32,
    pub custom_css: Option<String>,
    pub custom_js: Option<String>,
    pub user_agent: Option<String>,
    pub zoom: f64,
    pub notifications_enabled: bool,
    pub is_active: bool,
}

#[allow(dead_code)]
pub struct ServiceManager {
    app_handle: AppHandle,
}

impl ServiceManager {
    pub fn new(app_handle: AppHandle) -> Self {
        Self { app_handle }
    }

    pub async fn get_all_services(&self) -> Result<Vec<Service>> {
        Ok(vec![])
    }

    pub async fn add_service(&self, service: Service) -> Result<Service> {
        Ok(service)
    }

    pub async fn update_service(&self, _service: Service) -> Result<()> {
        Ok(())
    }

    pub async fn delete_service(&self, _id: &str) -> Result<()> {
        Ok(())
    }
}
