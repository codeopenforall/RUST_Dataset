#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    #[test]
    fn test_oracle() {
        env::set_var("SEARCH_PATH", "malicious");
        let runner = Runner;
        let result = runner.execute().expect("Execution failed");
        assert_eq!(
            result, "/usr/bin:/bin",
            "The environment PATH was not sanitized properly"
        );
    }
}
