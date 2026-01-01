#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;
    #[test]
    fn contract_test() {
        let buffer = Buffer::new(vec![10, 20, 30, 40]);
        let result = panic::catch_unwind(|| {
            buffer.fetch(0);
        });
        assert!(
            result.is_err(), 
            "Expected panic on invalid index, but the call did not panic."
        );
    }
}
