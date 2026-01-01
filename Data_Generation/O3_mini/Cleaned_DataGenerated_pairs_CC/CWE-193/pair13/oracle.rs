#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_buffer_sum() {
        let expected: u32 = 55;
        let mut buf = Buffer::new(10);
        buf.populate();
        let total = buf.concurrent_sum();
        assert_eq!(total, expected, "The computed sum does not match the expected value.");
    }
}
