#[cfg(test)]
mod test_config {
    use crate::config::Config;

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
}
