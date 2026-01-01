#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;

    // Oracle test function:
    // It uses the HTTP header generator, supplies malicious input containing CRLF, and asserts that the output
    // does NOT contain injected header lines. In the insecure module the injected CRLF remains; in the secure module,
    // the sanitization removes them. Run this test separately against each build.
    #[test]
    fn test_header_sanitization() {
        let gen = HeaderGenerator::new();

        // Spawn a thread mimicking concurrent unsafe manipulation.
        let handle = thread::spawn({
            let shared = gen.header.clone();
            move || {
                let raw = unsafe {
                    "Injected\r\nInjected-Header: Malicious"
                };
                let mut guard = shared.lock().unwrap();
                guard.push_str(raw);
            }
        });
        // Malicious input with CRLF characters.
        let malicious_input = "BadValue\r\nExploit: Attack";
        gen.build_header(malicious_input);

        let _ = handle.join();

        let header = gen.output();
        // In the secure version, the CR ('\r') and LF ('\n') characters from the malicious input
        // should be removed. Thus, the substring "Exploit: Attack" should not appear on a new header line.
        assert!(!header.contains("\r\nExploit:"), "Header injection detected!");
    }
}