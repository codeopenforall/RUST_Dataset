#[cfg(test)]
mod tests {
    use super::Data;
    #[test]
    fn test_oracle() {
        let mut obj = Data::new();
        let input = vec![1u8; 100]; 
        let result = obj.operate(&input);
        assert!(result.is_err(), "Operation must reject oversized input");
    }
}
