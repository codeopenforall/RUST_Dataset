#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_buffer_fill() {
        let mut buf = Buffer::new(10);
        buf.fill_data(42);
        assert_eq!(buf.data.len(), 10, "Buffer length is incorrect; vulnerability detected.");
        for &val in &buf.data {
            assert_eq!(val, 42, "Buffer element was not filled correctly.");
        }
    }
}
