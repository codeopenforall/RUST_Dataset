#[cfg(test)]
mod tests {
    use super::{DataPool, Modifier};
    #[test]
    fn test_bounds_check() {
        let mut dp = DataPool::new(10);
        assert!(dp.modify(5, 100).is_ok(), "In-bound modification must succeed");
        let result = dp.modify(11, 200);
        assert!(result.is_err(), "Out-of-bound modification should return an error");
    }
}
