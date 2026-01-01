#[cfg(test)]
mod tests {
    use std::panic;
    #[test]
    fn resource_management_test() {
        let result = panic::catch_unwind(|| {
            crate::main();
        });
        assert!(
            result.is_ok(),
            "Test failed: the process triggered a double free panic!"
        );
    }
}
