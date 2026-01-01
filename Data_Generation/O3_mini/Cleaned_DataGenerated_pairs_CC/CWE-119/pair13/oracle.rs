#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_memory_bounds() {
        let mut buf = Buffer::new(5);
        assert!(buf.update(4, 42).is_ok());
        assert!(buf.update(5, 99).is_err());
    }
}
