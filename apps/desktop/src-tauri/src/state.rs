use std::sync::Mutex;

use crate::error::{AppError, AppResult};
use crate::models::{AppConfig, Host};
use crate::moonlight::SessionManager;
use crate::store::Store;
use crate::sunshine::SunshineClient;

/// Global application state managed by Tauri.
pub struct AppState {
    pub store: Store,
    pub config: Mutex<AppConfig>,
    pub session: tokio::sync::Mutex<SessionManager>,
    pub app_version: String,
}

impl AppState {
    pub fn new(store: Store) -> AppResult<Self> {
        let mut config = store.load_config();
        if let Some(theme) = store.load_theme() {
            config.theme = theme;
        }
        let mut migrated = false;
        for host in &mut config.hosts {
            if host.source == crate::models::HostSource::Mdns && host.port == 47989 {
                host.port = 47990;
                migrated = true;
            }
        }
        if migrated {
            if let Err(error) = store.save_config(&config) {
                crate::logging::push("warn", "state", &format!("无法迁移 mDNS 主机端口: {error}"));
            }
        }
        Ok(Self {
            store,
            config: Mutex::new(config),
            session: tokio::sync::Mutex::new(SessionManager::new()),
            app_version: env!("CARGO_PKG_VERSION").to_string(),
        })
    }

    pub fn save(&self) -> AppResult<()> {
        let config = self.config.lock().unwrap();
        self.store.save_config(&config)?;
        self.store.save_theme(&config.theme)?;
        Ok(())
    }

    pub fn snapshot(&self) -> AppConfig {
        self.config.lock().unwrap().clone()
    }

    pub fn host(&self, host_id: &str) -> Option<Host> {
        self.config
            .lock()
            .unwrap()
            .hosts
            .iter()
            .find(|host| host.id == host_id)
            .cloned()
    }

    pub fn update_host(&self, host: Host) -> AppResult<()> {
        {
            let mut config = self.config.lock().unwrap();
            if let Some(existing) = config.hosts.iter_mut().find(|item| item.id == host.id) {
                *existing = host;
            } else {
                config.hosts.push(host);
            }
        }
        self.save()
    }

    /// Build an authenticated client for a host, persisting the certificate fingerprint on first trust.
    pub async fn client_for(&self, host_id: &str) -> AppResult<(Host, SunshineClient)> {
        let host = self
            .host(host_id)
            .ok_or_else(|| AppError::not_found(format!("主机不存在: {host_id}")))?;

        let credentials = self.store.get_credentials(&host.id)?;
        let (username, password) = match credentials {
            Some((username, password)) => (Some(username), Some(password)),
            None => (host.username.clone(), None),
        };

        let client = SunshineClient::connect(&host, username.as_deref(), password.as_deref()).await?;

        if host.cert_fingerprint.is_none() {
            let mut config = self.config.lock().unwrap();
            if let Some(existing) = config.hosts.iter_mut().find(|item| item.id == host.id) {
                existing.cert_fingerprint = Some(client.fingerprint.clone());
            }
            drop(config);
            self.save()?;
        }

        Ok((host, client))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::HostSource;

    #[test]
    fn migrates_mdns_game_stream_port_to_web_ui_port() {
        let dir = std::env::temp_dir().join(format!("originium-state-{}", uuid::Uuid::new_v4()));
        let store = Store::new(dir.clone()).expect("store");
        let mut config = AppConfig::default();
        config.hosts.push(Host {
            id: "discovered".into(),
            name: "Discovered".into(),
            address: "192.168.1.20".into(),
            port: 47989,
            username: None,
            cert_fingerprint: None,
            source: HostSource::Mdns,
            paired: false,
            favorite: false,
            last_connected_at: None,
            note: None,
        });
        store.save_config(&config).expect("save config");

        let state = AppState::new(store).expect("state");
        assert_eq!(state.snapshot().hosts[0].port, 47990);
        let reloaded = state.store.load_config();
        assert_eq!(reloaded.hosts[0].port, 47990);

        let _ = std::fs::remove_dir_all(dir);
    }
}
