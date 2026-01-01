#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    fn authenticate(message: Vec<u8>, signature: Vec<u8>) -> bool {
        let pubkey = vec![1, 2, 3, 4, 5];
        let checker = Arc::new(Mutex::new(Checker::new(pubkey)));
        run(checker, message, signature)
    }
    #[test]
    fn oracle_test() {
        let message = vec![10, 20, 30];
        let invalid_signature = vec![0];
        let result = authenticate(message, invalid_signature);
        assert!(!result, "Authentication should fail for an invalid signature.");
    }
}
