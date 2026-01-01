#[cfg(test)]
mod tests {
    // Bring in necessary items from the surrounding scope.
    use super::*;

    #[test]
    fn test_buffer_length() {
        // Create an instance with fixed capacity.
        let mut instance = CoreData::new(10);
        // Invoke the update method.
        instance.update();
        // On a correct implementation the vector length remains 10.
        // The vulnerable version incorrectly sets it to 11.
        assert_eq!(instance.vec.len(), 10, "The vector length is incorrect, indicating an out-of-bound write.");
    }
}