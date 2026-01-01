#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn config_timeout_must_be_correct() {
        let cfg = load_config();
        let result = execute(cfg);
        assert_eq!(result, 31, "The timeout has not been properly initialized; expected 31.");
    }
}
