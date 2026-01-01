#[cfg(test)]
mod tests {
    use super::*;
    
    // The oracle test ensures that the authentication function does not erroneously approve the backdoor token.
    // For the fixed implementation, passing the "test" token should yield a denial, while the vulnerable version would grant access.
    #[test]
    fn check_authentication() {
        let service = Service::new("SuperSecretPassword");
        // Expectation: the backdoor token "test" must NOT be authorized.
        assert!(!service.authenticate("test"), "Backdoor token should not be accepted");
    }
}