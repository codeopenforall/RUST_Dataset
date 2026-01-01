#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::sync::{Arc, Mutex};
    use std::thread;

    // Bring the concurrent_exec function into scope from the implementation.
    // For testing, the function signature and behavior must be identical in both versions.
    // This test triggers the vulnerability in the unsafe version and expects it to fail,
    // while it should pass (by rejecting the input) in the corrected version.
    #[test]
    fn test_injection() {
        // Malicious payload contains characters that should trigger the vulnerability.
        let injection_payload = "echo Hello; echo hacked";
        let results = super::concurrent_exec(injection_payload);
        for res in results {
            match res {
                Ok(out) => {
                    // In the vulnerable case, the shell executes both parts resulting in injected output.
                    // In the fixed case, the input is rejected—so no valid output should appear.
                    assert!(!out.contains("hacked"), "Injection vulnerability triggered");
                }
                Err(err) => {
                    // For the fixed version, we expect an error indicating invalid characters.
                    assert!(err.contains("Invalid"), "Expected invalid character error");
                }
            }
        }
    }
}