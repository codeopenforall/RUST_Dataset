//////////////////////////////////////////////////////////////
// The test oracle verifies that the concurrent increment routine works as expected.
// It asserts that the final counter value is exactly 10000. For the vulnerable version,
// the unsynchronized updates are likely to yield an incorrect counter value, failing the test.
// For the fixed version, the test passes reliably.
//////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    // Import the run function from the corresponding module.
    // When testing the vulnerable version, this test should frequently fail,
    // while it passes consistently for the fixed version.
    use super::run;

    #[test]
    fn counter_oracle() {
        let result = run();
        // Expected final value after 10 threads * 1000 increments.
        assert_eq!(result, 10000, "Final counter did not reach the expected 10000 due to a race condition");
    }
}