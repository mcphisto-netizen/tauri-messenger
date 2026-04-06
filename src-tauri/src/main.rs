// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod managers;

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::default().build())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            let app_handle = app.handle().clone();

            let service_manager = managers::service_manager::ServiceManager::new(app_handle.clone());
            let window_manager = managers::window_manager::WindowManager::new(app_handle.clone());
            let notification_manager = managers::notification_manager::NotificationManager::new(app_handle.clone());

            app.manage(service_manager);
            app.manage(window_manager);
            app.manage(notification_manager);

            Ok(())  // ← Esto es obligatorio
        })
        .invoke_handler(tauri::generate_handler![
            commands::service::get_services,
            commands::service::add_service,
            commands::service::update_service,
            commands::service::delete_service,
            commands::window::create_service_window,
            commands::window::close_service_window,
            commands::window::hibernate_service,
            commands::window::restore_service,
            commands::notification::show_notification,
            commands::notification::set_badge_count,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}