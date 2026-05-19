pub mod permissions;
pub mod scheduler;
pub mod simulator;

use crate::permissions::{get_status, PermissionStatus};
use crate::scheduler::{types::Schedule, Scheduler};
use crate::simulator::get_simulator;
use std::sync::Arc;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{Emitter, Manager, State};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};
use uuid::Uuid;

pub struct AppState {
    pub scheduler: Scheduler,
}

#[tauri::command]
async fn start_schedule(
    schedule: Schedule,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    state.scheduler.start_schedule(schedule, app)
}

#[tauri::command]
async fn stop_schedule(id: Uuid, state: State<'_, AppState>) -> Result<bool, String> {
    Ok(state.scheduler.stop_schedule(&id))
}

#[tauri::command]
async fn pause_schedule(id: Uuid, state: State<'_, AppState>) -> Result<bool, String> {
    Ok(state.scheduler.pause_schedule(&id))
}

#[tauri::command]
async fn resume_schedule(id: Uuid, state: State<'_, AppState>) -> Result<bool, String> {
    Ok(state.scheduler.resume_schedule(&id))
}

#[tauri::command]
async fn stop_all_schedules(state: State<'_, AppState>) -> Result<(), String> {
    state.scheduler.stop_all_schedules();
    Ok(())
}

#[tauri::command]
async fn get_permissions(_state: State<'_, AppState>) -> Result<PermissionStatus, String> {
    Ok(get_status())
}

#[tauri::command]
async fn request_permissions() -> Result<(), String> {
    permissions::request_accessibility();
    Ok(())
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let simulator = Arc::from(get_simulator().expect("failed to initialize simulator"));
    let scheduler = Scheduler::new(simulator);
    let state = AppState { scheduler };

    tauri::Builder::default()
        .manage(state)
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            start_schedule,
            stop_schedule,
            pause_schedule,
            resume_schedule,
            stop_all_schedules,
            get_permissions,
            request_permissions
        ])
        .setup(|app| {
            if let Some(window) = app.get_webview_window("main") {
                if let Ok(Some(monitor)) = window.current_monitor() {
                    let size = monitor.size();
                    if size.width < 1920 {
                        let _ = window.maximize();
                    }
                }
            }

            let panic_shortcut = Shortcut::new(
                Some(
                    tauri_plugin_global_shortcut::Modifiers::CONTROL
                        | tauri_plugin_global_shortcut::Modifiers::SHIFT,
                ),
                tauri_plugin_global_shortcut::Code::KeyX,
            );
            app.global_shortcut()
                .on_shortcut(panic_shortcut, |app, _shortcut, _event| {
                    let state: State<'_, AppState> = app.state();
                    state.scheduler.stop_all_schedules();
                    let _ = app.emit("schedules-stopped", ());
                })?;

            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let show_i = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let stop_all_i =
                MenuItem::with_id(app, "stop_all", "Stop All Schedules", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &stop_all_i, &quit_i])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    "show" => {
                        let window = app.get_webview_window("main").unwrap();
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                    "stop_all" => {
                        let state: State<'_, AppState> = app.state();
                        state.scheduler.stop_all_schedules();
                        let _ = app.emit("schedules-stopped", ());
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let tauri::tray::TrayIconEvent::Click {
                        button: tauri::tray::MouseButton::Left,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
