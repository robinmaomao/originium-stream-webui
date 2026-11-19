use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

fn default_port() -> u16 {
    47990
}

/// A host known to the application (either manually added or discovered via mDNS).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Host {
    pub id: String,
    pub name: String,
    pub address: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub cert_fingerprint: Option<String>,
    #[serde(default)]
    pub source: HostSource,
    #[serde(default)]
    pub paired: bool,
    #[serde(default)]
    pub favorite: bool,
    #[serde(default)]
    pub last_connected_at: Option<i64>,
    #[serde(default)]
    pub note: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum HostSource {
    #[default]
    Manual,
    Mdns,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostInput {
    #[serde(default)]
    pub id: Option<String>,
    pub name: String,
    pub address: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
    #[serde(default)]
    pub source: HostSource,
}

/// Streaming profile mapped onto moonlight CLI flags.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamProfile {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub bitrate_kbps: u32,
    pub video_codec: String,
    pub audio_config: String,
    pub display_mode: String,
    pub hdr: bool,
    pub yuv444: bool,
    pub vsync: bool,
    pub frame_pacing: bool,
    pub quit_after: bool,
    pub keep_awake: bool,
    pub performance_overlay: bool,
    pub capture_system_keys: String,
    #[serde(default)]
    pub packet_size: Option<u32>,
    #[serde(default)]
    pub extra_args: Vec<String>,
}

