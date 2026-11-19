use std::process::Stdio;
use std::time::{Duration, Instant};

use serde_json::Value;
use tauri::State;
use tokio::time::sleep;

use crate::error::{AppError, AppResult};
use crate::models::{BrowseResult, PairingResult, PendingPairing};
use crate::moonlight;
use crate::process::NoWindow;
use crate::state::AppState;

#[tauri::command]
pub async fn get_config(state: State<'_, AppState>, host_id: String) -> AppResult<Value> {
    let (_, client) = state.client_for(&host_id).await?;
    client.get_config().await
}

#[tauri::command]
pub async fn get_session_status(state: State<'_, AppState>, host_id: String) -> AppResult<Value> {
    let (_, client) = state.client_for(&host_id).await?;
    client.get_session_status().await
}

#[tauri::command]
pub async fn save_config(state: State<'_, AppState>, host_id: String, config: Value) -> AppResult<Value> {
    let (_, client) = state.client_for(&host_id).await?;
    client.save_config(&config).await
}

#[tauri::command]
pub async fn get_config_locale(state: State<'_, AppState>, host_id: String) -> AppResult<Value> {
    let (_, client) = state.client_for(&host_id).await?;
    client.get_config_locale().await
}

#[tauri::command]
pub async fn get_apps(state: State<'_, AppState>, host_id: String) -> AppResult<Value> {
    let (_, client) = state.client_for(&host_id).await?;
    client.get_apps().await
}

#[tauri::command]
pub async fn save_apps(state: State<'_, AppState>, host_id: String, apps: Value) -> AppResult<Value> {
    let (_, client) = state.client_for(&host_id).await?;
    client.save_apps(&apps).await
}

#[tauri::command]
pub async fn delete_app(state: State<'_, AppState>, host_id: String, index: u64) -> AppResult<Value> {
    let (_, client) = state.client_for(&host_id).await?;
    client.delete_app(index).await
}

#[tauri::command]
pub async fn close_app(state: State<'_, AppState>, host_id: String) -> AppResult<Value> {
    let (_, client) = state.client_for(&host_id).await?;
    client.close_app().await
}

#[tauri::command]
pub async fn get_clients(state: State<'_, AppState>, host_id: String) -> AppResult<Value> {
    let (_, client) = state.client_for(&host_id).await?;
    client.get_clients().await
}

#[tauri::command]
pub async fn unpair_client(state: State<'_, AppState>, host_id: String, uuid: String) -> AppResult<Value> {
    let (_, client) = state.client_for(&host_id).await?;
    client.unpair_client(&uuid).await
}

#[tauri::command]
pub async fn unpair_all(state: State<'_, AppState>, host_id: String) -> AppResult<Value> {
    let (_, client) = state.client_for(&host_id).await?;
    client.unpair_all().await
}

#[tauri::command]
pub async fn update_client(state: State<'_, AppState>, host_id: String, client: Value) -> AppResult<Value> {
    let (_, sunshine) = state.client_for(&host_id).await?;
    sunshine.update_client(&client).await
}

#[tauri::command]
pub async fn get_pending_pairings(state: State<'_, AppState>, host_id: String) -> AppResult<Vec<PendingPairing>> {
    let (_, client) = state.client_for(&host_id).await?;
    client.get_pending_pairings().await
}

#[tauri::command]
pub async fn submit_pin(
    state: State<'_, AppState>,
    host_id: String,
    pairing_id: String,
    pin: String,
    name: String,
) -> AppResult<Value> {
    let (_, client) = state.client_for(&host_id).await?;
    client.submit_pin(&pairing_id, &pin, &name).await
}

#[tauri::command]
pub async fn cancel_pairing(state: State<'_, AppState>, host_id: String, pairing_id: String) -> AppResult<Value> {
    let (_, client) = state.client_for(&host_id).await?;
    client.cancel_pairing(&pairing_id).await
}

#[tauri::command]
pub async fn change_password(state: State<'_, AppState>, host_id: String, payload: Value) -> AppResult<Value> {
    let (_, client) = state.client_for(&host_id).await?;
    client.change_password(&payload).await
}

#[tauri::command]
pub async fn get_host_logs(state: State<'_, AppState>, host_id: String) -> AppResult<String> {
    let (_, client) = state.client_for(&host_id).await?;
    client.get_logs().await
}

#[tauri::command]
pub async fn restart_host(state: State<'_, AppState>, host_id: String) -> AppResult<()> {
    let (_, client) = state.client_for(&host_id).await?;
    client.restart().await
}

