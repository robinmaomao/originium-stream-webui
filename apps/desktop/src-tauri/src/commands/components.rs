use std::path::Path;

use tauri::{AppHandle, State};

use crate::components::{self, ServiceStatus};
use crate::error::{AppError, AppResult};
use crate::models::{ComponentSpec, EnvironmentInfo, InstalledComponent};
use crate::state::AppState;

#[tauri::command]
pub fn list_components(state: State<'_, AppState>) -> Vec<InstalledComponent> {
    components::list_installed(&state.store)
}

#[tauri::command]
pub fn get_component_manifest(state: State<'_, AppState>) -> AppResult<Vec<ComponentSpec>> {
    components::manifest(&state.store)
}

#[tauri::command]
pub async fn install_component(
    app: AppHandle,
    state: State<'_, AppState>,
    id: String,
) -> AppResult<InstalledComponent> {
    components::install(&app, &state.store, &id).await
}

#[tauri::command]
pub async fn remove_component(state: State<'_, AppState>, id: String) -> AppResult<()> {
    components::remove(&state.store, &id)
}

#[tauri::command]
pub fn get_service_status() -> AppResult<ServiceStatus> {
    components::sunshine_service_status()
}

#[tauri::command]
pub async fn start_sunshine_service() -> AppResult<ServiceStatus> {
    let status = components::sunshine_service_status()?;
    if let Some(name) = status.name.clone() {
        let state = status.state.as_deref().unwrap_or_default();
        let running = state.eq_ignore_ascii_case("running");
        let starting = state.eq_ignore_ascii_case("start pending") || state.eq_ignore_ascii_case("starting");
        if !running && !starting {
            components::start_service(&name)?;
        }
        if !running {
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        }
    }
    components::sunshine_service_status()
}

#[tauri::command]
pub async fn stop_sunshine_service() -> AppResult<ServiceStatus> {
    let status = components::sunshine_service_status()?;
    if let Some(name) = status.name.clone() {
        components::stop_service(&name)?;
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
    components::sunshine_service_status()
}

#[tauri::command]
pub async fn install_sunshine_service(state: State<'_, AppState>) -> AppResult<ServiceStatus> {
    let config = state.snapshot();
    let exe = components::locate_binary(&state.store, &config.settings, components::SUNSHINE_ID)
        .ok_or_else(|| AppError::component("未找到 Sunshine 可执行文件"))?;
    let dir = exe
        .parent()
        .ok_or_else(|| AppError::component("无效的 Sunshine 路径"))?;
    let install_script = dir.join("scripts").join("install-service.bat");
    let autostart_script = dir.join("scripts").join("autostart-service.bat");
    if !install_script.is_file() {
        return Err(AppError::component(
            "未找到 install-service.bat，请使用 lite 版或 MSI 安装包",
        ));
    }
    components::run_batch_elevated(&install_script, dir)?;
    if autostart_script.is_file() {
        let _ = components::run_batch_elevated(&autostart_script, dir);
    }
    components::sunshine_service_status()
}

#[tauri::command]
pub async fn add_firewall_rules(state: State<'_, AppState>) -> AppResult<()> {
    let config = state.snapshot();
    let exe = components::locate_binary(&state.store, &config.settings, components::SUNSHINE_ID)
        .ok_or_else(|| AppError::component("未找到 Sunshine 可执行文件"))?;
    let dir = exe
        .parent()
        .ok_or_else(|| AppError::component("无效的 Sunshine 路径"))?;
    let script = dir.join("scripts").join("add-firewall-rule.bat");
    if !script.is_file() {
        return Err(AppError::component("未找到 add-firewall-rule.bat"));
    }
    components::run_batch_elevated(&script, dir)
}

#[tauri::command]
pub async fn set_sunshine_credentials(state: State<'_, AppState>, username: String, password: String) -> AppResult<()> {
    if username.trim().is_empty() || password.is_empty() {
        return Err(AppError::config("用户名与密码不能为空"));
    }
    let config = state.snapshot();
    let exe = components::locate_binary(&state.store, &config.settings, components::SUNSHINE_ID)
        .ok_or_else(|| AppError::component("未找到 Sunshine 可执行文件"))?;
    components::set_sunshine_credentials(&exe, username.trim(), &password)
}

#[tauri::command]
pub async fn detect_components(state: State<'_, AppState>) -> AppResult<EnvironmentInfo> {
    let config = state.snapshot();
    Ok(components::detect_environment(
        &state.store,
        &config.settings,
        &state.app_version,
    ))
}

#[tauri::command]
pub async fn set_component_path(state: State<'_, AppState>, id: String, path: Option<String>) -> AppResult<()> {
    {
        let mut config = state.config.lock().unwrap();
        match id.as_str() {
            components::SUNSHINE_ID => config.settings.sunshine_path = path,
            components::MOONLIGHT_ID => config.settings.moonlight_path = path,
            other => return Err(AppError::config(format!("未知组件: {other}"))),
        }
    }
    state.save()?;
    Ok(())
}

#[tauri::command]
pub async fn validate_component_path(id: String, path: String) -> AppResult<bool> {
    let path = Path::new(&path);
    let expected = match id.as_str() {
        components::SUNSHINE_ID => "sunshine.exe",
        components::MOONLIGHT_ID => "moonlight.exe",
        _ => return Ok(false),
    };
    Ok(path.is_file()
        && path
            .file_name()
            .map(|name| name.eq_ignore_ascii_case(expected))
            .unwrap_or(false))
}
