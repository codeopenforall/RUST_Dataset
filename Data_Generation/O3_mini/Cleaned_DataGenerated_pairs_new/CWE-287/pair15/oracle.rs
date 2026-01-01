#[cfg(test)]
mod tests {
    use super::process;
    #[test]
    fn oracle() {
        let test_token = "very";
        let result = process(test_token);
        assert!(!result, "Test failed: An incomplete token was incorrectly accepted");
    }
}
