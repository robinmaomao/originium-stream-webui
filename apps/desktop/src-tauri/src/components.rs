use std::fs;
use std::io::copy;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tauri::{AppHandle, Emitter};

use crate::error::{AppError, AppResult};
use crate::models::{AppSettings, ComponentSpec, EnvironmentInfo, InstalledComponent};
use crate::process::NoWindow;
use crate::store::Store;

pub const SUNSHINE_ID: &str = "sunshine";
pub const MOONLIGHT_ID: &str = "moonlight";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceStatus {
    pub found: bool,
    pub name: Option<String>,
    pub state: Option<String>,
    pub start_mode: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentProgress {
    pub id: String,
    pub phase: String,
    pub percent: u8,
    pub message: String,
}

pub fn manifest(store: &Store) -> AppResult<Vec<ComponentSpec>> {
    Ok(store.manifest()?.components)
}

pub fn find_spec(store: &Store, id: &str) -> AppResult<ComponentSpec> {
    manifest(store)?
        .into_iter()
        .find(|spec| spec.id == id)
        .ok_or_else(|| AppError::component(format!("组件清单中不存在: {id}")))
}

pub fn component_dir(store: &Store, id: &str) -> PathBuf {
    store.components_dir.join(id)
}

pub fn metadata_path(store: &Store, id: &str) -> PathBuf {
    component_dir(store, id).join("component.json")
}

pub fn list_installed(store: &Store) -> Vec<InstalledComponent> {
    let Ok(entries) = fs::read_dir(&store.components_dir) else {
        return Vec::new();
    };
    entries
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_dir())
        .filter_map(|entry| {
            let metadata = entry.path().join("component.json");
            let content = fs::read_to_string(metadata).ok()?;
            serde_json::from_str::<InstalledComponent>(&content).ok()
        })
        .collect()
}

pub fn installed(store: &Store, id: &str) -> Option<InstalledComponent> {
    list_installed(store).into_iter().find(|component| component.id == id)
}

pub async fn install(app: &AppHandle, store: &Store, id: &str) -> AppResult<InstalledComponent> {
    let spec = find_spec(store, id)?;
    let url = spec
        .url
        .clone()
        .ok_or_else(|| AppError::component(format!("组件 {id} 未配置下载地址")))?;

    emit_progress(app, id, "downloading", 5, "正在下载组件…");
    let client = reqwest::Client::builder()
        .connect_timeout(std::time::Duration::from_secs(20))
        .timeout(std::time::Duration::from_secs(900))
        .user_agent("Originium-Stream/0.1.2")
        .build()?;
    let response = client.get(&url).send().await?;
    if !response.status().is_success() {
        return Err(AppError::component(format!(
            "下载组件失败: HTTP {}",
            response.status().as_u16()
        )));
    }
    let total = response.content_length().unwrap_or(0);
    let mut downloaded = Vec::new();
    let mut stream = response.bytes_stream();
    use futures::StreamExt;
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        downloaded.extend_from_slice(&chunk);
        if total > 0 {
            let percent = ((downloaded.len() as f64 / total as f64) * 60.0) as u8 + 5;
            emit_progress(app, id, "downloading", percent.min(65), "正在下载组件…");
        }
    }

    emit_progress(app, id, "verifying", 70, "正在校验完整性…");
    if let Some(expected) = spec.sha256.as_deref().filter(|value| !value.is_empty()) {
        let actual = hex::encode(Sha256::digest(&downloaded));
        if !actual.eq_ignore_ascii_case(expected) {
            return Err(AppError::component(format!(
                "组件校验失败（期望 {expected}，实际 {actual}）"
            )));
        }
    }

    emit_progress(app, id, "extracting", 80, "正在解压组件…");
    let staging = store.components_dir.join(format!(".{id}.staging"));
    let _ = fs::remove_dir_all(&staging);
    fs::create_dir_all(&staging)?;
    extract_zip(&downloaded, &staging)?;

    let root = find_entry_root(&staging, &spec.entry)
        .ok_or_else(|| AppError::component(format!("组件包中未找到入口文件: {}", spec.entry)))?;

    let target = component_dir(store, id);
    let backup = store.components_dir.join(format!(".{id}.old"));
    let _ = fs::remove_dir_all(&backup);
    if target.exists() {
        fs::rename(&target, &backup)?;
    }
    fs::rename(&root, &target)?;
    let _ = fs::remove_dir_all(&staging);
    let _ = fs::remove_dir_all(&backup);

    let installed = InstalledComponent {
        id: spec.id.clone(),
        variant: spec.variant.clone(),
        version: spec.version.clone(),
        path: target.to_string_lossy().to_string(),
        entry: spec.entry.clone(),
        capabilities: spec.capabilities.clone(),
        installed_at: chrono::Utc::now().timestamp(),
    };
    fs::write(metadata_path(store, id), serde_json::to_string_pretty(&installed)?)?;

    emit_progress(app, id, "installed", 100, "组件安装完成");
    crate::logging::push("info", "components", &format!("已安装组件 {id} v{}", spec.version));
    Ok(installed)
}

