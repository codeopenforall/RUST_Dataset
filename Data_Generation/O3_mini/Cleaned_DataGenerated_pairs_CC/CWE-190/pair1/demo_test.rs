#[allow(dead_code)]
fn test_oracle() {
    // The fixed version of run_calculation should prevent overflow and leave the value unchanged at 250.
    let result = run_calculation();
    // In the vulnerable version, the result would be 4 (due to wrap-around), so this test should fail.
    assert_eq!(result, 250, "The accumulator value is incorrect; expected 250 when preventing overflow");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn check_overflow_prevention() {
        test_oracle();
    }
}