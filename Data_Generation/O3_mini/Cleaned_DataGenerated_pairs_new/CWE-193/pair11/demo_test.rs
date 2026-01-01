#[cfg(test)]
mod tests {
    // Import all public items from the surrounding module.
    use super::*;

    #[test]
    fn test_data_processing() {
        // Create an instance with sufficient capacity.
        let manager = DataManager::new(10);
        let src = vec![1, 2, 3, 4, 5];
        manager.process(&src);
        let result = manager.fetch();

        // The contract expects the vector length to match the input length.
        // In the vulnerable version, extra uninitialized byte(s) cause the length to be incorrect.
        assert_eq!(result.len(), src.len(), "Vector length does not match input length");
        assert_eq!(result, src, "Vector content does not match input content");
    }
}