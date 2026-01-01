//////////////////////////////////////////////////////////////
// Test Oracle: The following unit test verifies the underflow protection.
// It creates an instance with an initial counter of 0, calls the subtraction 
// method with -1, and asserts that the counter remains 0.
// In the vulnerable code, the negative value is cast to a large u32 causing the
// counter to become 4294967295, which fails the test.
//////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_underflow_protection() {
        let mut data = Data::new(0);
        data.subtract_val(-1);
        // For the fixed version the subtraction with a negative value is skipped.
        // The test asserts that the counter remains 0.
        assert_eq!(data.counter, 0, "Integer underflow vulnerability detected!");
    }
}