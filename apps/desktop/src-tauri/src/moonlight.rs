use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde_json::{Map, Value};
use tokio::io::{AsyncBufReadExt, BufReader};
#[cfg(windows)]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(windows)]
use tokio::net::windows::named_pipe::{ClientOptions, NamedPipeClient};
use tokio::process::Command;
use tokio::time::timeout;

use crate::error::{AppError, AppResult};
use crate::models::{Host, SessionSnapshot, SessionState, StreamProfile};
use crate::process::NoWindow;
use crate::store::Store;

const LOG_TAIL: usize = 120;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoonlightSettings {
    pub path: String,
    pub exists: bool,
    pub sections: Value,
}

/// Resolve the portable settings INI used by moonlight-qt.
pub fn settings_path(moonlight_exe: &Path) -> PathBuf {
    let base = moonlight_exe.parent().unwrap_or_else(|| Path::new("."));
    let candidates = [
        base.join("Moonlight Game Streaming Project").join("Moonlight.ini"),
        base.join("moonlight.ini"),
        base.join("Moonlight.ini"),
    ];
    for candidate in candidates {
        if candidate.exists() {
            return candidate;
        }
    }
    base.join("Moonlight Game Streaming Project").join("Moonlight.ini")
}

pub fn load_settings(moonlight_exe: &Path) -> MoonlightSettings {
    let path = settings_path(moonlight_exe);
    let sections = if path.exists() {
        fs::read_to_string(&path)
            .map(|content| parse_ini(&content))
            .unwrap_or_else(|_| Value::Object(Map::new()))
    } else {
        Value::Object(Map::new())
    };
    MoonlightSettings {
        path: path.to_string_lossy().to_string(),
        exists: path.exists(),
        sections,
    }
}

pub fn save_settings(moonlight_exe: &Path, sections: &Value) -> AppResult<MoonlightSettings> {
    let path = settings_path(moonlight_exe);
    let content = serialize_ini(sections)?;
    crate::store::atomic_write(&path, content.as_bytes())?;
    Ok(MoonlightSettings {
        path: path.to_string_lossy().to_string(),
        exists: true,
        sections: sections.clone(),
    })
}

pub fn parse_ini(content: &str) -> Value {
    let mut sections: BTreeMap<String, Map<String, Value>> = BTreeMap::new();
    let mut current = "General".to_string();
    sections.entry(current.clone()).or_default();

    for raw_line in content.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            current = line[1..line.len() - 1].trim().to_string();
            sections.entry(current.clone()).or_default();
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            sections
                .entry(current.clone())
                .or_default()
                .insert(key.trim().to_string(), Value::String(value.trim().to_string()));
        }
    }

    let mut object = Map::new();
    for (section, values) in sections {
        object.insert(section, Value::Object(values));
    }
    Value::Object(object)
}

pub fn serialize_ini(sections: &Value) -> AppResult<String> {
    let object = sections
        .as_object()
        .ok_or_else(|| AppError::config("INI 内容必须是对象"))?;
    let mut output = String::new();
    for (section, values) in object {
        output.push('[');
        output.push_str(section);
        output.push_str("]\n");
        if let Some(values) = values.as_object() {
            for (key, value) in values {
                let text = match value {
                    Value::String(text) => text.clone(),
                    other => other.to_string(),
                };
                output.push_str(key);
                output.push('=');
                output.push_str(&text);
                output.push('\n');
            }
        }
        output.push('\n');
    }
    Ok(output)
}

