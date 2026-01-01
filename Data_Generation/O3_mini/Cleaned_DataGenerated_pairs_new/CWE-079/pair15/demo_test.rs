///////////////////////////////////////////////////////////////
// Test Oracle: This unit test verifies XSS sanitization.
// It fails against the unsanitized (vulnerable) version and passes against the fixed version.
///////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;
    
    // Assume the implementation under test exposes Renderer and concurrent_process.
    // This oracle test should be compiled against the target implementation.
    // The input payload contains malicious code, and the expected output for the fixed version
    // must have its dangerous characters escaped.
    
    // Test payload and expected substring after sanitization.
    const PAYLOAD: &str = "<script>alert('XSS');</script>";
    const EXPECTED_ESCAPE: &str = "&lt;script&gt;";
    
    #[test]
    fn test_sanitization() {
        // Set up the template and shared renderer.
        let tpl = "<html><body>Hello, {user_data}!</body></html>";
        // The Renderer::new and concurrent_process functions are assumed to be imported from the implementation under test.
        let renderer = Arc::new(Mutex::new(crate::Renderer::new(tpl)));
        
        // Spawn thread to simulate concurrent processing.
        let handle = thread::spawn({
            let renderer = Arc::clone(&renderer);
            move || {
                crate::concurrent_process(renderer, PAYLOAD.to_string())
            }
        });
        
        let result = handle.join().unwrap();
        
        // The fixed, safe version should escape the "<script>" tag.
        // This test will fail if the output still contains the unsanitized tag.
        assert!(result.contains(EXPECTED_ESCAPE),
            "Sanitization failed: output does not contain expected escaped value. Output was: {}",
            result);
    }
}