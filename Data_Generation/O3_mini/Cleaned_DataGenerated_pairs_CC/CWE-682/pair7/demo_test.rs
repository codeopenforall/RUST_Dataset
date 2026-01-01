////////////////////////////////////////////////////////////////////////////////
// Test Oracle: This test case instantiates an Account with a balance of 100 and an
// interest rate of 10. The correct computation should yield 110. When run against the 
// vulnerable version, the test will fail due to the incorrect arithmetic, and it will pass
// when run against the corrected version.
////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn check_interest_computation() {
        let acc = Account { balance: 100 };
        let result = acc.compute(10);
        // The expected value is 110. The vulnerable version yields 11.
        assert_eq!(result, 110, "Interest computation did not produce the expected result");
    }
}