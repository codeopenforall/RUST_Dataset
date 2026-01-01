#[cfg(test)]
mod tests {
    use super::*;

    // The test oracle expects the configuration's timeout to be exactly 30 so that
    // execute(load_config()) returns 30 + 1 == 31. In the vulnerable version, the uninitialized
    // timeout field causes undefined behavior and the assertion fails.
    #[test]
    fn config_timeout_must_be_correct() {
        let cfg = load_config();
        let result = execute(cfg);
        assert_eq!(result, 31, "The timeout has not been properly initialized; expected 31.");
    }
}