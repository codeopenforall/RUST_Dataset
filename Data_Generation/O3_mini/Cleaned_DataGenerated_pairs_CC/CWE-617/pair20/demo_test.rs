//////////////////////////////////////////////////////////////
// Test Oracle: Reachable Assertion Test
// This test function simulates an attacker-supplied input (15) which would trigger 
// the assertion in the vulnerable version. In the vulnerable version, this test 
// is expected to panic, whereas in the fixed version it should pass successfully.
//////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_threshold() {
        // Use an input that exceeds the safe limit.
        let result = std::panic::catch_unwind(|| {
            let shared = gather_input(15);
            let data = shared.lock().unwrap();
            // The assertion below should panic in the flawed version.
            assert!(data.threshold <= 10, "Threshold exceeded safe limit in test");
        });
        // In the fixed version, the secure_update clamps the value: test must pass.
        assert!(result.is_ok(), "Oracle: The code reached an unsafe state causing a panic");
    }
}