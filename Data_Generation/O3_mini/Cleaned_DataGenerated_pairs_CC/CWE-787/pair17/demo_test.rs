#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_fill() {
        let mut buf = Buffer::new(10);
        buf.fill_data(42);
        // Expect the vector length to be exactly 10.
        // In the vulnerable version, the length becomes 11 due to the extra write,
        // causing this test to fail.
        assert_eq!(buf.data.len(), 10, "Buffer length is incorrect; vulnerability detected.");

        for &val in &buf.data {
            assert_eq!(val, 42, "Buffer element was not filled correctly.");
        }
    }
}