impl Default for StreamProfile {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
            fps: 60,
            bitrate_kbps: 40000,
            video_codec: "auto".into(),
            audio_config: "stereo".into(),
            display_mode: "borderless".into(),
            hdr: false,
            yuv444: false,
            vsync: true,
            frame_pacing: true,
            quit_after: false,
            keep_awake: true,
            performance_overlay: false,
            capture_system_keys: "always".into(),
            packet_size: None,
            extra_args: Vec::new(),
        }
    }
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub language: String,
    pub autostart: bool,
    pub start_minimized: bool,
    pub close_to_tray: bool,
    pub global_hotkey: String,
    pub sunshine_path: Option<String>,
    pub moonlight_path: Option<String>,
    pub default_profile: StreamProfile,
    pub update_channel: String,
    pub prefer_service_mode: bool,
    #[serde(default = "default_true")]
    pub auto_start_services: bool,
    #[serde(default = "default_true")]
    pub acrylic: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            language: "zh-CN".into(),
            autostart: false,
            start_minimized: false,
            close_to_tray: true,
            global_hotkey: "CommandOrControl+Alt+Shift+O".into(),
            sunshine_path: None,
            moonlight_path: None,
            default_profile: StreamProfile::default(),
            update_channel: "stable".into(),
            prefer_service_mode: true,
            auto_start_services: true,
            acrylic: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThemeColors {
    pub primary: String,
    pub secondary: String,
    pub surface: String,
    pub background: String,
    pub error: String,
    pub success: String,
    pub info: String,
    pub warning: String,
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self {
            primary: "#7C4DFF".into(),
            secondary: "#00E5FF".into(),
            surface: "#121218".into(),
            background: "#0B0B0F".into(),
            error: "#FF5370".into(),
            success: "#4ADE80".into(),
            info: "#38BDF8".into(),
            warning: "#FBBF24".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThemeTypography {
    pub font_family: String,
    pub scale: f32,
}

impl Default for ThemeTypography {
    fn default() -> Self {
        Self {
            font_family: "Inter, 'Noto Sans SC', 'Microsoft YaHei', sans-serif".into(),
            scale: 1.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThemeBackground {
    pub mode: String,
    pub opacity: f32,
    pub blur: u32,
    pub image: Option<String>,
}

impl Default for ThemeBackground {
    fn default() -> Self {
        Self {
            mode: "cover".into(),
            opacity: 0.35,
            blur: 12,
            image: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThemeEffects {
    pub rounded: u32,
    pub shadows: bool,
    pub motion: bool,
}

impl Default for ThemeEffects {
    fn default() -> Self {
        Self {
            rounded: 12,
            shadows: true,
            motion: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ThemeConfig {
    pub version: u32,
    pub name: String,
    pub preset: String,
    pub colors: ThemeColors,
    pub typography: ThemeTypography,
    pub background: ThemeBackground,
    pub effects: ThemeEffects,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            version: 1,
            name: "Originium".into(),
            preset: "originium".into(),
            colors: ThemeColors::default(),
            typography: ThemeTypography::default(),
            background: ThemeBackground::default(),
            effects: ThemeEffects::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct AppConfig {
    pub schema_version: u32,
    pub onboarding_complete: bool,
    pub hosts: Vec<Host>,
    pub settings: AppSettings,
    pub theme: ThemeConfig,
    pub active_host: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            schema_version: 1,
            onboarding_complete: false,
            hosts: Vec::new(),
            settings: AppSettings::default(),
            theme: ThemeConfig::default(),
            active_host: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentManifest {
    pub components: Vec<ComponentSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentSpec {
    pub id: String,
    #[serde(default)]
    pub variant: String,
    pub version: String,
    #[serde(default)]
    pub upstream: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub sha256: Option<String>,
    pub entry: String,
    #[serde(default)]
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstalledComponent {
    pub id: String,
    pub variant: String,
    pub version: String,
    pub path: String,
    pub entry: String,
    pub capabilities: Vec<String>,
    pub installed_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscoveredHost {
    pub name: String,
    pub address: String,
    pub port: u16,
    pub source: HostSource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PendingPairing {
    pub id: String,
    pub name: String,
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowseEntry {
    pub name: String,
    pub path: String,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowseResult {
    pub path: String,
    pub parent: String,
    pub entries: Vec<BrowseEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvironmentInfo {
    pub platform: String,
    pub arch: String,
    pub is_admin: bool,
    pub sunshine_installed: bool,
    pub sunshine_service: String,
    pub moonlight_installed: bool,
    pub moonlight_version: Option<String>,
    pub webview2_version: Option<String>,
    pub data_dir: String,
    pub hostname: String,
    pub lan_ips: Vec<String>,
    pub app_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum SessionState {
    Idle,
    Launching,
    Streaming,
    Stopping,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionSnapshot {
    pub state: SessionState,
    pub host_id: Option<String>,
    pub host_name: Option<String>,
    pub app_name: Option<String>,
    pub started_at: Option<i64>,
    pub pid: Option<u32>,
    pub error: Option<String>,
    pub log_tail: Vec<String>,
}

impl Default for SessionSnapshot {
    fn default() -> Self {
        Self {
            state: SessionState::Idle,
            host_id: None,
            host_name: None,
            app_name: None,
            started_at: None,
            pid: None,
            error: None,
            log_tail: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub target: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BootstrapState {
    pub config: AppConfig,
    pub components: Vec<InstalledComponent>,
    pub environment: EnvironmentInfo,
    pub session: SessionSnapshot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetsInfo {
    pub fonts: Vec<String>,
    pub backgrounds: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestHostResult {
    pub ok: bool,
    pub latency_ms: u128,
    pub platform: String,
    pub version: String,
    pub raw: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PairingResult {
    pub paired: bool,
    pub pairing_id: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThemePreset {
    pub id: String,
    pub name: String,
    pub theme: ThemeConfig,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_settings_missing_acrylic_defaults_to_true() {
        let mut value = serde_json::to_value(AppSettings::default()).expect("serialize settings");
        value.as_object_mut().expect("settings object").remove("acrylic");
        let settings: AppSettings = serde_json::from_value(value).expect("deserialize settings");
        assert!(settings.acrylic);
    }

    #[test]
    fn app_settings_missing_auto_start_services_defaults_to_true() {
        let mut value = serde_json::to_value(AppSettings::default()).expect("serialize settings");
        value
            .as_object_mut()
            .expect("settings object")
            .remove("autoStartServices");
        let settings: AppSettings = serde_json::from_value(value).expect("deserialize settings");
        assert!(settings.auto_start_services);
    }
}
