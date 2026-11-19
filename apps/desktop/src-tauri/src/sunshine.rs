use std::net::IpAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use base64::Engine as _;
use reqwest::redirect::Policy;
use reqwest::{Client, Method, Response, StatusCode};
use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
use rustls::crypto::CryptoProvider;
use rustls::{DigitallySignedStruct, SignatureScheme};
use rustls_pki_types::{CertificateDer, ServerName, UnixTime};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use tokio::net::TcpStream;
use tokio::time::timeout;

use crate::error::{AppError, AppResult};
use crate::models::{BrowseResult, Host, PendingPairing, TestHostResult};

/// HTTPS client for Sunshine's configuration API.
pub struct SunshineClient {
    http: Client,
    base: String,
    auth: Option<String>,
    pub fingerprint: String,
}

impl SunshineClient {
    pub async fn connect(host: &Host, username: Option<&str>, password: Option<&str>) -> AppResult<Self> {
        let fingerprint = fetch_fingerprint(&host.address, host.port).await?;
        if let Some(expected) = &host.cert_fingerprint {
            if !expected.eq_ignore_ascii_case(&fingerprint) {
                return Err(
                    AppError::new("certificateMismatch", "Sunshine 证书指纹与首次信任的不一致")
                        .with_hint("若确认主机已重装 Sunshine，请在主机详情中重置证书指纹"),
                );
            }
        }

        let http = Client::builder()
            .danger_accept_invalid_certs(true)
            .danger_accept_invalid_hostnames(true)
            .redirect(Policy::none())
            .timeout(Duration::from_secs(20))
            .build()?;

        let auth = username.map(|user| {
            let pass = password.unwrap_or_default();
            format!(
                "Basic {}",
                base64::engine::general_purpose::STANDARD.encode(format!("{user}:{pass}"))
            )
        });

        Ok(Self {
            http,
            base: format!("https://{}:{}", host.address, host.port),
            auth,
            fingerprint,
        })
    }

    fn request(&self, method: Method, path: &str) -> reqwest::RequestBuilder {
        let mut builder = self.http.request(method, format!("{}{}", self.base, path));
        if let Some(auth) = &self.auth {
            builder = builder.header("Authorization", auth);
        }
        builder
    }

    async fn execute(&self, method: Method, path: &str) -> AppResult<Response> {
        let response = self.request(method, path).send().await?;
        let status = response.status();
        match status {
            StatusCode::OK | StatusCode::CREATED | StatusCode::NO_CONTENT => Ok(response),
            StatusCode::FOUND | StatusCode::TEMPORARY_REDIRECT | StatusCode::SEE_OTHER => Err(AppError::needs_setup()),
            StatusCode::UNAUTHORIZED => Err(AppError::auth("Sunshine 认证失败")),
            StatusCode::FORBIDDEN => Err(AppError::forbidden("该来源 IP 未被允许访问")),
            StatusCode::NOT_FOUND => Err(AppError::not_found(format!("接口不存在: {path}"))),
            other => {
                let text = response.text().await.unwrap_or_default();
                let message = serde_json::from_str::<Value>(&text)
                    .ok()
                    .and_then(|value| value.get("error").and_then(|error| error.as_str()).map(str::to_string))
                    .unwrap_or_else(|| {
                        if text.is_empty() {
                            format!("HTTP {}", other.as_u16())
                        } else {
                            text
                        }
                    });
                Err(AppError::new(format!("http{}", other.as_u16()), message))
            }
        }
    }

    async fn get_json<T: DeserializeOwned>(&self, path: &str) -> AppResult<T> {
        Ok(self.execute(Method::GET, path).await?.json::<T>().await?)
    }

    async fn execute_with<B: Serialize>(&self, method: Method, path: &str, body: Option<&B>) -> AppResult<Response> {
        let mut builder = self.request(method, path);
        if let Some(body) = body {
            builder = builder.json(body);
        }
        let response = builder.send().await.map_err(map_reqwest)?;
        let status = response.status();
        match status {
            StatusCode::OK | StatusCode::CREATED | StatusCode::NO_CONTENT => Ok(response),
            StatusCode::FOUND | StatusCode::TEMPORARY_REDIRECT | StatusCode::SEE_OTHER => Err(AppError::needs_setup()),
            StatusCode::UNAUTHORIZED => Err(AppError::auth("Sunshine 认证失败")),
            StatusCode::FORBIDDEN => Err(AppError::forbidden("该来源 IP 未被允许访问")),
            StatusCode::NOT_FOUND => Err(AppError::not_found(format!("接口不存在: {path}"))),
            other => {
                let text = response.text().await.unwrap_or_default();
                let message = serde_json::from_str::<Value>(&text)
                    .ok()
                    .and_then(|value| value.get("error").and_then(|error| error.as_str()).map(str::to_string))
                    .unwrap_or_else(|| {
                        if text.is_empty() {
                            format!("HTTP {}", other.as_u16())
                        } else {
                            text
                        }
                    });
                Err(AppError::new(format!("http{}", other.as_u16()), message))
            }
        }
    }

