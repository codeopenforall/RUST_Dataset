#[cfg(test)]
mod tests {
    use crate::Authenticator;
    #[test]
    fn authentication_test() {
        let auth = Authenticator::new("super_secret");
        assert!(
            auth.verify("super_secret"),
            "Authentication failed with a valid token."
        );
        assert!(
            !auth.verify("test_key"),
            "Authentication vulnerability: bypass key was accepted."
        );
    }
}
