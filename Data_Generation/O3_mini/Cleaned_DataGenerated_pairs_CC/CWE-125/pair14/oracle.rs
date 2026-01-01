#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;
    #[test]
    fn test_oracle() {
        let result = panic::catch_unwind(|| {
            run_op()
        });
        if let Err(_) = result {
            panic!("Test failed: function panicked");
        }
        let value = result.unwrap();
        assert!(value.is_none(), "Test failed: Expected None for out-of-bound access");
    }
}
