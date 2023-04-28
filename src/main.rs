use dotenv::dotenv;
use serde::Deserialize;
use serde_json::json;
use std::error::Error;
use std::io::Read;
use std::thread;
use std::time::Duration;
use std::{env, fs::File};

#[derive(Debug, Deserialize)]
struct KeyFile {
    keys: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct HealthCheck {
    sealed: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
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

    println!("Starting vault unsealer ...");
    loop {
        match ureq::get(&health_url).call() {
            Err(ureq::Error::Status(code, resp)) if code == 503 => {
                if let Ok(HealthCheck { sealed: true }) = resp.into_json() {
                    for key in &keyfile.keys {
                        match ureq::post(&unseal_url).send_json(json!({ "key": key })) {
                            Ok(resp) if resp.status() != 200 => eprintln!("error unsealing vault"),
                            Ok(_) => println!("unsealed vault partially"),
                            Err(err) => eprintln!("error unsealing vault: {err}"),
                        }
                    }
                } else {
                    eprintln!("Can't unseal");
                }
            }
            Err(ureq::Error::Status(_, _)) => (),
            Err(e) => eprintln!("{e}"),
            _ => (),
        }

        thread::sleep(interval);
    }
}
