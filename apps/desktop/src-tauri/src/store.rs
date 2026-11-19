use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use base64::Engine as _;

use crate::error::{AppError, AppResult};
use crate::models::{AppConfig, AssetsInfo, ComponentManifest, ThemeConfig};

/// Persistent storage rooted at the application data directory.
pub struct Store {
    pub data_dir: PathBuf,
    pub config_path: PathBuf,
    pub components_dir: PathBuf,
    pub fonts_dir: PathBuf,
    pub backgrounds_dir: PathBuf,
    pub logs_dir: PathBuf,
}

impl Store {
    pub fn new(data_dir: PathBuf) -> AppResult<Self> {
        let components_dir = data_dir.join("components");
        let fonts_dir = data_dir.join("fonts");
        let backgrounds_dir = data_dir.join("backgrounds");
        let logs_dir = data_dir.join("logs");
        for dir in [&data_dir, &components_dir, &fonts_dir, &backgrounds_dir, &logs_dir] {
            fs::create_dir_all(dir)?;
        }
        Ok(Self {
            config_path: data_dir.join("config.json"),
            data_dir,
            components_dir,
            fonts_dir,
            backgrounds_dir,
            logs_dir,
        })
    }

    pub fn load_config(&self) -> AppConfig {
        let Ok(content) = fs::read_to_string(&self.config_path) else {
            return AppConfig::default();
        };
        match serde_json::from_str::<AppConfig>(&content) {
            Ok(config) => config,
            Err(error) => {
                let backup = self.config_path.with_extension("corrupt.json");
                let _ = fs::copy(&self.config_path, &backup);
                crate::logging::push(
                    "warn",
                    "store",
                    &format!("配置文件解析失败，已备份到 {}: {error}", backup.display()),
                );
                AppConfig::default()
            }
        }
    }

    pub fn save_config(&self, config: &AppConfig) -> AppResult<()> {
        let json = serde_json::to_string_pretty(config)?;
        atomic_write(&self.config_path, json.as_bytes())
    }

    pub fn load_theme(&self) -> Option<ThemeConfig> {
        let content = fs::read_to_string(self.data_dir.join("theme.json")).ok()?;
        serde_json::from_str(&content).ok()
    }

    pub fn save_theme(&self, theme: &ThemeConfig) -> AppResult<()> {
        let json = serde_json::to_string_pretty(theme)?;
        atomic_write(&self.data_dir.join("theme.json"), json.as_bytes())
    }

    pub fn manifest(&self) -> AppResult<ComponentManifest> {
        let override_path = self.data_dir.join("components.lock.json");
        let content = if override_path.exists() {
            fs::read_to_string(&override_path)?
        } else {
            include_str!("../../../../components.lock.json").to_string()
        };
        serde_json::from_str(&content).map_err(|error| AppError::component(error.to_string()))
    }

    pub fn credential_entry(host_id: &str) -> AppResult<keyring::Entry> {
        keyring::Entry::new("dev.originium.stream", &format!("host:{host_id}")).map_err(Into::into)
    }

    pub fn set_credentials(&self, host_id: &str, username: &str, password: &str) -> AppResult<()> {
        let payload = serde_json::json!({ "username": username, "password": password }).to_string();
        let entry = Self::credential_entry(host_id)?;
        entry.set_password(&payload)?;
        Ok(())
    }

