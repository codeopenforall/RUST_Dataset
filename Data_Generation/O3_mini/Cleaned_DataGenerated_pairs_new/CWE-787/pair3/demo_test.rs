#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer() {
        let mut buf = Buffer::new(6);
        let sum = buf.compute();
        // In the proper implementation, every element in the 6-length buffer is set to 1,
        // so the expected sum is 6.
        assert_eq!(sum, 6, "Buffer sum value is not as expected");
    }
}