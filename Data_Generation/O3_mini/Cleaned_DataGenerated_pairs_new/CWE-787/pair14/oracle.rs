#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_out_of_bounds_update() {
        let mut handler = Handler::new(10);
        let result = handler.update(10, 255);
        assert!(
            result.is_err(),
            "The update method should fail on an out-of-bound index, but it did not."
        );
    }
}
