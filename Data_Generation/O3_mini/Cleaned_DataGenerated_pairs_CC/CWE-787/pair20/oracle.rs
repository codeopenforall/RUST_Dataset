#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_buffer_length() {
        let mut instance = CoreData::new(10);
        instance.update();
        assert_eq!(instance.vec.len(), 10, "The vector length is incorrect, indicating an out-of-bound write.");
    }
}
