#[cfg(test)]
mod tests {
    use super::Buffer;
    #[test]
    fn memory_bounds_oracle() {
        let input: Vec<u8> = vec![42, 10, 20, 30, 40];
        let mut buf = Buffer::new(input.len());
        buf.copy_from(&input).expect("Copy failed");
        assert_eq!(buf.first(), 42, "Expected first byte to be 42");
    }
}
