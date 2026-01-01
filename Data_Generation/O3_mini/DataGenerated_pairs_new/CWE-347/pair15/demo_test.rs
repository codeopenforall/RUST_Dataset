#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_algorithm() {
        let tool = CryptoTool::new();
        let msg = b"Hello, oracle!";
        let sig = vec![0u8; msg.len()];
        // Intentionally use an invalid algorithm.
        let result = unsafe { tool.verify_signature(msg, &sig, "rsa") };
        // The proper behavior is to reject an invalid algorithm.
        assert!(!result, "Verification should fail for an invalid algorithm");
    }
}