    async fn post_json<B: Serialize, T: DeserializeOwned>(&self, path: &str, body: &B) -> AppResult<T> {
        Ok(self
            .execute_with(Method::POST, path, Some(body))
            .await?
            .json::<T>()
            .await?)
    }

    async fn delete_json<B: Serialize, T: DeserializeOwned>(&self, path: &str, body: &B) -> AppResult<T> {
        Ok(self
            .execute_with(Method::DELETE, path, Some(body))
            .await?
            .json::<T>()
            .await?)
    }

    pub async fn get_config(&self) -> AppResult<Value> {
        self.get_json("/api/config").await
    }

    pub async fn get_session_status(&self) -> AppResult<Value> {
        self.get_json("/api/session").await
    }

    pub async fn save_config(&self, config: &Value) -> AppResult<Value> {
        self.post_json("/api/config", config).await
    }

    pub async fn get_config_locale(&self) -> AppResult<Value> {
        self.get_json("/api/configLocale").await
    }

    pub async fn get_apps(&self) -> AppResult<Value> {
        self.get_json("/api/apps").await
    }

    pub async fn save_apps(&self, apps: &Value) -> AppResult<Value> {
        self.post_json("/api/apps", apps).await
    }

    pub async fn delete_app(&self, index: u64) -> AppResult<Value> {
        let response = self
            .execute_with::<Value>(Method::DELETE, &format!("/api/apps/{index}"), None)
            .await?;
        Ok(response.json().await.unwrap_or(Value::Null))
    }

    pub async fn close_app(&self) -> AppResult<Value> {
        let response = self
            .execute_with::<Value>(Method::POST, "/api/apps/close", None)
            .await?;
        Ok(response.json().await.unwrap_or(Value::Null))
    }

    pub async fn get_clients(&self) -> AppResult<Value> {
        self.get_json("/api/clients/list").await
    }

    pub async fn unpair_client(&self, uuid: &str) -> AppResult<Value> {
        self.post_json("/api/clients/unpair", &serde_json::json!({ "uuid": uuid }))
            .await
    }

    pub async fn unpair_all(&self) -> AppResult<Value> {
        self.post_json("/api/clients/unpair-all", &serde_json::json!({})).await
    }

    pub async fn update_client(&self, client: &Value) -> AppResult<Value> {
        self.post_json("/api/clients/update", client).await
    }

    pub async fn get_pending_pairings(&self) -> AppResult<Vec<PendingPairing>> {
        #[derive(serde::Deserialize)]
        struct Wrapper {
            #[serde(default)]
            pairings: Vec<PendingPairing>,
        }
        let wrapper: Wrapper = self.get_json("/api/pin").await?;
        Ok(wrapper.pairings)
    }

    pub async fn submit_pin(&self, pairing_id: &str, pin: &str, name: &str) -> AppResult<Value> {
        self.post_json(
            "/api/pin",
            &serde_json::json!({ "pairing_id": pairing_id, "pin": pin, "name": name }),
        )
        .await
    }

    pub async fn cancel_pairing(&self, pairing_id: &str) -> AppResult<Value> {
        self.delete_json("/api/pin", &serde_json::json!({ "pairing_id": pairing_id }))
            .await
    }

    pub async fn change_password(&self, payload: &Value) -> AppResult<Value> {
        self.post_json("/api/password", payload).await
    }

    pub async fn get_logs(&self) -> AppResult<String> {
        Ok(self.execute(Method::GET, "/api/logs").await?.text().await?)
    }

    pub async fn restart(&self) -> AppResult<()> {
        let request = self.request(Method::POST, "/api/restart");
        let _ = timeout(Duration::from_secs(8), request.send()).await;
        Ok(())
    }

    pub async fn browse(&self, path: &str, kind: &str) -> AppResult<BrowseResult> {
        let response = self
            .request(Method::GET, "/api/browse")
            .query(&[("path", path), ("type", kind)])
            .send()
            .await
            .map_err(map_reqwest)?;
        Ok(response.json::<BrowseResult>().await?)
    }

    pub async fn get_cover(&self, index: u64) -> AppResult<String> {
        let response = self.execute(Method::GET, &format!("/api/covers/{index}")).await?;
        let mime = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .unwrap_or("image/png")
            .to_string();
        let bytes = response.bytes().await?;
        let encoded = base64::engine::general_purpose::STANDARD.encode(bytes);
        Ok(format!("data:{mime};base64,{encoded}"))
    }

    pub async fn get_permissions(&self) -> AppResult<Value> {
        self.get_json("/api/permissions").await
    }

