#[cfg(test)]
mod tests {
    // Import the common interface from the module under test.
    use super::*;

    #[test]
    fn test_out_of_bounds_update() {
        let mut handler = Handler::new(10);
        // For the fixed implementation, the update should return an error when writing out-of-bounds.
        // For the vulnerable version, update returns Ok despite writing out-of-bounds.
        let result = handler.update(10, 255);
        assert!(
            result.is_err(),
            "The update method should fail on an out-of-bound index, but it did not."
        );
    }
}