pub fn build_stream_args(host: &Host, app_name: &str, profile: &StreamProfile) -> Vec<String> {
    let mut args = vec!["stream".to_string(), host.address.clone(), app_name.to_string()];

    args.push("--resolution".into());
    args.push(format!("{}x{}", profile.width.max(320), profile.height.max(240)));
    args.push("--fps".into());
    args.push(profile.fps.clamp(10, 480).to_string());
    args.push("--bitrate".into());
    args.push(profile.bitrate_kbps.clamp(500, 500_000).to_string());

    let codec = match profile.video_codec.as_str() {
        "H.264" | "hevc" | "HEVC" | "av1" | "AV1" => profile.video_codec.clone(),
        _ => "auto".into(),
    };
    args.push("--video-codec".into());
    args.push(codec);

    let audio = match profile.audio_config.as_str() {
        "5.1-surround" | "7.1-surround" => profile.audio_config.clone(),
        _ => "stereo".into(),
    };
    args.push("--audio-config".into());
    args.push(audio);

    let display = match profile.display_mode.as_str() {
        "fullscreen" | "windowed" => profile.display_mode.clone(),
        _ => "borderless".into(),
    };
    args.push("--display-mode".into());
    args.push(display);

    let capture = match profile.capture_system_keys.as_str() {
        "never" | "fullscreen" => profile.capture_system_keys.clone(),
        _ => "always".into(),
    };
    args.push("--capture-system-keys".into());
    args.push(capture);

    args.push(if profile.vsync { "--vsync" } else { "--no-vsync" }.into());
    args.push(
        if profile.frame_pacing {
            "--frame-pacing"
        } else {
            "--no-frame-pacing"
        }
        .into(),
    );
    if profile.hdr {
        args.push("--hdr".into());
    }
    if profile.yuv444 {
        args.push("--yuv444".into());
    }
    if profile.quit_after {
        args.push("--quit-after".into());
    }
    if profile.keep_awake {
        args.push("--keep-awake".into());
    }
    if profile.performance_overlay {
        args.push("--performance-overlay".into());
    }
    if let Some(packet_size) = profile.packet_size.filter(|size| *size > 1024) {
        args.push("--packet-size".into());
        args.push(packet_size.to_string());
    }
    for extra in &profile.extra_args {
        let trimmed = extra.trim();
        if !trimmed.is_empty() {
            args.push(trimmed.to_string());
        }
    }

    args
}

pub struct SessionManager {
    snapshot: Arc<Mutex<SessionSnapshot>>,
    #[cfg(windows)]
    control: Option<ControlPipeClient>,
}

#[cfg(windows)]
pub struct ControlPipeClient {
    pipe: NamedPipeClient,
}

#[cfg(windows)]
impl ControlPipeClient {
    pub async fn connect(name: &str, timeout_ms: u64) -> AppResult<Self> {
        let path = format!(r"\\.\pipe\{name}");
        let deadline = std::time::Instant::now() + std::time::Duration::from_millis(timeout_ms);
        loop {
            match ClientOptions::new().open(&path) {
                Ok(pipe) => return Ok(Self { pipe }),
                Err(error) => {
                    if std::time::Instant::now() >= deadline {
                        return Err(AppError::process(format!("控制通道连接失败（{path}）: {error}")));
                    }
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                }
            }
        }
    }

