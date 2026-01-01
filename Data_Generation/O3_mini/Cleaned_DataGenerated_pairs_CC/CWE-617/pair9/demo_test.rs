#[cfg(test)]
mod tests {
    use super::*;

    // This test oracle uses an input that causes the vulnerable code to panic
    // while the fixed code—ignoring the external threshold—succeeds.
    #[test]
    fn contract_test() {
        let instance = Processor::new(10);
        // With index 5, the data value is 5.
        // In the vulnerable version, providing threshold '5' triggers the assertion (5 < 5 fails).
        // In the fixed version, the external threshold is ignored, and the safe internal limit (10) is used.
        let value = instance.process(5, 5);
        assert_eq!(value, 5, "Processing did not return the expected value.");
    }
}