/*
The following test oracle ensures that the sensitive token is not leaked in error messages.
It should fail when run against the vulnerable code (because the secret is exposed)
and pass when run against the fixed code (since the secret is redacted).

To run the test, include this in your test suite (e.g., in a module with #[cfg(test)]).
*/

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn check_error_message() {
        let cred = Credential { secret: "super_secret_1234".to_string() };
        let proc = Processor::new(cred);
        let result = proc.process();
        assert!(result.is_err(), "Expected an error due to processing failure");
        let err_msg = format!("{}", result.unwrap_err());
        // This assertion fails in the vulnerable version (where the secret is leaked)
        // and passes in the fixed version (where the secret is redacted).
        assert!(
            !err_msg.contains("super_secret_1234"),
            "Secret leaked in error message!"
        );
    }
}