pub fn remove(store: &Store, id: &str) -> AppResult<()> {
    let dir = component_dir(store, id);
    if dir.exists() {
        fs::remove_dir_all(dir)?;
    }
    Ok(())
}

fn emit_progress(app: &AppHandle, id: &str, phase: &str, percent: u8, message: &str) {
    let _ = app.emit(
        "component://progress",
        ComponentProgress {
            id: id.to_string(),
            phase: phase.to_string(),
            percent,
            message: message.to_string(),
        },
    );
}

fn extract_zip(bytes: &[u8], target: &Path) -> AppResult<()> {
    let reader = std::io::Cursor::new(bytes);
    let mut archive = zip::ZipArchive::new(reader)?;
    for index in 0..archive.len() {
        let mut entry = archive.by_index(index)?;
        let Some(relative) = entry.enclosed_name() else {
            return Err(AppError::component("组件包包含非法路径"));
        };
        let output = target.join(relative);
        if entry.is_dir() {
            fs::create_dir_all(&output)?;
            continue;
        }
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = fs::File::create(&output)?;
        copy(&mut entry, &mut file)?;
    }
    Ok(())
}

fn find_entry_root(root: &Path, entry: &str) -> Option<PathBuf> {
    if root.join(entry).is_file() {
        return Some(root.to_path_buf());
    }
    let entries = fs::read_dir(root).ok()?;
    for child in entries.filter_map(Result::ok) {
        let path = child.path();
        if path.is_dir() && path.join(entry).is_file() {
            return Some(path);
        }
    }
    None
}

/// Resolve the executable for a component, preferring user overrides and installed bundles.
pub fn locate_binary(store: &Store, settings: &AppSettings, id: &str) -> Option<PathBuf> {
    let override_path = match id {
        SUNSHINE_ID => settings.sunshine_path.clone(),
        MOONLIGHT_ID => settings.moonlight_path.clone(),
        _ => None,
    };
    if let Some(path) = override_path {
        let path = PathBuf::from(path);
        if path.is_file() {
            return Some(path);
        }
    }

    if let Some(component) = installed(store, id) {
        let candidate = PathBuf::from(&component.path).join(&component.entry);
        if candidate.is_file() {
            return Some(candidate);
        }
    }

    let executable = match id {
        SUNSHINE_ID => "sunshine.exe",
        MOONLIGHT_ID => "moonlight.exe",
        _ => return None,
    };

    let known_paths = if id == SUNSHINE_ID {
        vec![
            PathBuf::from(r"C:\Program Files\Sunshine\sunshine.exe"),
            PathBuf::from(r"C:\Program Files (x86)\Sunshine\sunshine.exe"),
        ]
    } else {
        vec![
            PathBuf::from(r"C:\Program Files\Moonlight Game Streaming Project\moonlight.exe"),
            PathBuf::from(r"C:\Program Files (x86)\Moonlight Game Streaming Project\moonlight.exe"),
        ]
    };
    for path in known_paths {
        if path.is_file() {
            return Some(path);
        }
    }

    let mut where_command = Command::new("where.exe");
    where_command.arg(executable).no_window();
    let output = where_command.output().ok()?;
    if output.status.success() {
        let first = String::from_utf8_lossy(&output.stdout)
            .lines()
            .next()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(PathBuf::from);
        if let Some(path) = first {
            if path.is_file() {
                return Some(path);
            }
        }
    }
    None
}

pub fn detect_environment(store: &Store, settings: &AppSettings, app_version: &str) -> EnvironmentInfo {
    let sunshine = locate_binary(store, settings, SUNSHINE_ID);
    let moonlight = locate_binary(store, settings, MOONLIGHT_ID);
    let service = sunshine_service_status().unwrap_or(ServiceStatus {
        found: false,
        name: None,
        state: None,
        start_mode: None,
    });

    EnvironmentInfo {
        platform: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        is_admin: is_elevated(),
        sunshine_installed: sunshine.is_some(),
        sunshine_service: service.state.unwrap_or_else(|| "not-found".into()),
        moonlight_installed: moonlight.is_some(),
        moonlight_version: moonlight.as_ref().and_then(|path| moonlight_version(path)),
        webview2_version: webview2_version(),
        data_dir: store.data_dir.to_string_lossy().to_string(),
        hostname: std::env::var("COMPUTERNAME").unwrap_or_else(|_| "Originium".into()),
        lan_ips: local_ips(),
        app_version: app_version.to_string(),
    }
}

pub fn is_elevated() -> bool {
    let mut whoami = Command::new("whoami.exe");
    whoami.arg("/groups").no_window();
    let output = whoami.output();
    match output {
        Ok(output) => String::from_utf8_lossy(&output.stdout).contains("S-1-16-12288"),
        Err(_) => false,
    }
}

