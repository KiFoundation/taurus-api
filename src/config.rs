use anyhow::bail;
use log::{debug, trace};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct Chain {
    pub prefix: String,
    pub denom: String,
    pub lcd: String,
    pub rpc: String,
    pub chain_id: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Taurus {
    pub api_url: String,
    pub mail: String,
    pub passwd: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct Wallet {
    pub name: String,
    pub address: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Config {
    pub taurus: Taurus,
    pub chain: Vec<Chain>,
    pub wallet: Vec<Wallet>,
    pub slack_web_hook: Option<String>,
}

impl Config {
    fn get_config_path_and_file() -> Result<(String, String), anyhow::Error> {
        let homedir = std::env::var("HOME")?;

        Ok((
            format!("{}/.taurus-api", homedir),
            format!("{}/.taurus-api/config.toml", homedir),
        ))
    }

    fn default() -> Result<Self, anyhow::Error> {
        debug!("generating default config");
        Ok(Config {
            taurus: Taurus {
                api_url: "taurus.io".to_string(),
                mail: "taurus@taurus.io".to_string(),
                passwd: "password".to_string(),
            },
            chain: vec![
                Chain {
                    prefix: "tki".to_string(),
                    denom: "utki".to_string(),
                    lcd: "https://api-challenge.blockchain.ki".to_string(),
                    rpc: "https://rpc-challenge.blockchain.ki".to_string(),
                    chain_id: "kichain-t-4".to_string(),
                },
                Chain {
                    prefix: "xki".to_string(),
                    denom: "uxki".to_string(),
                    lcd: "https://api-mainnet.blockchain.ki".to_string(),
                    rpc: "https://rpc-mainnet.blockchain.ki".to_string(),
                    chain_id: "kichain-2".to_string(),
                },
            ],
            wallet: vec![Wallet {
                name: "toto".to_string(),
                address: "toto".to_string(),
            }],
            slack_web_hook: None,
        })
    }

    fn save(config: &Config, config_file: String) -> Result<(), anyhow::Error> {
        debug!("saving config file {}", config_file);
        let conf_content = toml::to_string(config);
        trace!("config: {:#?}", conf_content);

        fs::write(config_file, conf_content?)?;
        Ok(())
    }

    pub fn load() -> Result<Self, anyhow::Error> {
        let (config_path, config_file) = Self::get_config_path_and_file()?;

        if Path::new(&config_file).exists() {
            debug!("loading config file {}", config_file);

            let config_file = std::fs::read_to_string(config_file)?;

            Ok(toml::from_str(config_file.as_str())?)
        } else {
            fs::create_dir_all(config_path)?;

            let cfg = Config::default()?;
            Self::save(&cfg, config_file)?;
            Ok(cfg)
        }
    }

    pub fn find_wallet(&self, name: &String) -> Result<Wallet, anyhow::Error> {
        let index = self.wallet.iter().position(|w| w.name == *name);

        if let Some(idx) = index {
            Ok(self.wallet[idx].clone())
        } else {
            bail!("unknown wallet");
        }
    }

    pub fn find_chain(&self, chain_id: &String) -> Result<Chain, anyhow::Error> {
        let index = self.chain.iter().position(|c| c.chain_id == *chain_id);

        if let Some(idx) = index {
            Ok(self.chain[idx].clone())
        } else {
            bail!("unknown chain_id");
        }
    }
}
