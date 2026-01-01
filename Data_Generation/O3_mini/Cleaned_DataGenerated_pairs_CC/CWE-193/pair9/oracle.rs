#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_buffer_content() {
        let mut buf = DataBuffer::new(5);
        buf.fill(10);
        for i in 0..5 {
            assert_eq!(buf.data[i], 10, "Element at index {} did not match expected value", i);
        }
        assert_eq!(buf.data.len(), 5, "Buffer length is not correct");
    }
}
