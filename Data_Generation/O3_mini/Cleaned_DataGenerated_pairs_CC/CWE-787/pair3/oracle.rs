#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_buffer() {
        let mut buf = Buffer::new(6);
        let sum = buf.compute();
        assert_eq!(sum, 6, "Buffer sum value is not as expected");
    }
}
