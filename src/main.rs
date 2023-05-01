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
struct HealthCheck {
    sealed: bool,
}

fn is_sealed(health_url: &str) -> bool {
    fn parse_hc(x: Response) -> bool {
        match x.into_json() {
            Ok(HealthCheck { sealed }) => sealed,
            Err(_) => false,
        }
    }

    let resp = ureq::get(health_url).call();
    match resp {
        Ok(x) => parse_hc(x),
        Err(Status(503, resp)) => parse_hc(resp),
        Err(Status(429, _)) => {
            info!("got code 429: too many requests, waiting");
            // too many requests
            thread::sleep(Duration::from_secs(15));
            false
        }
        Err(Status(code, resp)) => {
            info!(
                "error checking health, got code: '{code}', with message: {}",
                resp.status_text()
            );
            false
        }
        Err(e) => {
            warn!("Got error: {e}");
            false
        }
    }
}

fn unseal(keyfile: &KeyFile, unseal_url: &str) {
    let len = keyfile.keys.len();
    for (i, key) in keyfile.keys.iter().enumerate() {
        let i = i + 1;
        match ureq::post(unseal_url).send_json(json!({ "key": key })) {
            Ok(resp) if resp.status() == 200 => {
                if i < len {
                    info!("unsealed vault partially {i}/{len}");
                } else {
                    info!("fully unsealed vault {i}/{len}");
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
