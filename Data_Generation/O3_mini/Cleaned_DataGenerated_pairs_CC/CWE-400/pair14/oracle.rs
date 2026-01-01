#[cfg(test)]
mod tests {
    use super::Handler;
    use super::ResourceManager;
    #[test]
    fn test_uncontrolled_resource_consumption() {
        let manager = ResourceManager;
        let input: Vec<u64> = (1..=1500).collect();
        let result = manager.process(&input);
        assert!(result.is_err(), "Expected error on input exceeding limit, but got Ok with result: {:?}", result);
    }
}
