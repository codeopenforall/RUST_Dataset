#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    #[test]
    fn oracle() {
        let data = Arc::new(Mutex::new(vec![0u8; 10]));
        let processor = DataProcessor { data: data.clone() };
        let res_valid = processor.process(&[1, 2, 3]);
        assert!(res_valid.is_ok(), "Valid input should succeed.");
        let res_invalid = processor.process(&[1,2,3,4,5,6,7,8,9,10,11]);
        assert!(res_invalid.is_err(), "Invalid input should produce an error.");
    }
}