    pub fn get_credentials(&self, host_id: &str) -> AppResult<Option<(String, String)>> {
        let entry = Self::credential_entry(host_id)?;
        match entry.get_password() {
            Ok(payload) => {
                let value: serde_json::Value = serde_json::from_str(&payload).unwrap_or_default();
                Ok(Some((
                    value
                        .get("username")
                        .and_then(|v| v.as_str())
                        .unwrap_or_default()
                        .to_string(),
                    value
                        .get("password")
                        .and_then(|v| v.as_str())
                        .unwrap_or_default()
                        .to_string(),
                )))
            }
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    pub fn delete_credentials(&self, host_id: &str) {
        if let Ok(entry) = Self::credential_entry(host_id) {
            let _ = entry.delete_credential();
        }
    }

    pub fn list_assets(&self) -> AssetsInfo {
        AssetsInfo {
            fonts: list_files(&self.fonts_dir),
            backgrounds: list_files(&self.backgrounds_dir),
        }
    }

    pub fn import_asset(&self, kind: &str, source: &Path) -> AppResult<String> {
        let dir = match kind {
            "font" => &self.fonts_dir,
            "background" => &self.backgrounds_dir,
            other => return Err(AppError::theme(format!("未知资源类型: {other}"))),
        };
        let file_name = source
            .file_name()
            .ok_or_else(|| AppError::theme("无效的文件路径"))?
            .to_string_lossy()
            .to_string();
        let metadata = fs::metadata(source)?;
        if metadata.len() > 64 * 1024 * 1024 {
            return Err(AppError::theme("资源文件过大（最大 64 MB）"));
        }
        let target = dir.join(&file_name);
        fs::copy(source, &target)?;
        let relative = format!("{kind}s/{file_name}");
        Ok(relative)
    }

    pub fn asset_path(&self, relative: &str) -> Option<PathBuf> {
        let normalized = relative.trim().replace('\\', "/");
        if normalized.is_empty() {
            return None;
        }
        let candidate = Path::new(&normalized);
        if candidate.is_absolute()
            || candidate.components().any(|component| {
                !matches!(
                    component,
                    std::path::Component::Normal(_) | std::path::Component::CurDir
                )
            })
        {
            return None;
        }
        let mut components = candidate.components();
        let Some(std::path::Component::Normal(root)) = components.next() else {
            return None;
        };
        if root != "fonts" && root != "backgrounds" {
            return None;
        }
        components.next()?;
        let base = fs::canonicalize(&self.data_dir).ok()?;
        let path = fs::canonicalize(self.data_dir.join(candidate)).ok()?;
        (path.starts_with(&base) && path.is_file()).then_some(path)
    }

    pub fn read_asset_base64(&self, relative: &str) -> AppResult<String> {
        let path = self
            .asset_path(relative)
            .ok_or_else(|| AppError::not_found(format!("资源不存在: {relative}")))?;
        if fs::metadata(&path)?.len() > 64 * 1024 * 1024 {
            return Err(AppError::theme("资源文件过大（最大 64 MB）"));
        }
        let mut file = fs::File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(base64::engine::general_purpose::STANDARD.encode(buffer))
    }
}

fn list_files(dir: &Path) -> Vec<String> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut files: Vec<String> = entries
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_file())
        .map(|entry| entry.file_name().to_string_lossy().to_string())
        .collect();
    files.sort();
    files
}

pub fn atomic_write(path: &Path, content: &[u8]) -> AppResult<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let temp = path.with_extension("tmp");
    {
        let mut file = fs::File::create(&temp)?;
        file.write_all(content)?;
        file.sync_all()?;
    }
    let backup = path.with_extension("bak");
    let had_original = path.exists();
    if had_original {
        fs::copy(path, &backup)?;
        fs::remove_file(path)?;
    }
    if let Err(error) = fs::rename(&temp, path) {
        if had_original {
            let _ = fs::rename(&backup, path);
        }
        let _ = fs::remove_file(&temp);
        return Err(error.into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asset_paths_are_confined_to_asset_directories() {
        let dir = std::env::temp_dir().join(format!("originium-store-{}", uuid::Uuid::new_v4()));
        let store = Store::new(dir.clone()).expect("create store");
        fs::write(store.config_path.clone(), "{}").expect("write config");
        fs::create_dir_all(store.fonts_dir.join("nested")).expect("create nested");
        fs::write(store.fonts_dir.join("nested").join("font.woff2"), b"font").expect("write font");
        fs::write(store.backgrounds_dir.join("wall.png"), b"png").expect("write background");

        assert!(store.asset_path("fonts/nested/font.woff2").is_some());
        assert!(store.asset_path("backgrounds/wall.png").is_some());
        assert!(store.asset_path("fonts/../config.json").is_none());
        assert!(store.asset_path("../config.json").is_none());
        assert!(store.asset_path("config.json").is_none());
        assert!(store.asset_path("fonts").is_none());
        assert!(store.asset_path("C:/Windows/win.ini").is_none());

        let _ = fs::remove_dir_all(dir);
    }
}
