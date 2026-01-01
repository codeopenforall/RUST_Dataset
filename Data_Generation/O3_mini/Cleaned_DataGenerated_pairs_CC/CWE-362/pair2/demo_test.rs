////////////////////////////////////
// Test Oracle for Concurrency Bug  //
// This unit test fails for the race //
// condition version and passes for   //
// the synchronized one.            //
////////////////////////////////////

#[cfg(test)]
mod tests {
    // Import the run_process function from the current crate.
    use super::run_process;

    #[test]
    fn concurrency_invariant() {
        // Run the process multiple times to overcome nondeterminism.
        // The correctly synchronized version should always produce 19.
        // The unsynchronized version (race condition) may produce an incorrect value.
        let result = run_process();
        // The expected value is determined as follows:
        // Starting count = 0, first thread finds even -> +1 results in 1.
        // All subsequent threads will see an odd value and add 2.
        // With 10 threads, final count should be 1 + 9*2 = 19.
        assert_eq!(result, 19, "Invariant violated: expected 19, got {}", result);
    }
}