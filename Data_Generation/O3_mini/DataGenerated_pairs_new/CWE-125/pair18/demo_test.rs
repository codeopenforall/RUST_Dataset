//////////////////////////////
// Test Oracle Function     //
// (Separate from the code) //
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    // This test invokes the extraction method using an index that is just out-of-range.
    // In the correctly fixed version, the call should return None.
    #[test]
    fn test_extraction_bounds() {
        let data = vec![10, 20, 30]; // Buffer with 3 elements.
        let processor = Processor::new(data);
        let index = 3; // Index is equal to the length => out-of-bounds.
        let proc_arc = Arc::new(processor);
        let proc_clone = Arc::clone(&proc_arc);
        
        let handle = thread::spawn(move || {
            // Use the extraction method.
            proc_clone.extract(index)
        });
        let result = handle.join().expect("Thread panicked");

        // The expected outcome for a safe extraction is None.
        // In the vulnerable version, this assertion will fail (as it returns Some(_)).
        assert_eq!(None, result, "Extraction did not properly check bounds");
    }
}