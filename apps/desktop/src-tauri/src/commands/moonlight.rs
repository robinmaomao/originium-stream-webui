use serde_json::Value;
use tauri::State;

use crate::error::{AppError, AppResult};
use crate::models::{SessionSnapshot, StreamProfile};
use crate::moonlight::{self, MoonlightSettings};
use crate::state::AppState;

#[tauri::command]
pub fn get_moonlight_settings(state: State<'_, AppState>) -> AppResult<MoonlightSettings> {
    let config = state.snapshot();
    match moonlight::locate(&state.store, &config.settings) {
        Some(exe) => Ok(moonlight::load_settings(&exe)),
        None => Ok(MoonlightSettings {
            path: String::new(),
            exists: false,
            sections: Value::Object(Default::default()),
        }),
    }
}

#[tauri::command]
pub fn save_moonlight_settings(state: State<'_, AppState>, sections: Value) -> AppResult<MoonlightSettings> {
    let config = state.snapshot();
    let exe = moonlight::locate(&state.store, &config.settings)
        .ok_or_else(|| AppError::component("未找到 moonlight 可执行文件"))?;
    moonlight::save_settings(&exe, &sections)
}

#[tauri::command]
pub fn get_stream_profile(state: State<'_, AppState>) -> StreamProfile {
    state.snapshot().settings.default_profile
}

#[tauri::command]
pub fn save_stream_profile(state: State<'_, AppState>, profile: StreamProfile) -> AppResult<StreamProfile> {
    {
        let mut config = state.config.lock().unwrap();
        config.settings.default_profile = profile.clone();
    }
    state.save()?;
    Ok(profile)
}

#[tauri::command]
pub async fn launch_session(
    state: State<'_, AppState>,
    host_id: String,
    app_name: String,
    profile: Option<StreamProfile>,
) -> AppResult<SessionSnapshot> {
    let host = state.host(&host_id).ok_or_else(|| AppError::not_found("主机不存在"))?;
    let config = state.snapshot();
    let exe = moonlight::locate(&state.store, &config.settings)
        .ok_or_else(|| AppError::component("未找到 moonlight 可执行文件"))?;
    let profile = profile.unwrap_or(config.settings.default_profile);
    let session = state.session.lock().await;
    session.launch(&exe, &host, &app_name, &profile).await
}

#[tauri::command]
pub async fn stop_session(state: State<'_, AppState>) -> AppResult<SessionSnapshot> {
    let config = state.snapshot();
    let exe = moonlight::locate(&state.store, &config.settings);
    let snapshot = state.session.lock().await.snapshot();
    let host = snapshot.host_id.as_deref().and_then(|host_id| state.host(host_id));
    match host {
        Some(host) => state.session.lock().await.stop(exe.as_deref(), &host).await,
        None => {
            let session = state.session.lock().await;
            session.reset();
            Ok(session.snapshot())
        }
    }
}

#[tauri::command]
pub async fn get_session(state: State<'_, AppState>) -> AppResult<SessionSnapshot> {
    Ok(state.session.lock().await.snapshot())
}

#[tauri::command]
pub async fn quit_remote(state: State<'_, AppState>, host_id: String) -> AppResult<()> {
    let host = state.host(&host_id).ok_or_else(|| AppError::not_found("主机不存在"))?;
    let config = state.snapshot();
    let exe = moonlight::locate(&state.store, &config.settings)
        .ok_or_else(|| AppError::component("未找到 moonlight 可执行文件"))?;
    moonlight::SessionManager::quit_remote(&exe, &host).await
}
