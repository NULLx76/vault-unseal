use dotenv::dotenv;
use serde::Deserialize;
use serde_json::json;
use std::error::Error;
use std::io::Read;
use std::thread;
use std::time::Duration;
use std::{env, fs::File};
use tracing::{info, subscriber, warn};
use tracing_subscriber::FmtSubscriber;
use ureq::Error::Status;
use ureq::Response;

#[derive(Debug, Deserialize)]
struct KeyFile {
    keys: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct UnsealResponse {
    sealed: bool,
    t: u8,
    n: u8,
    progress: u8,
}

/// returns true if the vault is sealed
///
/// see: https://developer.hashicorp.com/vault/api-docs/system/health
fn is_sealed(health_url: &str) -> bool {
    let resp = ureq::get(health_url).call();
    match resp {
        Ok(r) if r.status() == 200 => false,
        Ok(r) => {
            warn!(
                "unexpected status code: '{}': {}",
                r.status(),
                r.status_text()
            );
            false
        }
        Err(Status(429, _)) => false, // Unsealed and standby
        Err(Status(503, _)) => true,  // Sealed
        Err(Status(code, resp)) => {
            info!(
                "error checking health, got code: '{code}', with message: {}",
                resp.status_text()
            );
            false
        }
        Err(e) => {
            warn!("error checking health: {e}");
            false
        }
    }
}

fn unseal(keyfile: &KeyFile, unseal_url: &str) {
    for key in keyfile.keys.iter().enumerate() {
        match ureq::post(unseal_url).send_json(json!({ "key": key })) {
            Ok(resp) if resp.status() == 200 => {
                if let Ok(UnsealResponse {
                    sealed,
                    t,
                    progress,
                    ..
                }) = resp.into_json()
                {
                    if !sealed {
                        info!("vault unsealed");
                        return;
                    }
                    info!("unsealed vault partially {progress}/{t}");
                }
            }
            Ok(resp) => warn!(
                "error unsealing vault, got code '{}', with message: {}",
                resp.status(),
                resp.status_text()
            ),
            Err(err) => warn!("error unsealing vault: {err}"),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let subscriber = FmtSubscriber::new();
    subscriber::set_global_default(subscriber)?;

    let vault_addr = env::var("VAULT_ADDR")?;
    let file = env::var("VAULT_KEY_FILE")?;
    let interval = env::var("UNSEAL_INTERVAL").unwrap_or(String::from("15"));
    let interval = Duration::from_secs(interval.parse()?);

    let mut file = File::open(file)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let keyfile: KeyFile = serde_json::from_str(&data)?;

    let unseal_url = format!("{vault_addr}/v1/sys/unseal");
    let health_url = format!("{vault_addr}/v1/sys/health");

    info!("Starting vault unsealer at {vault_addr}");
    loop {
        if is_sealed(&health_url) {
            unseal(&keyfile, &unseal_url);
        }

        thread::sleep(interval);
    }
}
