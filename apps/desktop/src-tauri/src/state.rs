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
        Ok(Self {
            store,
            config: Mutex::new(config),
            session: tokio::sync::Mutex::new(SessionManager::new()),
            app_version: env!("CARGO_PKG_VERSION").to_string(),
        })
    }

    pub fn save(&self) -> AppResult<()> {
        let config = self.config.lock().unwrap().clone();
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
