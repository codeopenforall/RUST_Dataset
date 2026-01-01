////////////////////////////////////////////////////////////////////////////////
// Test Oracle: This unit test checks that the overall computed sum equals the expected
// value (0 + 1 + ... + 9 = 45). For the vulnerable version, due to the off‐by‐one bug,
// undefined behavior may lead to an incorrect sum (or even a crash), while the fixed version
// should always result in a sum of 45.
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    // Import the Accumulator struct and its methods from the current crate.
    use super::*;

    #[test]
    fn test_accumulation() {
        let aggregator = Accumulator::new(10);
        aggregator.process();
        let result = aggregator.total();
        assert_eq!(result, 45, "Expected the sum to be 45, but got {}", result);
    }
}