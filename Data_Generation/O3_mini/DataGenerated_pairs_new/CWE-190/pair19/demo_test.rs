/*
   The test oracle function uses a deterministic input that triggers an integer overflow in the
   vulnerable code. It asserts that calling the compute method returns an error.
   For the vulnerable version, the unchecked wrapping arithmetic causes an incorrect Ok result,
   causing the oracle to fail the test; whereas the fixed version returns an Err which satisfies the test.
*/
#[cfg(test)]
mod tests {
    use super::Processor;

    #[test]
    fn test_overflow_detection() {
        // Use an input that will cause input * 1000 to overflow a u32.
        let input: u32 = 5_000_000;
        let result = Processor::compute(input);
        // The correct behavior is to detect the overflow and return an error.
        assert!(result.is_err(), "Overflow was not detected as expected");
    }
}
  
// For running the test oracle separately.
#[allow(dead_code)]
fn test_oracle() {
    tests::test_overflow_detection();
}