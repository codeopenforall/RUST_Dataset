#[cfg(test)]
mod tests {
    use super::run_app;
    #[test]
    fn test_run_app_result() {
        let res = run_app();
        assert_eq!(res, 0, "Test failed: Expected result to be 0 for properly initialized resource.");
    }
}
