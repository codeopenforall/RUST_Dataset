#[cfg(test)]
mod tests {
    use super::*;

    // This test oracle validates the behavior under malicious input.
    // It expects that the returned string DOES NOT contain the sensitive secret.
    // In the vulnerable version, using malicious input like "Alert: {}" would include the secret,
    // causing this test to fail. The fixed version avoids revealing the secret.
    #[test]
    fn test_format_string_handling() {
        let malicious_input = "Alert: {}";
        let secret = "VerySecretInfo";

        // Call the simulate function from the code under test.
        let output = simulate(malicious_input);

        // The oracle asserts that the secret is not present in the output.
        // For a secure implementation, the secret should not have been formatted into the output.
        assert!(
            !output.contains(secret),
            "Sensitive information was leaked in the output!"
        );
    }
}