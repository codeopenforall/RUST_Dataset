#[cfg(test)]
mod tests {
    // Import items from the current module.
    use super::run_op;

    #[test]
    fn test_oracle() {
        // The operation is expected to return 100.
        // In the vulnerable code this test is likely to fail or behave unpredictably due 
        // to use-after-free, whereas the corrected version will pass.
        let result = run_op();
        assert_eq!(result, 100, "Expected the result to be 100");
    }
}