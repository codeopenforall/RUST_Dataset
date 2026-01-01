#[cfg(test)]
mod oracle_test {
    use super::*;
    #[test]
    fn test_invalid_input() {
        let handler = Handler { id: 42 };
        let resp = handler.execute("15");
        assert!(resp.is_err(), "Expected error for an out-of-bound index input");
    }
}
