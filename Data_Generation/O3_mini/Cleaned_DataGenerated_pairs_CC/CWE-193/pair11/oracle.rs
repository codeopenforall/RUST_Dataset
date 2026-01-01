#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_data_processing() {
        let manager = DataManager::new(10);
        let src = vec![1, 2, 3, 4, 5];
        manager.process(&src);
        let result = manager.fetch();
        assert_eq!(result.len(), src.len(), "Vector length does not match input length");
        assert_eq!(result, src, "Vector content does not match input content");
    }
}
