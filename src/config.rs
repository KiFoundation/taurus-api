use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Chain {
    pub prefix: String,
    pub denom: String,
    pub lcd: String,
    pub rpc: String,
    pub chain_id: String,
    pub overwrite_grantee: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Taurus {
    pub api_url: String,
    pub mail: String,
    pub passwd: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Wallet {
    pub name: String,
    pub address: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub taurus: Taurus,
    pub chain: Vec<Chain>,
    pub wallet: Vec<Wallet>,
}

impl Config {
    pub fn new(path: &str) -> Result<Self, anyhow::Error> {
        let config_file = std::fs::read_to_string(path)?;

        Ok(toml::from_str(config_file.as_str())?)
    }
}
