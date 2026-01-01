#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;
    #[test]
    fn oracle() {
        let buf = Buffer::new(10);
        let result = panic::catch_unwind(|| {
            buf.update(15, 99);
        });
        assert!(result.is_err(), "Update did not panic on out-of-bounds access");
    }
}
