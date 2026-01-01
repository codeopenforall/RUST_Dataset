/*
Test oracle function for verifying proper authentication.
This test ensures that providing a token that is only a prefix of the secret (e.g., "pass" when the secret is "password")
fails the authentication check.

For the vulnerable version, this test would incorrectly pass authentication and thus fail the test.
For the fixed version, the test will pass, as the token does not match the full secret.

This oracle function can be used in a unit test harness.
*/
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn authentication_oracle() {
        let gate = Gatekeeper::new();
        // The token "pass" should not be considered a valid authentication.
        let test_token = "pass";
        let result = gate.verify(test_token);
        assert!(!result, "Authentication should fail for partial token input.");
    }
}