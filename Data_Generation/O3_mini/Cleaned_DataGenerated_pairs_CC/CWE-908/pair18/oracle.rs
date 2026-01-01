#[cfg(test)]
mod tests {
    use super::load_config;
    #[test]
    fn test_configuration() {
        let cfg = load_config();
        assert_eq!(cfg.desc, "default config", "Configuration description is not properly initialized");
    }
}