pub fn local_ips() -> Vec<String> {
    let mut ips = Vec::new();
    if let Ok(socket) = std::net::UdpSocket::bind("0.0.0.0:0") {
        if socket.connect("8.8.8.8:80").is_ok() {
            if let Ok(address) = socket.local_addr() {
                ips.push(address.ip().to_string());
            }
        }
    }
    if ips.is_empty() {
        let mut hostname = Command::new("hostname.exe");
        hostname.arg("-I").no_window();
        if let Ok(output) = hostname.output() {
            ips.extend(
                String::from_utf8_lossy(&output.stdout)
                    .split_whitespace()
                    .map(str::to_string),
            );
        }
    }
    ips.sort();
    ips.dedup();
    ips
}

pub fn moonlight_version(path: &Path) -> Option<String> {
    let mut version_command = Command::new(path);
    version_command.arg("--version").no_window();
    let output = version_command.output().ok()?;
    let text = if output.stdout.is_empty() {
        String::from_utf8_lossy(&output.stderr).to_string()
    } else {
        String::from_utf8_lossy(&output.stdout).to_string()
    };
    text.lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .map(str::to_string)
}

pub fn webview2_version() -> Option<String> {
    let keys = [
        r"HKLM\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}",
        r"HKCU\SOFTWARE\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}",
    ];
    for key in keys {
        let mut reg_command = Command::new("reg.exe");
        reg_command.args(["query", key, "/v", "pv"]).no_window();
        if let Ok(output) = reg_command.output() {
            if output.status.success() {
                let text = String::from_utf8_lossy(&output.stdout);
                if let Some(version) = text.split_whitespace().last() {
                    return Some(version.to_string());
                }
            }
        }
    }
    None
}

pub fn sunshine_service_status() -> AppResult<ServiceStatus> {
    let script = "Get-CimInstance Win32_Service -Filter \"Name LIKE '%sunshine%'\" | Select-Object Name,State,StartMode | ConvertTo-Json -Compress";
    let mut powershell = Command::new("powershell.exe");
    powershell
        .args(["-NoProfile", "-NonInteractive", "-Command", script])
        .no_window();
    let output = powershell.output()?;
    let text = String::from_utf8_lossy(&output.stdout);
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Ok(ServiceStatus {
            found: false,
            name: None,
            state: None,
            start_mode: None,
        });
    }
    let value: serde_json::Value =
        serde_json::from_str(trimmed).map_err(|error| AppError::process(error.to_string()))?;
    let item = match &value {
        serde_json::Value::Array(items) => items.first().cloned().unwrap_or_default(),
        other => other.clone(),
    };
    Ok(ServiceStatus {
        found: true,
        name: item.get("Name").and_then(|value| value.as_str()).map(str::to_string),
        state: item.get("State").and_then(|value| value.as_str()).map(str::to_string),
        start_mode: item
            .get("StartMode")
            .and_then(|value| value.as_str())
            .map(str::to_string),
    })
}

pub fn start_service(name: &str) -> AppResult<()> {
    run_elevated_powershell(&format!("Start-Service -Name '{}'", escape(name)))
}

pub fn stop_service(name: &str) -> AppResult<()> {
    run_elevated_powershell(&format!("Stop-Service -Name '{}' -Force", escape(name)))
}

pub fn run_elevated(program: &Path, args: &[String]) -> AppResult<()> {
    let program = program.to_string_lossy().replace('\'', "''");
    let arguments = args
        .iter()
        .map(|arg| format!("'{}'", arg.replace('\'', "''")))
        .collect::<Vec<_>>()
        .join(",");
    let script = format!(
        "$process = Start-Process -FilePath '{program}' -ArgumentList @({arguments}) -Verb RunAs -WindowStyle Hidden -PassThru -Wait; exit $process.ExitCode"
    );
    run_elevated_powershell(&script)
}

pub fn run_elevated_powershell(script: &str) -> AppResult<()> {
    let command = format!(
        "$encoded = [Convert]::ToBase64String([Text.Encoding]::Unicode.GetBytes(\"{}\")); \
         $process = Start-Process -FilePath 'powershell.exe' -ArgumentList @('-NoProfile','-NonInteractive','-EncodedCommand',$encoded) -Verb RunAs -WindowStyle Hidden -PassThru -Wait; \
         exit $process.ExitCode",
        script.replace('"', "`\"")
    );
    let mut powershell = Command::new("powershell.exe");
    powershell
        .args(["-NoProfile", "-NonInteractive", "-Command", &command])
        .no_window();
    let status = powershell.status()?;
    if !status.success() {
        return Err(AppError::process("提权操作已取消或失败"));
    }
    Ok(())
}

pub fn run_batch_elevated(script_path: &Path, working_dir: &Path) -> AppResult<()> {
    let script = format!(
        "Set-Location -LiteralPath '{}'; & '{}'",
        escape(&working_dir.to_string_lossy()),
        escape(&script_path.to_string_lossy())
    );
    run_elevated_powershell(&script)
}

pub fn set_sunshine_credentials(sunshine: &Path, username: &str, password: &str) -> AppResult<()> {
    run_elevated(
        sunshine,
        &["--creds".to_string(), username.to_string(), password.to_string()],
    )
}

fn escape(value: &str) -> String {
    value.replace('\'', "''")
}
