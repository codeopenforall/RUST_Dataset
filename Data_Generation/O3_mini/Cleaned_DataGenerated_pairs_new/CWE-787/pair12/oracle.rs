#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;
    #[test]
    fn test_out_of_bound_write() {
        let input = b"ABCDE";
        let result = panic::catch_unwind(|| {
            let mut buf = Buffer::new(10);
            unsafe {
                buf.write_data(8, input);
            }
            buf.data.len()
        });
        match result {
            Ok(len) => {
                assert!(
                    len <= 10,
                    "Test failure: Buffer write extended the length to {} (expected <= 10)", len
                );
            }
            Err(_) => {
            }
        }
    }
}
