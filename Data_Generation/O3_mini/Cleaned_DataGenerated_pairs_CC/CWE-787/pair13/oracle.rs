#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;
    #[test]
    fn test_buffer_capacity_guard() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let result = super::run(input);
        assert!(
            result.is_err(),
            "Contract violation: Expected an error due to input size exceeding capacity."
        );
    }
}
