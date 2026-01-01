#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authentication() {
        // The test verifies that the /debug endpoint does not incorrectly bypass authentication.
        // It creates three inquiries: one to /debug with no token, one to /data with an invalid token,
        // and one to /data with a correct token.

        // For the vulnerable implementation, the /debug inquiry would return Outcome::Granted,
        // but the secure implementation should return Outcome::Refused.
        let inquiry_debug = Inquiry { uri: "/debug".to_string(), header: None };
        let inquiry_bad = Inquiry { uri: "/data".to_string(), header: Some("wrong".to_string()) };
        let inquiry_good = Inquiry { uri: "/data".to_string(), header: Some("secret".to_string()) };

        let outcome_debug = execute(inquiry_debug);
        let outcome_bad = execute(inquiry_bad);
        let outcome_good = execute(inquiry_good);

        // The test requires that the /debug inquiry must fail authentication.
        match outcome_debug {
            Outcome::Granted(_) => {
                // This should fail for the fix; an authenticated bypass is not allowed.
                assert!(false, "Debug endpoint should not bypass authentication.");
            },
            Outcome::Refused(_) => {}
        }
        // Check that a bad token fails and a good token succeeds.
        assert!(matches!(outcome_bad, Outcome::Refused(_)),
                "Request with an incorrect token should be refused.");
        assert!(matches!(outcome_good, Outcome::Granted(_)),
                "Request with a correct token should be granted.");
    }
}