    pub async fn request(&mut self, command: &str) -> AppResult<Value> {
        let payload = serde_json::json!({ "v": 1, "id": "originium-1", "cmd": command });
        let bytes = serde_json::to_vec(&payload)?;
        let length = u32::try_from(bytes.len()).map_err(|_| AppError::process("控制通道消息过长"))?;
        self.pipe.write_all(&length.to_le_bytes()).await?;
        self.pipe.write_all(&bytes).await?;
        self.pipe.flush().await?;

        let mut length_bytes = [0u8; 4];
        timeout(
            std::time::Duration::from_secs(3),
            self.pipe.read_exact(&mut length_bytes),
        )
        .await
        .map_err(|_| AppError::process("控制通道响应超时"))??;
        let response_length = u32::from_le_bytes(length_bytes) as usize;
        let mut response = vec![0u8; response_length];
        timeout(std::time::Duration::from_secs(3), self.pipe.read_exact(&mut response))
            .await
            .map_err(|_| AppError::process("控制通道响应超时"))??;
        Ok(serde_json::from_slice(&response)?)
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            snapshot: Arc::new(Mutex::new(SessionSnapshot::default())),
            #[cfg(windows)]
            control: None,
        }
    }

    pub fn snapshot(&self) -> SessionSnapshot {
        self.snapshot.lock().unwrap().clone()
    }

    pub fn reset(&mut self) {
        #[cfg(windows)]
        {
            self.control = None;
        }
        *self.snapshot.lock().unwrap() = SessionSnapshot::default();
    }

    /// Launch a moonlight streaming session and monitor its lifecycle.
    pub async fn launch(
        &mut self,
        moonlight_exe: &Path,
        host: &Host,
        app_name: &str,
        profile: &StreamProfile,
        control_enabled: bool,
    ) -> AppResult<SessionSnapshot> {
        {
            let snapshot = self.snapshot.lock().unwrap();
            if matches!(
                snapshot.state,
                SessionState::Launching | SessionState::Streaming | SessionState::Stopping
            ) {
                return Err(AppError::process("已有串流会话正在进行"));
            }
        }

        let mut args = build_stream_args(host, app_name, profile);
        let mut control_name = None;
        if control_enabled {
            let name = format!("originium-moonlight-{}", uuid::Uuid::new_v4().simple());
            args.push("--control-pipe".into());
            args.push(name.clone());
            control_name = Some(name);
        }
        crate::logging::push(
            "info",
            "moonlight",
            &format!("启动会话: {} {}", moonlight_exe.display(), args.join(" ")),
        );

        let mut command = Command::new(moonlight_exe);
        command
            .args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .no_window();

        let mut child = command.spawn().map_err(|error| {
            AppError::process(format!("无法启动 moonlight（{}）: {error}", moonlight_exe.display()))
        })?;
        let pid = child.id();

        #[cfg(windows)]
        if let Some(name) = control_name {
            match timeout(Duration::from_secs(4), ControlPipeClient::connect(&name, 3000)).await {
                Ok(Ok(client)) => {
                    self.control = Some(client);
                    crate::logging::push("info", "moonlight", "已连接控制通道");
                }
                Ok(Err(error)) => {
                    crate::logging::push("warn", "moonlight", &format!("控制通道不可用: {error}"));
                }
                Err(_) => {
                    crate::logging::push("warn", "moonlight", "控制通道连接超时");
                }
            }
        }

        {
            let mut snapshot = self.snapshot.lock().unwrap();
            snapshot.state = SessionState::Streaming;
            snapshot.host_id = Some(host.id.clone());
            snapshot.host_name = Some(host.name.clone());
            snapshot.app_name = Some(app_name.to_string());
            snapshot.started_at = Some(chrono::Utc::now().timestamp());
            snapshot.pid = pid;
            snapshot.error = None;
            snapshot.log_tail.clear();
        }

        if let Some(stdout) = child.stdout.take() {
            let snapshot = self.snapshot.clone();
            tokio::spawn(async move {
                let mut reader = BufReader::new(stdout).lines();
                while let Ok(Some(line)) = reader.next_line().await {
                    push_log(&snapshot, "info", &line);
                }
            });
        }
        if let Some(stderr) = child.stderr.take() {
            let snapshot = self.snapshot.clone();
            tokio::spawn(async move {
                let mut reader = BufReader::new(stderr).lines();
                while let Ok(Some(line)) = reader.next_line().await {
                    push_log(&snapshot, "warn", &line);
                }
            });
        }

        let snapshot = self.snapshot.clone();
        let host_id = host.id.clone();
        tokio::spawn(async move {
            let status = child.wait().await;
            let mut guard = snapshot.lock().unwrap();
            if guard.pid != pid {
                crate::logging::push(
                    "info",
                    "moonlight",
                    &format!("会话结束（host={host_id}，已由停止操作处理）"),
                );
                return;
            }
            guard.pid = None;
            guard.state = match status {
                Ok(status) if status.success() => SessionState::Idle,
                Ok(status) => {
                    guard.error = Some(format!("moonlight 退出码: {}", status.code().unwrap_or(-1)));
                    SessionState::Error
                }
                Err(error) => {
                    guard.error = Some(error.to_string());
                    SessionState::Error
                }
            };
            crate::logging::push(
                "info",
                "moonlight",
                &format!("会话结束（host={host_id}，state={:?}）", guard.state),
            );
        });

        Ok(self.snapshot())
    }

    pub async fn stop(&mut self, moonlight_exe: Option<&Path>, host: &Host) -> AppResult<SessionSnapshot> {
        let pid = {
            let mut snapshot = self.snapshot.lock().unwrap();
            snapshot.state = SessionState::Stopping;
            let pid = snapshot.pid;
            snapshot.pid = None;
            pid
        };

        #[cfg(windows)]
        if let Some(mut control) = self.control.take() {
            let _ = timeout(Duration::from_secs(2), control.request("session.stop")).await;
        }

        if let Some(pid) = pid {
            let mut taskkill = Command::new("taskkill.exe");
            taskkill.args(["/PID", &pid.to_string(), "/T", "/F"]).no_window();
            let _ = taskkill.output().await;
        }

        if let Some(exe) = moonlight_exe {
            let mut quit = Command::new(exe);
            quit.args(["quit", &host.address]).no_window();
            let _ = timeout(Duration::from_secs(8), quit.output()).await;
        }

        {
            let mut snapshot = self.snapshot.lock().unwrap();
            snapshot.state = SessionState::Idle;
            snapshot.pid = None;
        }
        crate::logging::push("info", "moonlight", "会话已停止");
        Ok(self.snapshot())
    }

    pub async fn quit_remote(moonlight_exe: &Path, host: &Host) -> AppResult<()> {
        let mut quit = Command::new(moonlight_exe);
        quit.args(["quit", &host.address]).no_window();
        let _ = timeout(Duration::from_secs(8), quit.output()).await;
        Ok(())
    }
}

