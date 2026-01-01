#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;
    fn simulate_render<R: Fn(&str) -> String>(render_fn: R) -> String {
        let malicious_input = "<script>alert('xss');</script>";
        let template = "<html><body>Welcome, {input}!</body></html>".to_string();
        render_fn(malicious_input).replace("{input}", "")
    }
    #[test]
    fn test_html_escaping() {
        let template = "<html><body>Welcome, {input}!</body></html>".to_string();
        let renderer = super::Renderer { template };
        let malicious = "<script>alert('xss');</script>";
        let output = renderer.render(malicious);
        assert!(
            !output.contains("<script"),
            "Output should not contain raw <script> tags; found potential XSS vulnerability."
        );
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
