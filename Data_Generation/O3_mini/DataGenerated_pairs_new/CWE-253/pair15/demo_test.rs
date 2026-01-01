//////////////////////
// Test Oracle Function
//////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    // The test oracle checks that when provided an invalid value (0),
    // the concurrent update function reports failure.
    // For the vulnerable version, the test should fail because it erroneously reports success.
    // For the fixed version, the test should pass.
    #[test]
    fn contract_check() {
        let shared = Arc::new(Mutex::new(Container { number: 100 }));
        // Use an invalid value to trigger the error.
        let result = concurrent_update(shared.clone(), 0);
        // The contract: invalid update should NOT be reported as success.
        assert_eq!(result, false, "Update incorrectly reported as successful upon invalid input");
    }
}