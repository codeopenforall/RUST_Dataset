#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authentication_enforcement() {
        // Instantiate the endpoint with the valid key.
        let endpoint = Endpoint::new("secret");
        // This token should be rejected.
        let malicious = "secXXX";
        // The expected contract: authentication must fail for incorrect credentials.
        // For the vulnerable version this assertion would fail (i.e. it would improperly grant access),
        // whereas the fixed version passes.
        assert_eq!(endpoint.process(malicious), false, "Authentication bypassed for token: {}", malicious);
    }
}