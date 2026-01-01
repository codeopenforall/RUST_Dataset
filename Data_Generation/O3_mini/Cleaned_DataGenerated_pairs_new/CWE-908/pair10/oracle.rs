#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_computation() {
        let result = compute();
        assert_eq!(result, (168, false), "The computation result did not match the expected safe output.");
    }
}
