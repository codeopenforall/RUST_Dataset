#[cfg(test)]
mod tests {
    use super::Buffer;
    use std::sync::Arc;
    use std::thread;
    #[test]
    fn test_sentinel_invariant() {
        let buffer = Arc::new(Buffer::new(10));
        assert_eq!(buffer.get(10), 0, "Sentinel should be initially 0");
        let buffer_clone = Arc::clone(&buffer);
        let handle = thread::spawn(move || {
            unsafe { buffer_clone.fill(0xAA); }
        });
        unsafe { buffer.fill(0xAA); }
        handle.join().unwrap();
        assert_eq!(buffer.get(10), 0, "Sentinel was corrupted by out-of-bound write");
    }
}
