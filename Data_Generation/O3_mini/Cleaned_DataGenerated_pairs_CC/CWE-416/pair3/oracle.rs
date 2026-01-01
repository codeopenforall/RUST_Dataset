#[cfg(test)]
mod tests {
    use super::run_op;
    #[test]
    fn test_oracle() {
        let result = run_op();
        assert_eq!(result, 100, "Expected the result to be 100");
    }
}
