#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_overflow_handling() {
        let processor = BufferProcessor::new();
        let count = 0x40000000;
        let res = processor.process(count);
        assert_eq!(res, Err("Integer overflow in allocation size"));
    }
}
