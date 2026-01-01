#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_buffer_bounds() {
        let mut container = Container::new();
        let original = container.sentinel;
        unsafe {
            let _ = container.buf.update(20, &[1; 16]);
        }
        assert_eq!(
            container.sentinel, original,
            "Sentinel value corrupted due to buffer overflow"
        );
    }
}
