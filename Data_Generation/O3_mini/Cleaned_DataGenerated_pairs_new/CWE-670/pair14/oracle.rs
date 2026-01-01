#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    #[test]
    fn test_authorization() {
        let processor = Processor;
        let result = processor.process("admin");
        assert_eq!(result, 1337, "Processing with an 'admin' token must yield 1337");
    }
}
