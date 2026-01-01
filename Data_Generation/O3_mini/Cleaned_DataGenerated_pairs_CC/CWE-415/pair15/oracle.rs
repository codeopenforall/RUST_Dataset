#[cfg(test)]
mod tests {
    use std::panic;
    #[test]
    fn test_simulate() {
        let result = panic::catch_unwind(|| {
            crate::simulate();
        });
        assert!(result.is_ok(), "simulate() panicked, indicating a double free vulnerability");
    }
}
