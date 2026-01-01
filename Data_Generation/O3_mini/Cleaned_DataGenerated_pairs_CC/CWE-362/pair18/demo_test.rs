//////////////////////////////// Test Oracle //////////////////////////////////
#[cfg(test)]
mod tests {
    // Import run_system from the current module. When linking against the vulnerable build, this test should fail,
    // while linking against the fixed build, the test should pass.
    use super::run_system;

    #[test]
    fn oracle_test() {
        // The expected final count should be 1000 (10 threads * 100 increments each).
        let result = run_system();
        assert_eq!(result, 1000, "Final count did not match expected value! Race condition exists.");
    }
}