/////////////////////// TEST ORACLE ///////////////////////
// This test oracle should be compiled and run separately from the above codes.
// It will fail for the vulnerable version (as it leaks "SuperSecretPassword123")
// and pass for the fixed version (since the secret is redacted).

#[cfg(test)]
mod oracle {
    use std::sync::Arc;
    // Note: Import the Credential and concurrent_process from the corresponding module.
    use crate::{Credential, concurrent_process};

    #[test]
    fn secret_leak_test() {
        let cred = Arc::new(Credential::new("usr", "SuperSecretPassword123"));
        let result = concurrent_process(cred);
        if let Err(e) = result {
            let error_message = format!("{}", e);
            // For the secure version, the error message should not contain the sensitive password.
            assert!(
                !error_message.contains("SuperSecretPassword123"),
                "Sensitive information leaked!"
            );
        } else {
            panic!("Expected error due to username condition but none occurred.");
        }
    }
}
///////////////////////////////////////////////////////