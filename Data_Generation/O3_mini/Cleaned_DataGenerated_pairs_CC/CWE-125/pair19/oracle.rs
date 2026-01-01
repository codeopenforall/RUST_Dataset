#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;
    #[test]
    fn test_computation() {
        let holder = DataHolder { data: vec![1, 2, 3, 4, 5] };
        let result = panic::catch_unwind(|| holder.compute());
        assert!(result.is_ok(), "The computation panicked unexpectedly");
        let value = result.unwrap();
        assert_eq!(value, 24, "The computed sum is incorrect");
    }
}
