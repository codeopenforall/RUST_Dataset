#[cfg(test)]
mod tests {
    use super::run_app;
    #[test]
    fn oracle() {
        let result = run_app();
        assert_eq!(
            result, 
            "Initialized!", 
            "Test failed: The configuration message did not match the expected initialized value."
        );
    }
}
