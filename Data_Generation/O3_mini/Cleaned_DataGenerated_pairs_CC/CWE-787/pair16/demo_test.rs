////////////////////////////////////////////////////////////////
// Oracle test function using Rust's built-in test framework.
// This test calls the public function that calculates the sum from the vector.
// For the safe (corrected) version, the expected sum is 15.
// In the unsafe version, the out-of-bounds write corrupts the data and the result 
// deviates from 15, making the test fail.
////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    // Import the items from the crate. For this oracle test, 
    // the targeted function is assumed to be accessible.
    use super::*;

    #[test]
    fn test_total() {
        let total = compute_value();
        assert_eq!(total, 15, "The computed total does not match the expected safe sum.");
    }
}