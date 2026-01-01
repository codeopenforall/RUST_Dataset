#[cfg(test)]
mod tests {
    // Import the execute function from the module (it should be available in both vulnerable and fixed builds).
    use super::execute;

    #[test]
    fn test_oracle() {
        // When the code is fixed, execute() must consistently return 100.
        // In the vulnerable variant, due to the race and uninitialized memory, this may not hold.
        let res = execute();
        assert_eq!(res, 100, "Expected the result to be 100, but got {}", res);
    }
}