use serde::Serialize;

/// User-facing error payload returned to the frontend.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppError {
    pub code: String,
    pub message: String,
    pub hint: Option<String>,
}

impl AppError {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            hint: None,
        }
    }

    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    pub fn config(message: impl Into<String>) -> Self {
        Self::new("configError", message)
    }

    pub fn network(message: impl Into<String>) -> Self {
        Self::new("networkError", message)
    }

    pub fn auth(message: impl Into<String>) -> Self {
        Self::new("authenticationFailed", message).with_hint("请检查 Sunshine 用户名与密码")
    }

    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::new("originNotAllowed", message).with_hint("请在 Sunshine 配置中允许该来源 IP（origin_web_ui_allowed）")
    }

    pub fn needs_setup() -> Self {
        Self::new("needsSetup", "Sunshine 尚未创建管理员账号").with_hint("请先完成首次初始化，设置用户名与密码")
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new("notFound", message)
    }

    pub fn component(message: impl Into<String>) -> Self {
        Self::new("componentError", message)
    }

    pub fn process(message: impl Into<String>) -> Self {
        Self::new("processError", message)
    }

    pub fn theme(message: impl Into<String>) -> Self {
        Self::new("themeError", message)
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::new("internalError", message)
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}

impl std::error::Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        Self::new("ioError", value.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(value: serde_json::Error) -> Self {
        Self::new("jsonError", value.to_string())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(value: reqwest::Error) -> Self {
        if value.is_timeout() {
            return Self::new("networkTimeout", "连接超时").with_hint("请确认主机在线且端口可访问");
        }
        if value.is_connect() {
            return Self::new("networkConnect", format!("无法连接主机: {value}"))
                .with_hint("请确认 Sunshine 正在运行，且防火墙已放行 47989-48010 端口");
        }
        Self::network(value.to_string())
    }
}

impl From<zip::result::ZipError> for AppError {
    fn from(value: zip::result::ZipError) -> Self {
        Self::component(format!("压缩包处理失败: {value}"))
    }
}

impl From<keyring::Error> for AppError {
    fn from(value: keyring::Error) -> Self {
        Self::new("credentialError", format!("凭据管理失败: {value}"))
    }
}

impl From<tauri::Error> for AppError {
    fn from(value: tauri::Error) -> Self {
        Self::internal(value.to_string())
    }
}

impl From<tokio::task::JoinError> for AppError {
    fn from(value: tokio::task::JoinError) -> Self {
        Self::internal(value.to_string())
    }
}

impl From<mdns_sd::Error> for AppError {
    fn from(value: mdns_sd::Error) -> Self {
        Self::new("discoveryError", format!("mDNS 发现失败: {value}"))
    }
}

pub type AppResult<T> = Result<T, AppError>;
