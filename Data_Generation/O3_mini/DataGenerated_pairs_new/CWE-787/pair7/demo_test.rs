/////////////////////
// Test Oracle
/////////////////////
#[cfg(test)]
mod tests {
    // The external function 'run' is expected to return the sum of the filled buffer.
    // For a proper fill of 10 elements with the value 42, the expected sum is 420.
    // In the vulnerable implementation, the out-of-bounds write may corrupt memory,
    // causing the computed sum to deviate from this expected value or trigger undefined behavior.
    #[test]
    fn out_of_bounds_oracle() {
        let result = crate::run();
        assert_eq!(result, 420, "Expected sum is 420; vulnerability causes a deviation.");
    }
}