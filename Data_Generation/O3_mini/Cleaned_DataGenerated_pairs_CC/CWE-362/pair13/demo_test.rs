/////////////////// Test Oracle ///////////////////
// This oracle function is designed to be used in a unit test environment.
// It runs the shared counter update routine and asserts that the final value is exactly 10000.
// When run against the vulnerable version, the race condition typically causes the test to fail.
// When run against the fixed version employing atomic operations, the test passes.

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn verify_counter() {
        let final_value = run();
        // Assert that the counter matches the expected total.
        // For the vulnerable version this assertion is expected to fail intermittently.
        assert_eq!(final_value, 10000, "Final counter value does not match expected value.");
    }
}