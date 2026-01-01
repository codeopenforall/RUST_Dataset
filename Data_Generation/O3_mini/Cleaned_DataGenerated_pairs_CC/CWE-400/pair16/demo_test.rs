/*
The following test oracle function is designed to be compiled and run separately from both the vulnerable and fixed programs.
It calls the process function with a value that exceeds the safe threshold.
For the vulnerable implementation, this test is expected to trigger uncontrolled resource consumption (or even hang),
causing the test to fail (or time out). For the fixed version, the function should return an error immediately.
*/
#[cfg(test)]
mod tests {
    // In order to test the process function, we bring it into scope. 
    // Note: Run this test with the corresponding implementation (vulnerable or fixed).
    use super::process;
    
    #[test]
    fn test_oracle() {
        // Use a limit that is above the safe threshold of the fixed version.
        let result = process(1500);
        // For the fixed version, an error is expected due to the limit check.
        // For the vulnerable version, the test may hang or yield a valid result (indicating a flaw).
        assert!(result.is_err(), "Expected an error when the limit exceeds safe threshold");
    }
}