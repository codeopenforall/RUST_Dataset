#[cfg(test)]
mod tests {
    use super::compute;
    #[test]
    fn test_resource_consumption() {
        let input: u64 = 1500;
        let result = compute(input);
        assert_eq!(result, 1000, "Resource consumption is not properly limited");
    }
}
