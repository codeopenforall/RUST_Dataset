#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_error_message_does_not_expose_secret() {
        // The known sensitive secret.
        let sensitive_data = "SuperSecretAPIKey123";
        let cred = std::sync::Arc::new(Credential::new());
        let proc_inst = Processor::new(cred.clone());

        // Capture the error message from processing.
        let err = proc_inst.process().unwrap_err();

        // The vulnerable version returns an error message containing the secret,
        // causing this assertion to fail. The corrected version returns a sanitized message.
        assert!(
            !err.contains(sensitive_data),
            "Secret was exposed in error message"
        );
    }
}