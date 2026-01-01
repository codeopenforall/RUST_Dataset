#[cfg(test)]
mod tests {
    use std::env;
    use super::run;
    const SAFE_LIMIT: usize = 5000;
    #[test]
    fn test_resource_limit() {
        let total = run(10000);
        assert!(total <= SAFE_LIMIT, "Resource consumption exceeded safe limit: {}", total);
    }
}
