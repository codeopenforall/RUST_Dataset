#[cfg(test)]
mod tests {
    // The test oracle asserts that the function returns 256.
    // With the vulnerable version, the use‐after‐free corrupts the value (returning 9999) and the test will fail.
    // With the fixed version, the test passes.
    #[test]
    fn memory_safety_test() {
        let result = crate::run();
        assert_eq!(result, 256, "Test failed: memory value corrupted due to use‐after‐free vulnerability");
    }
}