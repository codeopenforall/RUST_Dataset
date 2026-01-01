#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;
    #[test]
    fn test_out_of_bounds() {
        let instance = Arc::new(Mutex::new(Holder::new(10)));
        let result = panic::catch_unwind(|| {
            operate(Arc::clone(&instance), 10, 99);
        });
        assert!(result.is_err(), "Expected panic on out-of-bound update");
    }
}
