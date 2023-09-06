use std::{env, fs};

use serde::{Deserialize, Serialize};
use webrtc::ice_transport::ice_server::RTCIceServer;

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Config {
    #[serde(default = "default_listen")]
    pub listen: String,
    #[serde(default = "default_ice_servers")]
    pub ice_servers: Vec<IceServer>,
    #[serde(default)]
    pub auth: Auth,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Auth {
    pub accounts: Vec<Account>,
    pub tokens: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub username: String,
    pub password: String,
}

fn default_listen() -> String {
    format!(
        "0.0.0.0:{}",
        env::var("PORT").unwrap_or(String::from("3000"))
    )
}

fn default_ice_servers() -> Vec<IceServer> {
    vec![IceServer {
        urls: vec!["stun:stun.l.google.com:19302".to_string()],
        username: "".to_string(),
        credential: "".to_string(),
        credential_type: "".to_string(),
    }]
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IceServer {
    #[serde(default)]
    pub urls: Vec<String>,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub credential: String,
    #[serde(default)]
    pub credential_type: String,
}

impl Into<RTCIceServer> for IceServer {
    fn into(self) -> RTCIceServer {
        RTCIceServer {
            urls: self.urls,
            username: self.username,
            credential: self.credential,
            credential_type: self.credential_type.as_str().into(),
        }
    }
}

impl Config {
    pub(crate) fn parse() -> Self {
        let mut result = fs::read_to_string("config.toml");
        if result.is_err() {
            result = fs::read_to_string("/etc/live777/config.toml");
        }
        if let Ok(cfg) = result {
            toml::from_str(cfg.as_str()).expect("config parse error")
        } else {
            Config {
                ice_servers: default_ice_servers(),
                listen: default_listen(),
                auth: Default::default(),
            }
        }
    }
}
