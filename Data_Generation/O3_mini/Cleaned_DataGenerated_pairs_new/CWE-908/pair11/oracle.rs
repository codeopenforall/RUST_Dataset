#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_run_app_value() {
        let res = run_app();
        assert_eq!(res, 42, "The configuration flag should be 42");
    }
}
