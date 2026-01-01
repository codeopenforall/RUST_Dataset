#[cfg(test)]
mod tests {
    use super::Buffer;
    use std::sync::Arc;
    #[test]
    fn oracle_test() {
        let buf = Arc::new(Buffer { data: vec![100, 200, 300] });
        let result = buf.read_value(0);
        assert_eq!(result, 0, "Unsafe out-of-bounds access detected: expected 0");
    }
}
