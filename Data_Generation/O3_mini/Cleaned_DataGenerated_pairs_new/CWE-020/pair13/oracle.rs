#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    #[test]
    fn test_oracle() {
        let input_data = "12345".to_string();
        let manager = BufferManager { data: input_data.into_bytes() };
        let proc_inst = Processor { manager: Arc::new(Mutex::new(manager)) };
        let result = proc_inst.run(3, 5);
        assert!(result.is_err(), "Expected an error due to out-of-bound extraction");
        assert_eq!(result.err().unwrap(), "out of bounds");
    }
}
