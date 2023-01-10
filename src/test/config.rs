#[cfg(test)]
mod test_config {
    use crate::config::Config;

    #[test]
    pub fn load_config() -> Result<(), anyhow::Error> {
        let cfg = Config::new("./default_config.toml")?;

        assert_eq!(cfg.wallet.len(), 2);
        assert_eq!(cfg.chain.len(), 9);
        assert_eq!(cfg.taurus.passwd, "password");
        assert_eq!(cfg.taurus.mail, "mail@mail.com");
        assert_eq!(cfg.taurus.api_url, "taurus.url");

        Ok(())
    }
}