#[tauri::command]
pub async fn browse_host(
    state: State<'_, AppState>,
    host_id: String,
    path: Option<String>,
    kind: Option<String>,
) -> AppResult<BrowseResult> {
    let (_, client) = state.client_for(&host_id).await?;
    client
        .browse(path.as_deref().unwrap_or_default(), kind.as_deref().unwrap_or("any"))
        .await
}

#[tauri::command]
pub async fn get_cover(state: State<'_, AppState>, host_id: String, index: u64) -> AppResult<String> {
    let (_, client) = state.client_for(&host_id).await?;
    client.get_cover(index).await
}

#[tauri::command]
pub async fn get_permissions(state: State<'_, AppState>, host_id: String) -> AppResult<Value> {
    let (_, client) = state.client_for(&host_id).await?;
    client.get_permissions().await
}

#[tauri::command]
pub async fn request_permission(state: State<'_, AppState>, host_id: String, id: String) -> AppResult<Value> {
    let (_, client) = state.client_for(&host_id).await?;
    client.request_permission(&id).await
}

#[tauri::command]
pub async fn get_virtual_input_status(state: State<'_, AppState>, host_id: String) -> AppResult<Value> {
    let (_, client) = state.client_for(&host_id).await?;
    client.get_virtual_input_status().await
}

#[tauri::command]
pub async fn get_virtual_input_license(state: State<'_, AppState>, host_id: String) -> AppResult<Value> {
    let (_, client) = state.client_for(&host_id).await?;
    client.get_virtual_input_license().await
}

#[tauri::command]
pub async fn reset_display_persistence(state: State<'_, AppState>, host_id: String) -> AppResult<Value> {
    let (_, client) = state.client_for(&host_id).await?;
    client.reset_display_persistence().await
}

#[tauri::command]
pub async fn get_csrf_token(state: State<'_, AppState>, host_id: String) -> AppResult<Value> {
    let (_, client) = state.client_for(&host_id).await?;
    client.get_csrf_token().await
}

/// Automate pairing by launching moonlight with a predefined PIN and approving it on the host.
#[tauri::command]
pub async fn pair_with_pin(
    state: State<'_, AppState>,
    host_id: String,
    pin: String,
    name: Option<String>,
) -> AppResult<PairingResult> {
    if pin.len() != 4 || !pin.chars().all(|c| c.is_ascii_digit()) {
        return Err(AppError::config("PIN 必须是 4 位数字"));
    }
    let host = state.host(&host_id).ok_or_else(|| AppError::not_found("主机不存在"))?;
    let config = state.snapshot();
    let moonlight_exe = moonlight::locate(&state.store, &config.settings)
        .ok_or_else(|| AppError::component("未找到 moonlight 可执行文件"))?;
    let (_, client) = state.client_for(&host_id).await?;

    let mut pair_command = tokio::process::Command::new(&moonlight_exe);
    pair_command
        .args(["pair", &host.address, "--pin", &pin])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .no_window();
    let mut child = pair_command
        .spawn()
        .map_err(|error| AppError::process(format!("无法启动 moonlight: {error}")))?;

    let deadline = Instant::now() + Duration::from_secs(60);
    let mut result = PairingResult {
        paired: false,
        pairing_id: None,
        message: "等待配对请求超时".into(),
    };

    while Instant::now() < deadline {
        if let Ok(Some(status)) = child.try_wait() {
            if status.success() {
                result.paired = true;
                result.message = "客户端已完成配对".into();
            } else {
                result.message = format!("moonlight 退出码: {}", status.code().unwrap_or(-1));
            }
            break;
        }

        if let Ok(pairings) = client.get_pending_pairings().await {
            if let Some(pairing) = pairings.into_iter().next() {
                let pairing_id = pairing.id.clone();
                let pairing_name = name
                    .clone()
                    .filter(|value| !value.is_empty())
                    .unwrap_or_else(|| host.name.clone());
                match client.submit_pin(&pairing_id, &pin, &pairing_name).await {
                    Ok(value) => {
                        let paired = value.get("status").and_then(|status| status.as_bool()).unwrap_or(false);
                        if paired {
                            result = PairingResult {
                                paired: true,
                                pairing_id: Some(pairing_id),
                                message: "配对成功".into(),
                            };
                            break;
                        }
                    }
                    Err(error) => {
                        result.message = error.message;
                    }
                }
            }
        }
        sleep(Duration::from_millis(800)).await;
    }

    if result.paired {
        let _ = tokio::time::timeout(Duration::from_secs(15), child.wait()).await;
        if let Some(mut host) = state.host(&host_id) {
            host.paired = true;
            state.update_host(host)?;
        }
        crate::logging::push("info", "pairing", &format!("配对成功: {}", host.name));
    } else {
        let _ = child.kill().await;
    }

    Ok(result)
}
