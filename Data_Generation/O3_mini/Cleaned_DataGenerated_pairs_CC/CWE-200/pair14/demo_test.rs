///////////////////////////////////////////////
// Test Oracle Function
///////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    // Bring the definition of Cred and process_data into scope.
    use crate::{Cred, process_data};

    #[test]
    fn test_secret_exposure() {
        let data = Arc::new(Mutex::new(Cred::new("alice", "s3cr3t")));
        let output = process_data(&data);
        // The vulnerable version will include "s3cr3t" in its output,
        // whereas the corrected version masks it with "******".
        assert!(
            !output.contains("s3cr3t"),
            "Sensitive information exposure detected: secret leaked"
        );
        assert!(
            output.contains("******"),
            "Sanitization not applied: secret not masked"
        );
    }
}