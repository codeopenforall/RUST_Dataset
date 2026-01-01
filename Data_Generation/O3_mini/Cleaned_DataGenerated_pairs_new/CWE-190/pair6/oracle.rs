#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_overflow_detection() {
        let count = usize::MAX / 2 + 1;
        let elem_size = 2;
        let res = Buffer::new(count, elem_size);
        match res {
            Ok(buf) => {
                assert!(buf.size > 0, "Detected vulnerability: buffer size incorrect due to arithmetic overflow");
            },
            Err(e) => {
                assert_eq!(e, "Overflow detected", "Properly detected arithmetic overflow");
            }
        }
    }
}
