#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_oracle() {
        let mut buf = ManagedBuffer::new(10);
        let source = vec![1, 2, 3, 4, 5];
        let result = buf.update(&source, 8, 5);
        assert!(result.is_err(), "The update function should detect the out-of-bound operation");
    }
}
