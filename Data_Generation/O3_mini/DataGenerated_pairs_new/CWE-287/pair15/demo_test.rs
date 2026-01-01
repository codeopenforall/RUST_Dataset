/////////////////////////////
// Test Oracle Functionality
/////////////////////////////
#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn oracle() {
        // This input is only a prefix ("very") of the valid secret "verysecurekey".
        // Expect authentication to be rejected.
        let test_token = "very";
        let result = process(test_token);
        assert!(!result, "Test failed: An incomplete token was incorrectly accepted");
    }
}