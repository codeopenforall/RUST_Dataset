#[cfg(test)]
mod tests {
    use super::compute;
    #[test]
    fn test_computation() {
        let result = compute();
        assert_eq!(result, 45, "The computed sum does not match the expected value");
    }
}
