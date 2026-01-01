/* Test Oracle Function

The following test oracle function asserts that when processing input 0,
the vulnerable version erroneously returns a success (i.e. Ok(0)) while the fixed version
correctly returns an error. This function is not part of the vulnerable or fixed binaries
and must be run separately.

Usage:
    - When linked against the vulnerable binary, calling test_oracle() should panic.
    - When linked against the fixed binary, calling test_oracle() should complete silently.
*/

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_exception_handling() {
        let res = process(0);
        // The correct behavior is to return an error due to a timeout.
        // Vulnerable version erroneously yields Ok(0)
        assert!(
            res.is_err(),
            "Expected an error for input 0; vulnerability causes a false success"
        );
        if let Err(msg) = res {
            assert_eq!(msg, "Operation timed out", "Expected timeout error message");
        }
    }
}