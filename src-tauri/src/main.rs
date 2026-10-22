#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{Manager, State};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Device {
    name: String,
    address: String,
    device_type: String, // "bluetooth" or "2.4g"
    battery_level: Option<u8>,
}

#[derive(Default)]
struct AppState {
    devices: Mutex<Vec<Device>>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn scan_devices(state: State<'_, AppState>) -> Result<Vec<Device>, String> {
    let session = bluer::Session::new().await.map_err(|e| e.to_string())?;
    let adapter = session.default_adapter().await.map_err(|e| e.to_string())?;
    adapter.set_powered(true).await.map_err(|e| e.to_string())?;

    let mut devices = Vec::new();

    // Scan for Bluetooth devices
    let device_ids = adapter.device_addresses().await.map_err(|e| e.to_string())?;
    for addr in device_ids {
        if let Ok(device) = adapter.device(addr) {
            let name = device.name().await.unwrap_or_else(|_| "Unknown".to_string());
            let address = addr.to_string();
            let battery_level = device.battery_level().await.ok();

            devices.push(Device {
                name,
                address,
                device_type: "bluetooth".to_string(),
                battery_level,
            });
        }
    }

    // For 2.4G devices, we'll simulate with a placeholder since actual
    // 2.4G scanning depends on specific receiver hardware/driver
    devices.push(Device {
        name: "Example 2.4G Device".to_string(),
        address: "2.4G-001".to_string(),
        device_type: "2.4g".to_string(),
        battery_level: Some(85),
    });

    *state.devices.lock().unwrap() = devices.clone();
    Ok(devices)
}

#[tauri::command]
fn get_added_devices(state: State<'_, AppState>) -> Vec<Device> {
    state.devices.lock().unwrap().clone()
}

fn main() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![scan_devices, get_added_devices])
        .setup(|app| {
            // Tray icon setup
            use tauri::TrayIconBuilder;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .on_tray_icon_event(|tray, event| {
                    if let tauri::TrayIconEvent::Click {
                        button: tauri::MouseButton::Left,
                        button_state: tauri::MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_window("main") {
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
