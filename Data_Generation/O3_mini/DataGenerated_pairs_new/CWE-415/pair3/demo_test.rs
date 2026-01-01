#[cfg(test)]
mod tests {
    use std::panic;

    #[test]
    fn resource_management_test() {
        // When executing the code, the fixed version should run without panicking.
        // In the vulnerable version, the double free will trigger a panic.
        let result = panic::catch_unwind(|| {
            // Invokes the main function which in turn calls process().
            crate::main();
        });
        assert!(
            result.is_ok(),
            "Test failed: the process triggered a double free panic!"
        );
    }
}