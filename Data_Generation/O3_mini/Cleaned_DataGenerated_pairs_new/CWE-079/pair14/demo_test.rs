///////////////////////////////////////////////////////////
// Test oracle for verifying HTML escaping of untrusted input.
// This test should fail when run against the insecure implementation,
// since the output contains raw HTML tags, and should pass against
// the secure version where the untrusted input is properly escaped.
///////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;

    // Assume the shared Renderer struct and its render() method are available.

    // Helper to simulate a rendering process.
    fn simulate_render<R: Fn(&str) -> String>(render_fn: R) -> String {
        let malicious_input = "<script>alert('xss');</script>";
        // Use a simple HTML template with a marker.
        let template = "<html><body>Welcome, {input}!</body></html>".to_string();
        render_fn(malicious_input).replace("{input}", "")
    }

    #[test]
    fn test_html_escaping() {
        // Prepare a Renderer with the template.
        let template = "<html><body>Welcome, {input}!</body></html>".to_string();
        let renderer = super::Renderer { template };
        let malicious = "<script>alert('xss');</script>";

        // Synchronously test the render function.
        let output = renderer.render(malicious);
        // In a secure implementation, the result should not include the literal "<script"
        // but the escaped version "&lt;script".
        assert!(
            !output.contains("<script"),
            "Output should not contain raw <script> tags; found potential XSS vulnerability."
        );
        // Conversely, the output should contain the escaped form.
        assert!(
            output.contains("&lt;script"),
            "Output should contain escaped script tag to mitigate XSS."
        );
    }

    #[test]
    fn test_concurrent_rendering() {
        let template = "<html><body>Welcome, {input}!</body></html>".to_string();
        let renderer = super::Renderer { template };
        let shared = Arc::new(Mutex::new(renderer));

        let handles: Vec<_> = (0..4)
            .map(|_| {
                let lock = Arc::clone(&shared);
                thread::spawn(move || {
                    let user_input = "<script>alert('xss');</script>";
                    let guard = lock.lock().unwrap();
                    guard.render(user_input)
                })
            })
            .collect();

        for handle in handles {
            let out = handle.join().unwrap();
            assert!(
                !out.contains("<script"),
                "Concurrent rendering should not produce raw <script> tags."
            );
            assert!(
                out.contains("&lt;script"),
                "Output must include escaped representation of script tags."
            );
        }
    }
}