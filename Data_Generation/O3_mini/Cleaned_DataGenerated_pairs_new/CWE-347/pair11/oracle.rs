#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;
    #[test]
    fn test_authentication() {
        let instance = Arc::new(Checker::new());
        let public_key: [u8; 32] = [1; 32];
        let message = b"Hello, World!";
        let bogus_signature: [u8; 64] = [0; 64];
        let is_valid = instance.authenticate(&public_key, message, &bogus_signature);
        assert!(!is_valid, "Bogus signature should be rejected");
    }
}
