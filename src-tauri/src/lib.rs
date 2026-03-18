mod api_types;
mod clipboard;
mod server;
mod settings;

use std::sync::Arc;

use settings::{AppState, Settings};
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};
use tauri_plugin_autostart::ManagerExt;
use tokio::sync::Mutex;

#[tauri::command]
async fn get_settings(
    state: tauri::State<'_, Arc<AppState>>,
) -> Result<Settings, String> {
    let settings = state.settings.lock().await;
    Ok(settings.clone())
}

#[tauri::command]
async fn save_settings(
    app: tauri::AppHandle,
    state: tauri::State<'_, Arc<AppState>>,
    new_settings: Settings,
) -> Result<(), String> {
    // Update autostart
    let autostart = app.autolaunch();
    if new_settings.auto_start {
        autostart.enable().map_err(|e| format!("Failed to enable autostart: {e}"))?;
    } else {
        autostart.disable().map_err(|e| format!("Failed to disable autostart: {e}"))?;
    }

    settings::save_settings(&new_settings)?;
    let mut current = state.settings.lock().await;
    *current = new_settings;
    Ok(())
}

#[tauri::command]
async fn get_server_status(
    state: tauri::State<'_, Arc<AppState>>,
) -> Result<api_types::ServerStatus, String> {
    let settings = state.settings.lock().await;
    let shutdown = state.server_shutdown_tx.lock().await;
    let local_ip = local_ip_address::local_ip()
        .map(|ip| ip.to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    Ok(api_types::ServerStatus {
        running: shutdown.is_some(),
        port: settings.port,
        local_ip,
    })
}

#[tauri::command]
async fn restart_server(
    state: tauri::State<'_, Arc<AppState>>,
) -> Result<(), String> {
    // Shutdown existing server
    {
        let mut shutdown = state.server_shutdown_tx.lock().await;
        if let Some(tx) = shutdown.take() {
            tx.send(()).ok();
        }
    }

    // Start new server
    let new_tx = server::start_server(Arc::clone(&state)).await?;
    {
        let mut shutdown = state.server_shutdown_tx.lock().await;
        *shutdown = Some(new_tx);
    }

    Ok(())
}

#[tauri::command]
async fn regenerate_api_key(
    state: tauri::State<'_, Arc<AppState>>,
) -> Result<String, String> {
    let new_key = settings::regenerate_key();
    let mut current = state.settings.lock().await;
    current.api_key = new_key.clone();
    settings::save_settings(&current)?;
    Ok(new_key)
}

#[tauri::command]
fn get_local_ip() -> Result<String, String> {
    local_ip_address::local_ip()
        .map(|ip| ip.to_string())
        .map_err(|e| format!("Failed to get local IP: {e}"))
}

#[tauri::command]
fn get_clipboard_text() -> Result<String, String> {
    clipboard::read_clipboard_text()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let initial_settings = settings::load_settings();
    let app_state = Arc::new(AppState {
        settings: Arc::new(Mutex::new(initial_settings)),
        server_shutdown_tx: Mutex::new(None),
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .manage(app_state.clone())
        .invoke_handler(tauri::generate_handler![
            get_settings,
            save_settings,
            get_server_status,
            restart_server,
            regenerate_api_key,
            get_local_ip,
            get_clipboard_text,
        ])
        .setup(move |app| {
            // System tray
            let show_item =
                MenuItem::with_id(app, "show", "Settings", true, None::<&str>)?;
            let quit_item =
                MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let tray_menu = Menu::with_items(app, &[&show_item, &quit_item])?;

            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&tray_menu)
                .tooltip("Cross-Paste")
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            window.show().ok();
                            window.set_focus().ok();
                        }
                    }
                    "quit" => {
                        std::process::exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            window.show().ok();
                            window.set_focus().ok();
                        }
                    }
                })
                .build(app)?;

            // Start HTTP server
            let state = app_state.clone();
            tauri::async_runtime::spawn(async move {
                match server::start_server(state.clone()).await {
                    Ok(tx) => {
                        let mut shutdown = state.server_shutdown_tx.lock().await;
                        *shutdown = Some(tx);
                    }
                    Err(e) => {
                        eprintln!("Failed to start HTTP server: {e}");
                    }
                }
            });

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                window.hide().ok();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
