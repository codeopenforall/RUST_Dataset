//////////////////////////////////////////////////////////
// Test Oracle Function
// The following test function is designed to work with the
// compiled binaries separately. When executed, it will call
// the compute function with the triggering input (e.g., 1500).
// In the vulnerable version, compute(1500) will produce 1500
// (indicating all threads were spawned) causing potential DoS,
// while the fixed version clamps the output to 1000.
// The oracle asserts that the result equals 1000.
//////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::compute;

    #[test]
    fn test_resource_consumption() {
        let input: u64 = 1500;
        let result = compute(input);
        // The fixed version should return exactly MAX_THREADS (1000)
        // while the vulnerable version returns the full count (1500).
        assert_eq!(result, 1000, "Resource consumption is not properly limited");
    }
}