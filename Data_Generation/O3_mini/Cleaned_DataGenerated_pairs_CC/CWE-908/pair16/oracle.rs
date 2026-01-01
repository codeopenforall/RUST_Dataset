#[cfg(test)]
mod tests {
    use super::compute;
    #[test]
    fn oracle() {
        let result = std::panic::catch_unwind(|| {
            compute()
        });
        if result.is_err() {
            panic!("Test failed: The configuration processing panicked (vulnerability present)");
        }
        let valid = result.unwrap();
        assert!(valid, "Test failed: The configuration was not valid (unexpected value)");
    }
}
