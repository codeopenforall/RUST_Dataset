#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;
    #[test]
    fn test_extraction_bounds() {
        let data = vec![10, 20, 30]; 
        let processor = Processor::new(data);
        let index = 3; 
        let proc_arc = Arc::new(processor);
        let proc_clone = Arc::clone(&proc_arc);
        let handle = thread::spawn(move || {
            proc_clone.extract(index)
        });
        let result = handle.join().expect("Thread panicked");
        assert_eq!(None, result, "Extraction did not properly check bounds");
    }
}
