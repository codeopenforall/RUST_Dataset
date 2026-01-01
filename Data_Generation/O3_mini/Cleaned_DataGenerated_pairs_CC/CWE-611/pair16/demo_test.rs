#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    // The test oracle verifies the behavior against malicious XML input.
    // For the vulnerable implementation, the external entity resolution will attempt to read a file,
    // which (in a test environment without the file) will yield a "File read error" error.
    // For the corrected implementation, the parser should refuse the external entity, returning
    // Err("External entity resolution disabled").
    //
    // This test asserts the error message expected from the corrected behavior.
    #[test]
    fn test_parser_against_external_entity() {
        let xml_input = "<!DOCTYPE test [ <!ENTITY file SYSTEM \"test_input.txt\"> ]><test>&file;</test>";
        let parser = Parser;
        let shared = Arc::new(parser);
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            shared_clone.process(xml_input)
        });
        let result = handle.join().expect("Thread panicked");
        // Expect the fixed (correct) behavior.
        assert_eq!(result, Err("External entity resolution disabled".to_string()));
    }
}