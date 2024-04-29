use std::error::Error;
use std::fmt::format;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_urlencoded;
#[derive(Debug)]
pub struct Config {
    pub url: String,
    pub realm: String,
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginData {
    grant_type: String,
    client_id: String,
    client_secret: String,
    password: String,
    username: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse {
    access_token: String,
    expires_in: i64,
    refresh_expires_in: i64,
    refresh_token: String,
    token_type: String,
    //"not-before-policy": i64,
    session_state: String,
    scope: String
}

pub struct AuthenticationService {
    config: Config,
    client: Client
}

impl AuthenticationService {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            client: Client::new()
        }
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<LoginResponse, Box<dyn Error>> {
        println!("{} {}", username, password);
        let data = LoginData {
            grant_type: "password".to_string(),
            client_id: self.config.client_id.clone(),
            client_secret: self.config.client_secret.clone(),
            password: password.to_string(),
            username: username.to_string()
        };

        let resp = self.client.post("http://127.0.0.1:8080/realms/sncf/protocol/openid-connect/token")
            .form(&data)
            .send()
            .await?;

        if resp.status().is_success() {
            let login_response = resp.json().await?;
            Ok(login_response)
        } else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Invalid Credentials")))
        }
    }
}