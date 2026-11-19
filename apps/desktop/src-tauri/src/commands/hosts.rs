use tauri::State;

use crate::error::{AppError, AppResult};
use crate::models::{DiscoveredHost, Host, HostInput, TestHostResult};
use crate::state::AppState;
use crate::sunshine::SunshineClient;

#[tauri::command]
pub async fn list_hosts(state: State<'_, AppState>) -> AppResult<Vec<Host>> {
    Ok(state.snapshot().hosts)
}

fn normalize_input(mut input: HostInput) -> AppResult<HostInput> {
    input.name = input.name.trim().to_string();
    input.address = input.address.trim().to_string();
    input.username = input
        .username
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    input.password = input.password.filter(|value| !value.is_empty());

    if input.name.is_empty() {
        return Err(AppError::config("主机名称不能为空"));
    }
    if input.address.is_empty() {
        return Err(AppError::config("主机地址不能为空"));
    }
    if input.port == 0 {
        return Err(AppError::config("主机端口必须大于 0"));
    }
    if input.password.is_some() && input.username.is_none() {
        return Err(AppError::config("填写密码时必须同时填写用户名"));
    }
    Ok(input)
}

fn duplicate_host(state: &AppState, address: &str, port: u16, exclude_id: Option<&str>) -> bool {
    state.snapshot().hosts.iter().any(|host| {
        Some(host.id.as_str()) != exclude_id && host.address.eq_ignore_ascii_case(address) && host.port == port
    })
}

#[tauri::command]
pub async fn add_host(state: State<'_, AppState>, input: HostInput) -> AppResult<Host> {
    let input = normalize_input(input)?;
    let id = input.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    if state.host(&id).is_some() {
        return Err(AppError::config("主机已存在"));
    }
    if duplicate_host(&state, &input.address, input.port, None) {
        return Err(AppError::config("该地址与端口已存在"));
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
    let input = normalize_input(input)?;
    let id = input.id.clone().ok_or_else(|| AppError::config("更新主机时缺少 id"))?;
    let mut host = state.host(&id).ok_or_else(|| AppError::not_found("主机不存在"))?;
    if duplicate_host(&state, &input.address, input.port, Some(&id)) {
        return Err(AppError::config("该地址与端口已存在"));
    }
    if host.address != input.address || host.port != input.port {
        host.cert_fingerprint = None;
    }
    host.name = input.name;
    host.address = input.address;
    host.port = input.port;
    host.username = input.username.clone();
    if let Some(username) = input.username.as_deref() {
        if let Some(password) = input.password.as_deref() {
            state.store.set_credentials(&id, username, password)?;
        } else if let Ok(Some((_, stored_password))) = state.store.get_credentials(&id) {
            state.store.set_credentials(&id, username, &stored_password)?;
        }
    } else {
        state.store.delete_credentials(&id);
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
pub async fn test_host(state: State<'_, AppState>, input: HostInput) -> AppResult<TestHostResult> {
    let input = normalize_input(input)?;
    let mut password = input.password.clone();
    let mut username = input.username.clone();
    if let Some(host_id) = input.id.as_deref() {
        if let Ok(Some((stored_username, stored_password))) = state.store.get_credentials(host_id) {
            if username.is_none() {
                username = Some(stored_username);
            }
            if password.is_none() {
                password = Some(stored_password);
            }
        }
    }
    let host = Host {
        id: input.id.unwrap_or_else(|| "probe".into()),
        name: input.name,
        address: input.address,
        port: input.port,
        username,
        cert_fingerprint: None,
        source: input.source,
        paired: false,
        favorite: false,
        last_connected_at: None,
        note: None,
    };
    SunshineClient::probe(&host, password.as_deref()).await
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
    let name = discovered.name.trim().to_string();
    let address = discovered.address.trim().to_string();
    if name.is_empty() || address.is_empty() || discovered.port == 0 {
        return Err(AppError::config("发现的主机信息无效"));
    }
    {
        let config = state.config.lock().unwrap();
        if let Some(existing) = config
            .hosts
            .iter()
            .find(|host| host.address.eq_ignore_ascii_case(&address) && host.port == discovered.port)
        {
            return Ok(existing.clone());
        }
    }
    let host = Host {
        id: uuid::Uuid::new_v4().to_string(),
        name,
        address,
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
    if let Some(ref host_id) = id {
        if state.host(host_id).is_none() {
            return Err(AppError::not_found("主机不存在"));
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::HostSource;

    fn input(name: &str, address: &str, port: u16) -> HostInput {
        HostInput {
            id: None,
            name: name.into(),
            address: address.into(),
            port,
            username: None,
            password: None,
            source: HostSource::Manual,
        }
    }

    #[test]
    fn normalize_trims_and_validates() {
        let mut value = input("  Host  ", " 192.168.1.2 ", 47990);
        value.username = Some("  user ".into());
        value.password = Some("".into());
        let normalized = normalize_input(value).expect("valid input");
        assert_eq!(normalized.name, "Host");
        assert_eq!(normalized.address, "192.168.1.2");
        assert_eq!(normalized.username.as_deref(), Some("user"));
        assert!(normalized.password.is_none());

        assert!(normalize_input(input("", "127.0.0.1", 47990)).is_err());
        assert!(normalize_input(input("Host", " ", 47990)).is_err());
        assert!(normalize_input(input("Host", "127.0.0.1", 0)).is_err());

        let mut password_only = input("Host", "127.0.0.1", 47990);
        password_only.password = Some("secret".into());
        assert!(normalize_input(password_only).is_err());
    }
}