fn push_log(snapshot: &Arc<Mutex<SessionSnapshot>>, level: &str, line: &str) {
    if line.trim().is_empty() {
        return;
    }
    crate::logging::push(level, "moonlight", line);
    let mut guard = snapshot.lock().unwrap();
    if guard.log_tail.len() >= LOG_TAIL {
        guard.log_tail.remove(0);
    }
    guard.log_tail.push(line.to_string());
}

/// Locate the moonlight executable through settings, bundles, or system installs.
pub fn locate(store: &Store, settings: &crate::models::AppSettings) -> Option<PathBuf> {
    crate::components::locate_binary(store, settings, crate::components::MOONLIGHT_ID)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{HostSource, StreamProfile};

    fn host() -> Host {
        Host {
            id: "host-1".into(),
            name: "Test Host".into(),
            address: "192.168.1.10".into(),
            port: 47990,
            username: None,
            cert_fingerprint: None,
            source: HostSource::Manual,
            paired: false,
            favorite: false,
            last_connected_at: None,
            note: None,
        }
    }

    #[test]
    fn ini_roundtrip_preserves_sections_and_unknown_keys() {
        let input = "[General]\nwidth=1920\ncustom=keep-me\n\n[video]\ncodec=hevc\n";
        let parsed = parse_ini(input);
        let output = serialize_ini(&parsed).expect("serialize");

        assert!(output.contains("width=1920"));
        assert!(output.contains("custom=keep-me"));
        assert!(output.contains("[video]"));
        assert!(output.contains("codec=hevc"));
    }

    #[test]
    fn stream_args_include_core_flags() {
        let profile = StreamProfile {
            width: 2560,
            height: 1440,
            fps: 120,
            bitrate_kbps: 80_000,
            ..StreamProfile::default()
        };
        let args = build_stream_args(&host(), "Desktop", &profile);

        assert_eq!(args[0], "stream");
        assert_eq!(args[1], "192.168.1.10");
        assert_eq!(args[2], "Desktop");
        assert!(args
            .windows(2)
            .any(|pair| pair[0] == "--resolution" && pair[1] == "2560x1440"));
        assert!(args.windows(2).any(|pair| pair[0] == "--fps" && pair[1] == "120"));
        assert!(args.windows(2).any(|pair| pair[0] == "--bitrate" && pair[1] == "80000"));
    }

    #[test]
    fn stream_args_clamp_out_of_range_values() {
        let profile = StreamProfile {
            fps: 9999,
            bitrate_kbps: 1,
            video_codec: "bogus".into(),
            display_mode: "bogus".into(),
            audio_config: "bogus".into(),
            capture_system_keys: "bogus".into(),
            ..StreamProfile::default()
        };
        let args = build_stream_args(&host(), "App", &profile);

        assert!(args.windows(2).any(|pair| pair[0] == "--fps" && pair[1] == "480"));
        assert!(args.windows(2).any(|pair| pair[0] == "--bitrate" && pair[1] == "500"));
        assert!(args
            .windows(2)
            .any(|pair| pair[0] == "--video-codec" && pair[1] == "auto"));
        assert!(args
            .windows(2)
            .any(|pair| pair[0] == "--display-mode" && pair[1] == "borderless"));
        assert!(args
            .windows(2)
            .any(|pair| pair[0] == "--audio-config" && pair[1] == "stereo"));
        assert!(args
            .windows(2)
            .any(|pair| pair[0] == "--capture-system-keys" && pair[1] == "always"));
    }

    #[test]
    fn stream_args_add_toggle_flags() {
        let profile = StreamProfile {
            hdr: true,
            yuv444: true,
            quit_after: true,
            keep_awake: true,
            performance_overlay: true,
            packet_size: Some(2048),
            ..StreamProfile::default()
        };
        let args = build_stream_args(&host(), "App", &profile);

        for flag in [
            "--hdr",
            "--yuv444",
            "--quit-after",
            "--keep-awake",
            "--performance-overlay",
        ] {
            assert!(args.contains(&flag.to_string()), "missing {flag}");
        }
        assert!(args
            .windows(2)
            .any(|pair| pair[0] == "--packet-size" && pair[1] == "2048"));
    }
}
