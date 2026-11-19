use base64::Engine as _;
use serde_json::Value;
use tauri::State;

use crate::error::{AppError, AppResult};
use crate::models::{
    AssetsInfo, ThemeBackground, ThemeColors, ThemeConfig, ThemeEffects, ThemePreset, ThemeTypography,
};
use crate::state::AppState;

fn preset_originium() -> ThemeConfig {
    ThemeConfig {
        version: 1,
        name: "Originium".into(),
        preset: "originium".into(),
        colors: ThemeColors {
            primary: "#7C4DFF".into(),
            secondary: "#00E5FF".into(),
            surface: "#121218".into(),
            background: "#0B0B0F".into(),
            error: "#FF5370".into(),
            success: "#4ADE80".into(),
            info: "#38BDF8".into(),
            warning: "#FBBF24".into(),
        },
        typography: ThemeTypography {
            font_family: "Inter, 'Noto Sans SC', 'Microsoft YaHei', sans-serif".into(),
            scale: 1.0,
        },
        background: ThemeBackground {
            mode: "cover".into(),
            opacity: 0.35,
            blur: 12,
            image: None,
        },
        effects: ThemeEffects {
            rounded: 14,
            shadows: true,
            motion: true,
        },
    }
}

fn preset_midnight() -> ThemeConfig {
    ThemeConfig {
        version: 1,
        name: "Midnight".into(),
        preset: "midnight".into(),
        colors: ThemeColors {
            primary: "#22D3EE".into(),
            secondary: "#A78BFA".into(),
            surface: "#0F172A".into(),
            background: "#060B18".into(),
            error: "#FB7185".into(),
            success: "#34D399".into(),
            info: "#60A5FA".into(),
            warning: "#FBBF24".into(),
        },
        typography: ThemeTypography {
            font_family: "'Segoe UI', 'Noto Sans SC', sans-serif".into(),
            scale: 1.0,
        },
        background: ThemeBackground {
            mode: "cover".into(),
            opacity: 0.45,
            blur: 18,
            image: None,
        },
        effects: ThemeEffects {
            rounded: 10,
            shadows: true,
            motion: true,
        },
    }
}

fn preset_light() -> ThemeConfig {
    ThemeConfig {
        version: 1,
        name: "Aurora Light".into(),
        preset: "light".into(),
        colors: ThemeColors {
            primary: "#6D28D9".into(),
            secondary: "#0891B2".into(),
            surface: "#FFFFFF".into(),
            background: "#F6F4FF".into(),
            error: "#DC2626".into(),
            success: "#059669".into(),
            info: "#2563EB".into(),
            warning: "#D97706".into(),
        },
        typography: ThemeTypography {
            font_family: "'Segoe UI', 'Noto Sans SC', sans-serif".into(),
            scale: 1.0,
        },
        background: ThemeBackground {
            mode: "cover".into(),
            opacity: 0.2,
            blur: 0,
            image: None,
        },
        effects: ThemeEffects {
            rounded: 12,
            shadows: true,
            motion: true,
        },
    }
}

#[tauri::command]
pub fn get_theme(state: State<'_, AppState>) -> ThemeConfig {
    state.snapshot().theme
}

#[tauri::command]
pub fn list_theme_presets() -> Vec<ThemePreset> {
    vec![
        ThemePreset {
            id: "originium".into(),
            name: "源石（默认深色）".into(),
            theme: preset_originium(),
        },
        ThemePreset {
            id: "midnight".into(),
            name: "午夜".into(),
            theme: preset_midnight(),
        },
        ThemePreset {
            id: "light".into(),
            name: "极光浅色".into(),
            theme: preset_light(),
        },
    ]
}

#[tauri::command]
pub async fn save_theme(state: State<'_, AppState>, theme: ThemeConfig) -> AppResult<ThemeConfig> {
    let theme = validate_theme(&state, theme)?;
    {
        let mut config = state.config.lock().unwrap();
        config.theme = theme.clone();
    }
    state.save()?;
    Ok(theme)
}

