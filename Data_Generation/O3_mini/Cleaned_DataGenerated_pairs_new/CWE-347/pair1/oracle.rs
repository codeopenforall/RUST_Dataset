#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_signature_validation() {
        let data = b"Important message";
        let invalid_sig = b"forged signature";
        let worker = Worker { algorithm: 1 };
        let result = run(&worker, data, invalid_sig);
        assert!(!result, "Signature verification should fail for an invalid signature.");
    }
}
