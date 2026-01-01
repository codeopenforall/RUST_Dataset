#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_resource_limit() {
        let dangerous_jobs = 6;
        let result = run_app(dangerous_jobs);
        assert!(result.is_err(), "Expected resource limit error, but got success");
    }
}
