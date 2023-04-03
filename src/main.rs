use std::{path::Path};
use std::io;

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
        if NetworkMonitor::can_reach(&proxy, "net2.sharif.edu", 443).await? {
            sleep(Duration::from_millis(500)).await;
            let res = client.post("https://net2.sharif.edu/login")
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
    let _ = client.post("https://net2.sharif.edu/login")
        .form(&creds)
        .send()
        .await;
    run(&creds).await.unwrap();
}
