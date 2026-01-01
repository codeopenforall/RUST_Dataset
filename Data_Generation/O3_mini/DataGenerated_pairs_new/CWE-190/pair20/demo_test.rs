/*
   Test oracle function which verifies the correct behavior.
   It calls compute_final() and asserts that the final result is u32::MAX.
   In the vulnerable implementation, unchecked addition leads to a wrapped value 
   that does not equal u32::MAX, causing the test to fail.
   In the corrected version, the test passes.
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oracle() {
        let res = compute_final();
        assert_eq!(res, u32::MAX, "Test oracle failed: Expected saturated value of u32::MAX.");
    }
}