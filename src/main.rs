use std::{io, thread, path::Path};

use serde::{Serialize, Deserialize};
use reqwest;
use tokio::time::{sleep, Duration};

use ashpd::desktop::network_monitor::NetworkMonitor;

#[derive(Serialize, Deserialize)]
struct SharifLogin {
    username: String,
    password: String
}

impl Default for SharifLogin {
    fn default() -> Self {
        Self{
            username: "".to_string(),
            password: "".to_string()
        }
    }
}

async fn run(creds: &SharifLogin) -> ashpd::Result<()> {
    let proxy = NetworkMonitor::new().await?;
    let client = reqwest::Client::builder().build().unwrap();
    loop { 
        NetworkMonitor::receive_changed(&proxy).await?;
        if NetworkMonitor::can_reach(&proxy, "rasgw01.sharif.ir", 443).await? {
            sleep(Duration::from_millis(500)).await;
            let res = client.post("https://rasgw01.sharif.ir/login")
                .form(&creds)
                .send()
                .await;
            match res {
                Err(e) => println!("Error: {}", e),
                Ok(resp) => {
                    if !resp.status().is_success() {
                        println!("Response Error: {}", resp.status());
                    } else {
                        println!("Success!");
                    }
                }
            }
        }
    }
}

async fn get_creds(path: Option<&std::path::Path>) -> io::Result<SharifLogin> {
    let path = path.unwrap_or(Path::new("/etc/sharif-creds/config.json"));

    let result = tokio::fs::read(path).await?;
    let login: SharifLogin = serde_json::from_slice(&result)?;

    Ok(login)
}

#[tokio::main]
async fn main() {
    let creds = get_creds(None).await.unwrap();
    let client = reqwest::Client::builder().build().unwrap();
    let _ = client.post("https://rasgw01.sharif.ir/login")
        .form(&creds)
        .send()
        .await;
    tokio::spawn(async move {
        let client = reqwest::Client::builder().build().unwrap();
        let creds = get_creds(None).await.unwrap();
        loop {
            thread::sleep(Duration::from_secs(3600 * 3)); // Run every 3 hours
            let _ = client.post("https://rasgw01.sharif.ir/login")
                .form(&creds)
                .send()
                .await;
        }
    });
    run(&creds).await.unwrap();
}
