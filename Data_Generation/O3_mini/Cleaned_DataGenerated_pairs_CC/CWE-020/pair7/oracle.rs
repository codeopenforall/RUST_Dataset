#[cfg(test)]
mod tests {
    use std::panic;
    use super::{Container, DataAccess};
    #[test]
    fn test_valid_index() {
        let container = Container::new();
        assert_eq!(container.read(2), 3);
    }
    #[test]
    fn test_out_of_bounds() {
        let container = Container::new();
        let result = panic::catch_unwind(|| {
            container.read(15);
        });
        assert!(result.is_err(), "Out-of-bound read did not panic as expected");
    }
}
