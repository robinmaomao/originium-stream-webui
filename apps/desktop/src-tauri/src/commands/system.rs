use std::io::Write;

use tauri::{AppHandle, Manager, State};
use tauri_plugin_autostart::ManagerExt as AutostartExt;
use tauri_plugin_global_shortcut::GlobalShortcutExt;

use crate::components;
use crate::error::{AppError, AppResult};
use crate::logging;
use crate::models::{BootstrapState, EnvironmentInfo, LogEntry};
use crate::state::AppState;

#[tauri::command]
pub async fn get_bootstrap(state: State<'_, AppState>) -> AppResult<BootstrapState> {
    let config = state.snapshot();
    let components = components::list_installed(&state.store);
    let environment = components::detect_environment(&state.store, &config.settings, &state.app_version);
    let session = state.session.lock().await.snapshot();
    Ok(BootstrapState {
        config,
        components,
        environment,
        session,
    })
}

#[tauri::command]
pub async fn get_environment(state: State<'_, AppState>) -> AppResult<EnvironmentInfo> {
    let config = state.snapshot();
    Ok(components::detect_environment(
        &state.store,
        &config.settings,
        &state.app_version,
    ))
}

#[tauri::command]
pub fn get_app_logs(limit: Option<usize>) -> Vec<LogEntry> {
    logging::snapshot(limit.unwrap_or(500))
}

#[tauri::command]
pub fn clear_app_logs() {
    logging::clear();
}

#[tauri::command]
pub fn get_log_path() -> Option<String> {
    logging::log_file_path().map(|path| path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn set_autostart(app: AppHandle, enabled: bool) -> AppResult<()> {
    let manager = app.autolaunch();
    if enabled {
        manager.enable().map_err(|error| AppError::internal(error.to_string()))
    } else {
        manager.disable().map_err(|error| AppError::internal(error.to_string()))
    }
}

#[tauri::command]
pub fn get_autostart(app: AppHandle) -> AppResult<bool> {
    app.autolaunch()
        .is_enabled()
        .map_err(|error| AppError::internal(error.to_string()))
}

#[tauri::command]
pub async fn set_global_hotkey(app: AppHandle, state: State<'_, AppState>, accelerator: String) -> AppResult<String> {
    let shortcut = accelerator
        .parse::<tauri_plugin_global_shortcut::Shortcut>()
        .map_err(|error| AppError::config(format!("无效的快捷键: {error}")))?;
    let manager = app.global_shortcut();
    manager
        .unregister_all()
        .map_err(|error| AppError::internal(error.to_string()))?;
    manager
        .register(shortcut)
        .map_err(|error| AppError::internal(format!("快捷键注册失败: {error}")))?;
    {
        let mut config = state.config.lock().unwrap();
        config.settings.global_hotkey = accelerator.clone();
    }
    state.save()?;
    Ok(accelerator)
}

#[tauri::command]
pub fn show_main_window(app: AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

#[tauri::command]
pub fn set_window_effects(app: AppHandle, acrylic: bool, dark: bool) -> AppResult<()> {
    #[cfg(windows)]
    if let Some(window) = app.get_webview_window("main") {
        use tauri::window::{Color, Effect, EffectsBuilder};

        if acrylic {
            let tint = if dark {
                Color(18, 18, 24, 190)
            } else {
                Color(248, 247, 255, 205)
            };
            let effects = EffectsBuilder::new().effect(Effect::Acrylic).color(tint).build();
            window
                .set_effects(effects)
                .map_err(|error| AppError::internal(error.to_string()))?;
        } else {
            window
                .set_effects(None::<tauri::utils::config::WindowEffectsConfig>)
                .map_err(|error| AppError::internal(error.to_string()))?;
        }
    }
    #[cfg(not(windows))]
    let _ = (app, acrylic, dark);
    Ok(())
}

#[tauri::command]
pub fn quit_app(app: AppHandle) {
    app.exit(0);
}

#[tauri::command]
pub fn toggle_hud(app: AppHandle) {
    crate::toggle_hud(&app);
}

#[tauri::command]
pub fn show_hud(app: AppHandle) {
    if let Some(hud) = app.get_webview_window("hud") {
        let _ = hud.eval("window.location.hash = '#/hud'");
        let _ = hud.show();
        let _ = hud.set_focus();
    }
}

#[tauri::command]
pub async fn export_diagnostics(state: State<'_, AppState>) -> AppResult<String> {
    let timestamp = chrono::Local::now().format("%Y%m%d-%H%M%S");
    let zip_path = state.store.data_dir.join(format!("diagnostics-{timestamp}.zip"));
    let file = std::fs::File::create(&zip_path)?;
    let mut zip = zip::ZipWriter::new(file);
    let options = zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    let config = state.snapshot();
    let environment = components::detect_environment(&state.store, &config.settings, &state.app_version);
    let payload = serde_json::json!({
        "config": config,
        "environment": environment,
        "components": components::list_installed(&state.store),
        "logs": logging::snapshot(2000),
    });
    zip.start_file("diagnostics.json", options)?;
    zip.write_all(serde_json::to_string_pretty(&payload)?.as_bytes())?;

    if let Some(log_path) = logging::log_file_path() {
        if let Ok(bytes) = std::fs::read(log_path) {
            zip.start_file("originium.log", options)?;
            zip.write_all(&bytes)?;
        }
    }

    zip.finish()?;
    Ok(zip_path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn open_main_window(app: AppHandle) -> AppResult<()> {
    show_main_window(app);
    Ok(())
}

#[tauri::command]
pub async fn save_app_settings(
    state: State<'_, AppState>,
    settings: crate::models::AppSettings,
) -> AppResult<crate::models::AppSettings> {
    {
        let mut config = state.config.lock().unwrap();
        config.settings = settings.clone();
    }
    state.save()?;
    Ok(settings)
}

#[tauri::command]
pub async fn complete_onboarding(state: State<'_, AppState>) -> AppResult<()> {
    {
        let mut config = state.config.lock().unwrap();
        config.onboarding_complete = true;
    }
    state.save()?;
    Ok(())
}
