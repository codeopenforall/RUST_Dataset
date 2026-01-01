#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    #[test]
    fn test_oracle() {
        let bad_sig = vec![1u8; 63];
        let doc = Data::new(vec![0u8; 10], bad_sig);
        let shared = Arc::new(Mutex::new(doc));
        let result = process(shared);
        assert_eq!(result, false, "Verification should fail when signature length is not exactly 64 bytes");
    }
}
