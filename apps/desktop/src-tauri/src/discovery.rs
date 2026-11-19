use std::collections::HashMap;
use std::time::{Duration, Instant};

use mdns_sd::{ServiceDaemon, ServiceEvent};

use crate::error::{AppError, AppResult};
use crate::models::{DiscoveredHost, HostSource};

/// Browse the local network for Sunshine hosts advertising `_nvstream._tcp`.
pub async fn scan(timeout_ms: u64) -> AppResult<Vec<DiscoveredHost>> {
    let timeout = Duration::from_millis(timeout_ms.clamp(500, 15_000));
    tokio::task::spawn_blocking(move || scan_blocking(timeout))
        .await
        .map_err(|error| AppError::internal(error.to_string()))?
}

fn scan_blocking(timeout: Duration) -> AppResult<Vec<DiscoveredHost>> {
    let daemon = ServiceDaemon::new()?;
    let receiver = daemon.browse("_nvstream._tcp.local.")?;
    let deadline = Instant::now() + timeout;
    let mut hosts: HashMap<String, DiscoveredHost> = HashMap::new();

    while let Some(remaining) = deadline.checked_duration_since(Instant::now()) {
        if remaining.is_zero() {
            break;
        }
        match receiver.recv_timeout(remaining) {
            Ok(ServiceEvent::ServiceResolved(info)) => {
                let port = info.get_port();
                let hostname = info
                    .get_hostname()
                    .trim_end_matches('.')
                    .trim_end_matches(".local")
                    .to_string();
                let display = info
                    .get_properties()
                    .get_property_val_str("hostname")
                    .filter(|value| !value.is_empty())
                    .unwrap_or(&hostname)
                    .to_string();
                for address in info.get_addresses() {
                    if !address.is_ipv4() {
                        continue;
                    }
                    let key = format!("{address}:{port}");
                    hosts.entry(key).or_insert(DiscoveredHost {
                        name: display.clone(),
                        address: address.to_string(),
                        port,
                        source: HostSource::Mdns,
                    });
                }
            }
            Ok(_) => {}
            Err(_) => break,
        }
    }

    let _ = daemon.shutdown();
    let mut result: Vec<DiscoveredHost> = hosts.into_values().collect();
    result.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(result)
}
