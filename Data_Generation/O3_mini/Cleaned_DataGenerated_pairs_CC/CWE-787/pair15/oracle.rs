#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_buffer_update() {
        let mut buffer = DataBuffer::new(5);
        buffer.update(2, 100);
        buffer.update(7, 200);
        assert_eq!(buffer.data[7], 200, "The update did not correctly set the value at index 7");
    }
}