#[tauri::command]
pub fn export_theme(state: State<'_, AppState>) -> AppResult<String> {
    Ok(serde_json::to_string_pretty(&state.snapshot().theme)?)
}

#[tauri::command]
pub async fn import_theme(state: State<'_, AppState>, json: String) -> AppResult<ThemeConfig> {
    let theme: ThemeConfig =
        serde_json::from_str(&json).map_err(|error| AppError::theme(format!("主题解析失败: {error}")))?;
    let theme = validate_theme(&state, theme)?;
    {
        let mut config = state.config.lock().unwrap();
        config.theme = theme.clone();
    }
    state.save()?;
    Ok(theme)
}

#[tauri::command]
pub async fn import_asset(state: State<'_, AppState>, kind: String, path: String) -> AppResult<String> {
    state.store.import_asset(&kind, std::path::Path::new(&path))
}

#[tauri::command]
pub fn list_assets(state: State<'_, AppState>) -> AssetsInfo {
    state.store.list_assets()
}

#[tauri::command]
pub fn get_asset_data_url(state: State<'_, AppState>, relative: String) -> AppResult<String> {
    let encoded = state.store.read_asset_base64(&relative)?;
    let mime = mime_for(&relative);
    Ok(format!("data:{mime};base64,{encoded}"))
}

#[tauri::command]
pub fn get_raw_theme(state: State<'_, AppState>) -> Value {
    serde_json::to_value(state.snapshot().theme).unwrap_or(Value::Null)
}

fn validate_theme(state: &AppState, mut theme: ThemeConfig) -> AppResult<ThemeConfig> {
    for (name, color) in [
        ("primary", &theme.colors.primary),
        ("secondary", &theme.colors.secondary),
        ("surface", &theme.colors.surface),
        ("background", &theme.colors.background),
        ("error", &theme.colors.error),
        ("success", &theme.colors.success),
        ("info", &theme.colors.info),
        ("warning", &theme.colors.warning),
    ] {
        if !is_hex_color(color) {
            return Err(AppError::theme(format!("颜色 {name} 不是合法的十六进制颜色")));
        }
    }

    if theme.name.trim().is_empty() {
        theme.name = "Custom".into();
    }
    theme.typography.scale = theme.typography.scale.clamp(0.8, 1.4);
    theme.background.opacity = theme.background.opacity.clamp(0.0, 1.0);
    theme.background.blur = theme.background.blur.min(64);
    theme.effects.rounded = theme.effects.rounded.min(32);
    if !matches!(theme.background.mode.as_str(), "cover" | "contain" | "tile" | "center") {
        theme.background.mode = "cover".into();
    }
    if let Some(image) = &theme.background.image {
        if !image.is_empty() && state.store.asset_path(image).is_none() {
            return Err(AppError::theme(format!("背景图不存在: {image}")));
        }
    }
    if theme.typography.font_family.trim().is_empty() {
        theme.typography.font_family = "Inter, 'Noto Sans SC', sans-serif".into();
    }
    Ok(theme)
}

fn is_hex_color(value: &str) -> bool {
    let Some(hex) = value.strip_prefix('#') else {
        return false;
    };
    matches!(hex.len(), 3 | 4 | 6 | 8) && hex.chars().all(|c| c.is_ascii_hexdigit())
}

fn mime_for(path: &str) -> &'static str {
    match path
        .rsplit('.')
        .next()
        .unwrap_or_default()
        .to_ascii_lowercase()
        .as_str()
    {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "webp" => "image/webp",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "woff2" => "font/woff2",
        "woff" => "font/woff",
        "ttf" => "font/ttf",
        "otf" => "font/otf",
        _ => "application/octet-stream",
    }
}

#[allow(dead_code)]
fn data_url(mime: &str, bytes: &[u8]) -> String {
    format!(
        "data:{mime};base64,{}",
        base64::engine::general_purpose::STANDARD.encode(bytes)
    )
}
