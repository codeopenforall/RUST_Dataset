//////////////////////////////
// Test Oracle Function     //
// This test function should be compiled and run separately.
// It verifies that supplying an unreasonably high count results in an error in the corrected code,
// while the vulnerable version does not enforce the limit and would erroneously succeed.
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::Manager;
    use super::Processor;

    #[test]
    fn test_resource_cap() {
        let instance = Manager;
        // Use an input count that exceeds the allowed maximum.
        let excessive_count = 1500;
        let result = instance.process(excessive_count);
        // In the corrected version, an error is expected due to the cap,
        // while in the vulnerable version, the resource limit is not enforced.
        assert!(result.is_err(), "Expected an error when exceeding the resource cap.");
    }
}