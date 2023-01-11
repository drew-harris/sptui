use std::{fs, io::stdin, path::Path};

use anyhow::{Ok, Result};
use rspotify::Token;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub device_id: Option<String>,
    pub token: Option<Token>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            client_id: "".to_string(),
            client_secret: "".to_string(),
            device_id: None,
            token: None,
        }
    }

    pub fn load_config_from_file(&mut self) -> Result<()> {
        // TODO: Un hard code this
        let config_string = fs::read_to_string(Path::new("config.yaml"))?;
        let config_yml: Config = serde_yaml::from_str(&config_string)?;

        self.client_id = config_yml.client_id;
        self.client_secret = config_yml.client_secret;

        return Ok(());
    }

    pub fn save_config(&self) -> Result<()> {
        let config_string = serde_yaml::to_string(self)?;
        fs::write(Path::new("config.yaml"), config_string)?;
        Ok(())
    }

    pub fn prompt_for_config(&mut self) -> Result<()> {
        let mut client_id: String = String::new();
        println!("\nEnter your client id: ");
        stdin().read_line(&mut client_id)?;
        client_id = client_id.trim().to_string();

        let mut client_secret: String = String::new();
        println!("\nEnter your client secret: ");
        stdin().read_line(&mut client_secret)?;
        client_secret = client_secret.trim().to_string();

        self.client_secret = client_secret;
        self.client_id = client_id;

        return Ok(());
    }
}
