#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    #[test]
    fn test_process_update_empty_input() {
        let res = Arc::new(Resource::new());
        let result = process_update(&res, b"");
        assert_eq!(result, false, "Expected update failure for empty input.");
    }
}
