///////////////////////////////////////////////////////////////
// Test Oracle Function
///////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    // Import the execute function from the library.
    use super::execute;

    // This test simulates a case where an untrusted caller provides excessive
    // resource parameters. The fixed version should throttle the parameters and return 0,
    // while the vulnerable version (if unchecked) may produce undefined behavior or panic.
    #[test]
    fn oracle() {
        // Triggering input with excessive values.
        // In the fixed version, these parameters are capped (100 threads, 10_000 allocation),
        // and since each thread initializes a buffer of zeros, the aggregated sum will be 0.
        let result = execute(150, 20000);
        assert_eq!(result, 0, "The aggregated result should be 0 after safe throttling.");
    }
}