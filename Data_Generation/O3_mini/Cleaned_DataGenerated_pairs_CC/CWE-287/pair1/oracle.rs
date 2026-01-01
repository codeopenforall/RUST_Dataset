#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_authentication() {
        let service = Service::new("SuperSecretPassword");
        assert!(!service.authenticate("test"), "Backdoor token should not be accepted");
    }
}
