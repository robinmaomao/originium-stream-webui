use tauri::State;

use crate::error::{AppError, AppResult};
use crate::models::{DiscoveredHost, Host, HostInput, TestHostResult};
use crate::state::AppState;
use crate::sunshine::SunshineClient;

#[tauri::command]
pub async fn list_hosts(state: State<'_, AppState>) -> AppResult<Vec<Host>> {
    Ok(state.snapshot().hosts)
}

#[tauri::command]
pub async fn add_host(state: State<'_, AppState>, input: HostInput) -> AppResult<Host> {
    let id = input.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    if state.host(&id).is_some() {
        return Err(AppError::config("主机已存在"));
    }
    let host = Host {
        id: id.clone(),
        name: input.name,
        address: input.address,
        port: input.port,
        username: input.username.clone(),
        cert_fingerprint: None,
        source: input.source,
        paired: false,
        favorite: false,
        last_connected_at: None,
        note: None,
    };
    if let (Some(username), Some(password)) = (input.username.as_deref(), input.password.as_deref()) {
        state.store.set_credentials(&id, username, password)?;
    }
    state.update_host(host.clone())?;
    Ok(host)
}

#[tauri::command]
pub async fn update_host(state: State<'_, AppState>, input: HostInput) -> AppResult<Host> {
    let id = input.id.ok_or_else(|| AppError::config("更新主机时缺少 id"))?;
    let mut host = state.host(&id).ok_or_else(|| AppError::not_found("主机不存在"))?;
    host.name = input.name;
    host.address = input.address;
    host.port = input.port;
    host.username = input.username.clone();
    if let (Some(username), Some(password)) = (input.username.as_deref(), input.password.as_deref()) {
        state.store.set_credentials(&id, username, password)?;
    }
    state.update_host(host.clone())?;
    Ok(host)
}

#[tauri::command]
pub async fn remove_host(state: State<'_, AppState>, id: String) -> AppResult<()> {
    {
        let mut config = state.config.lock().unwrap();
        config.hosts.retain(|host| host.id != id);
        if config.active_host.as_deref() == Some(id.as_str()) {
            config.active_host = None;
        }
    }
    state.store.delete_credentials(&id);
    state.save()?;
    Ok(())
}

#[tauri::command]
pub async fn test_host(_state: State<'_, AppState>, input: HostInput) -> AppResult<TestHostResult> {
    let host = Host {
        id: input.id.unwrap_or_else(|| "probe".into()),
        name: input.name,
        address: input.address,
        port: input.port,
        username: input.username,
        cert_fingerprint: None,
        source: input.source,
        paired: false,
        favorite: false,
        last_connected_at: None,
        note: None,
    };
    SunshineClient::probe(&host).await
}

#[tauri::command]
pub async fn connect_host(state: State<'_, AppState>, id: String) -> AppResult<serde_json::Value> {
    let (_, client) = state.client_for(&id).await?;
    let config = client.get_config().await?;
    {
        let mut config_state = state.config.lock().unwrap();
        if let Some(host) = config_state.hosts.iter_mut().find(|host| host.id == id) {
            host.last_connected_at = Some(chrono::Utc::now().timestamp());
        }
    }
    state.save()?;
    Ok(config)
}

#[tauri::command]
pub async fn discover_hosts(timeout_ms: Option<u64>) -> AppResult<Vec<DiscoveredHost>> {
    crate::discovery::scan(timeout_ms.unwrap_or(3000)).await
}

#[tauri::command]
pub async fn add_discovered_host(state: State<'_, AppState>, discovered: DiscoveredHost) -> AppResult<Host> {
    {
        let config = state.config.lock().unwrap();
        if let Some(existing) = config
            .hosts
            .iter()
            .find(|host| host.address == discovered.address && host.port == discovered.port)
        {
            return Ok(existing.clone());
        }
    }
    let host = Host {
        id: uuid::Uuid::new_v4().to_string(),
        name: discovered.name,
        address: discovered.address,
        port: discovered.port,
        username: None,
        cert_fingerprint: None,
        source: discovered.source,
        paired: false,
        favorite: false,
        last_connected_at: None,
        note: None,
    };
    state.update_host(host.clone())?;
    Ok(host)
}

#[tauri::command]
pub async fn set_active_host(state: State<'_, AppState>, id: Option<String>) -> AppResult<()> {
    {
        let mut config = state.config.lock().unwrap();
        config.active_host = id;
    }
    state.save()?;
    Ok(())
}

#[tauri::command]
pub async fn reset_host_fingerprint(state: State<'_, AppState>, id: String) -> AppResult<Host> {
    let mut host = state.host(&id).ok_or_else(|| AppError::not_found("主机不存在"))?;
    host.cert_fingerprint = None;
    state.update_host(host.clone())?;
    Ok(host)
}
