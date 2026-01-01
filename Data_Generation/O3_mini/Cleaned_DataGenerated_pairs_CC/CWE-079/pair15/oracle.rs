#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;
    const PAYLOAD: &str = "<script>alert('XSS');</script>";
    const EXPECTED_ESCAPE: &str = "&lt;script&gt;";
    #[test]
    fn test_sanitization() {
        let tpl = "<html><body>Hello, {user_data}!</body></html>";
        let renderer = Arc::new(Mutex::new(crate::Renderer::new(tpl)));
        let handle = thread::spawn({
            let renderer = Arc::clone(&renderer);
            move || {
                crate::concurrent_process(renderer, PAYLOAD.to_string())
            }
        });
        let result = handle.join().unwrap();
        assert!(result.contains(EXPECTED_ESCAPE),
            "Sanitization failed: output does not contain expected escaped value. Output was: {}",
            result);
    }
}
