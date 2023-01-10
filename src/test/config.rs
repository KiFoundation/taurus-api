#[cfg(test)]
mod test_config {
    use crate::config::{Chain, Config, Wallet};

    #[test]
    pub fn load_config() -> Result<(), anyhow::Error> {
        let cfg = Config::load()?;

        assert_eq!(cfg.wallet.len(), 1);
        assert_eq!(cfg.chain.len(), 2);
        assert_eq!(cfg.taurus.passwd, "password");
        assert_eq!(cfg.taurus.mail, "taurus@taurus.io");
        assert_eq!(cfg.taurus.api_url, "taurus.io");

        Ok(())
    }

    #[test]
    pub fn find_wallet() -> Result<(), anyhow::Error> {
        let cfg = Config::load()?;

        let wallet = cfg.find_wallet("toto".to_string());

        assert!(wallet.is_ok());
        assert_eq!(
            wallet?,
            Wallet {
                name: "toto".to_string(),
                address: "toto".to_string()
            }
        );

        let wallet = cfg.find_wallet("a".to_string());
        assert!(wallet.is_err());

        Ok(())
    }

    #[test]
    pub fn find_chain() -> Result<(), anyhow::Error> {
        let cfg = Config::load()?;

        let chain = cfg.find_chain("kichain-t-4".to_string());

        assert!(chain.is_ok());
        assert_eq!(
            chain?,
            Chain {
                prefix: "tki".to_string(),
                denom: "utki".to_string(),
                lcd: "https://api-challenge.blockchain.ki".to_string(),
                rpc: "https://rpc-challenge.blockchain.ki".to_string(),
                chain_id: "kichain-t-4".to_string()
            }
        );

        let chain = cfg.find_chain("a".to_string());
        assert!(chain.is_err());

        Ok(())
    }
}
