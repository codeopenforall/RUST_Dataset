#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calculate() {
        let value = calculate();
        assert_eq!(value, 42, "Test oracle: expected value to be 42");
    }
}
