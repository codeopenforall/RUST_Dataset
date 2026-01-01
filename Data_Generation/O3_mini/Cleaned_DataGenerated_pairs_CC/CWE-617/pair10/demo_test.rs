#[cfg(test)]
mod tests {
    use std::panic;
    
    // This test oracle calls the application's run function with an attacker-controlled input.
    // For the vulnerable build (compiled with `--features vulnerable`), the unsynchronized update
    // leads to the THRESHOLD being exceeded and the reachable assertion triggering a panic.
    // For the safe build, the atomic update prevents overshooting the threshold, and no panic occurs.
    #[test]
    fn oracle() {
        // An attacker-supplied delta value that will force the shared value to exceed the threshold in the unsafe version.
        let attacker_input = 100;
    
        let result = panic::catch_unwind(|| {
            // Assumes that run_app is exposed from the current crate.
            crate::run_app(attacker_input);
        });
    
        #[cfg(feature = "vulnerable")]
        {
            assert!(result.is_err(), "Test oracle: Expected panic due to unsafe state update.");
        }
    
        #[cfg(not(feature = "vulnerable"))]
        {
            assert!(result.is_ok(), "Test oracle: Unexpected panic in safe update.");
        }
    }
}