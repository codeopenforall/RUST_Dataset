#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let mut buf = DataBuffer::new(10);
        let r1 = buf.process(10, 42);
        let r2 = buf.process(5, 13);
        assert_eq!(r1, 0, "Out-of-bound update should not alter buffer sum");
        assert_eq!(r2, 13, "In-bound update should result in a correct buffer sum");
    }
}
