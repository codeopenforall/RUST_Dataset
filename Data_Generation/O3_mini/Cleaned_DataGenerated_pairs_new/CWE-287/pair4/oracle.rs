#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_authentication_enforcement() {
        let endpoint = Endpoint::new("secret");
        let malicious = "secXXX";
        assert_eq!(endpoint.process(malicious), false, "Authentication bypassed for token: {}", malicious);
    }
}