    pub async fn request_permission(&self, id: &str) -> AppResult<Value> {
        self.post_json("/api/permissions/request", &serde_json::json!({ "id": id }))
            .await
    }

    pub async fn get_virtual_input_status(&self) -> AppResult<Value> {
        self.get_json("/api/virtual-input/status").await
    }

    pub async fn get_virtual_input_license(&self) -> AppResult<Value> {
        self.get_json("/api/virtual-input/license").await
    }

    pub async fn reset_display_persistence(&self) -> AppResult<Value> {
        self.post_json("/api/reset-display-device-persistence", &serde_json::json!({}))
            .await
    }

    pub async fn get_csrf_token(&self) -> AppResult<Value> {
        self.get_json("/api/csrf-token").await
    }

    pub async fn probe(host: &Host, password: Option<&str>) -> AppResult<TestHostResult> {
        let started = Instant::now();
        let fingerprint = fetch_fingerprint(&host.address, host.port).await?;
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .danger_accept_invalid_hostnames(true)
            .redirect(Policy::none())
            .timeout(Duration::from_secs(10))
            .build()?;
        let mut request = client.get(format!("https://{}:{}/api/config", host.address, host.port));
        if let Some(username) = host.username.as_deref() {
            let auth = format!(
                "Basic {}",
                base64::engine::general_purpose::STANDARD.encode(format!(
                    "{}:{}",
                    username,
                    password.unwrap_or_default()
                ))
            );
            request = request.header("Authorization", auth);
        }
        let response = request.send().await?;
        let status = response.status();
        let raw: Value = response.json().await.unwrap_or(Value::Null);
        let platform = raw
            .get("platform")
            .and_then(|value| value.as_str())
            .unwrap_or("unknown")
            .to_string();
        let version = raw
            .get("version")
            .and_then(|value| value.as_str())
            .unwrap_or("unknown")
            .to_string();
        let mut raw_object = raw.as_object().cloned().unwrap_or_default();
        raw_object.insert("fingerprint".into(), Value::String(fingerprint));
        Ok(TestHostResult {
            ok: status.is_success(),
            latency_ms: started.elapsed().as_millis(),
            platform,
            version,
            raw: raw_object,
        })
    }
}

fn map_reqwest(error: reqwest::Error) -> AppError {
    AppError::from(error)
}

#[derive(Debug)]
struct CaptureVerifier {
    captured: Arc<Mutex<Option<Vec<u8>>>>,
    provider: Arc<CryptoProvider>,
}

impl ServerCertVerifier for CaptureVerifier {
    fn verify_server_cert(
        &self,
        end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        let digest = Sha256::digest(end_entity.as_ref());
        *self.captured.lock().unwrap() = Some(digest.to_vec());
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        rustls::crypto::verify_tls12_signature(message, cert, dss, &self.provider.signature_verification_algorithms)
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        rustls::crypto::verify_tls13_signature(message, cert, dss, &self.provider.signature_verification_algorithms)
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.provider.signature_verification_algorithms.supported_schemes()
    }
}

/// Perform a TLS handshake and return the SHA-256 fingerprint of the peer certificate.
pub async fn fetch_fingerprint(host: &str, port: u16) -> AppResult<String> {
    let provider = Arc::new(rustls::crypto::ring::default_provider());
    let captured: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));
    let verifier = Arc::new(CaptureVerifier {
        captured: captured.clone(),
        provider: provider.clone(),
    });

    let config = rustls::ClientConfig::builder_with_provider(provider)
        .with_safe_default_protocol_versions()
        .map_err(|error| AppError::network(error.to_string()))?
        .dangerous()
        .with_custom_certificate_verifier(verifier)
        .with_no_client_auth();

    let connector = tokio_rustls::TlsConnector::from(Arc::new(config));
    let stream = timeout(Duration::from_secs(8), TcpStream::connect((host, port)))
        .await
        .map_err(|_| AppError::new("networkTimeout", "连接主机超时"))??;

    let server_name = match host.parse::<IpAddr>() {
        Ok(ip) => ServerName::IpAddress(ip.into()),
        Err(_) => ServerName::try_from(host.to_string())
            .map_err(|error| AppError::network(format!("无效的主机名: {error}")))?,
    };

    let _tls = match timeout(Duration::from_secs(8), connector.connect(server_name, stream)).await {
        Ok(Ok(stream)) => stream,
        Ok(Err(error)) => {
            return Err(AppError::network(format!("TLS 握手失败: {error}")));
        }
        Err(_) => return Err(AppError::new("networkTimeout", "TLS 握手超时")),
    };

    let fingerprint = captured
        .lock()
        .unwrap()
        .clone()
        .ok_or_else(|| AppError::network("未能读取主机证书"))?;
    Ok(hex::encode(fingerprint))
}
