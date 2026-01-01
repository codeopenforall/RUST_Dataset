#[cfg(test)]
mod tests {
    use super::run;
    #[test]
    fn test_out_of_bounds() {
        let result = run(3);
        assert!(result.is_err(), "Expected an error for out-of-bounds access (index 3), but got: {:?}", result);
    }
}
