#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_guard_integrity() {
        let mut buf = Buffer::new(10);
        let _ = buf.update_range(5, 5, 42);
        assert_eq!(buf.guard(), 0, "Guard byte should remain unchanged after update_range");
    }
}
