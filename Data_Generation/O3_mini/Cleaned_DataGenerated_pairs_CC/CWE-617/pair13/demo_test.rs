////////////////////////////////////////
// Test Oracle for Reachable Assertion  //
////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::panic;
    // Bring the Engine struct into scope.
    use super::*;

    #[test]
    fn test_reachable_assertion() {
        // Using an even index (2) that is within bounds.
        // In the vulnerable version, the spawned thread sets the value to 100,
        // causing the assertion to fail and thus panicking.
        // In the corrected version, the value is modified to 40, keeping the invariant intact.
        let engine = Engine::new();
        let result = panic::catch_unwind(|| {
            engine.run(2);
        });
        // The test asserts that the Engine::run call does not panic.
        // This oracle will fail when testing the vulnerable version and pass when testing the fix.
        assert!(result.is_ok(), "Engine::run should not panic for index 2");
    }
}