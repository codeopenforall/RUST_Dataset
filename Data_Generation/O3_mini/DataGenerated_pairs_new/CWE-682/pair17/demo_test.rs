#[cfg(test)]
mod tests {
    use super::*;
    
    // Test contract: For an input of 120 with factor 100,
    // the intended percentage gain is computed as ((120 - 100) * 100) / 120 = 2000 / 120 = 16 (integer division).
    #[test]
    fn calculation_test() {
        // Using the same function 'run' for both versions. In the vulnerable version,
        // the error in arithmetic results in 0, causing the test to fail.
        let result = run(120, 100);
        assert_eq!(result, 16, "Incorrect percentage calculation, expected 16 but got {}", result);
    }
}