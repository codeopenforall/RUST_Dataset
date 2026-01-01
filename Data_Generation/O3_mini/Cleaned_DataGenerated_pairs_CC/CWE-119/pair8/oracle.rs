#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bounds_check() {
        let result = process();
        match result {
            Ok(val) => {
                assert!(false, "Test failed: Expected error for out-of-bound update, got Ok with value {}", val);
            }
            Err(err) => {
                assert_eq!(err, "Index out of bounds");
            }
        }
    }
}
