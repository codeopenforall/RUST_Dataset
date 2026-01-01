/* 
This program simulates a simple web template renderer where the user‐supplied input is directly embedded into 
an HTML response. An unsafe block is used to manually construct the HTTP response using raw pointer operations.
This code is vulnerable to Cross-Site Scripting (CWE-079) because it directly copies untrusted input into an HTML template 
without performing any escaping or sanitization.
*/
use std::sync::Arc;
use std::thread;

fn unsafe_copy(src: *const u8, dest: *mut u8, count: usize) {
    unsafe {
        std::ptr::copy_nonoverlapping(src, dest, count);
    }
}

struct App {
    // In a real application, configuration and state could be stored here.
}

impl App {
    fn new() -> Self {
        App {}
    }

    fn render(&self, input: &str) -> String {
        let header = "<html><body>Hello, ";
        let footer = "</body></html>";
        // Calculate total length without any escaping
        let total_len = header.len() + input.len() + footer.len();
        let mut buffer: Vec<u8> = Vec::with_capacity(total_len);
        unsafe {
            // Unsafely set the length; this is inherently risky.
            buffer.set_len(total_len);
            // Copy header into buffer.
            std::ptr::copy_nonoverlapping(header.as_ptr(), buffer.as_mut_ptr(), header.len());
            // SECURITY FLAW: Directly copy the untrusted input without escaping HTML special characters.
            std::ptr::copy_nonoverlapping(
                input.as_ptr(),
                buffer.as_mut_ptr().add(header.len()),
                input.len(),
            );
            // Copy footer into buffer.
            std::ptr::copy_nonoverlapping(
                footer.as_ptr(),
                buffer.as_mut_ptr().add(header.len() + input.len()),
                footer.len(),
            );
        }
        // Convert the byte buffer to a UTF-8 string.
        String::from_utf8(buffer).unwrap()
    }

    fn handle(&self, payload: String) -> String {
        self.render(&payload)
    }
}

fn main() {
    let app = Arc::new(App::new());
    let app_clone = Arc::clone(&app);
    let handle = thread::spawn(move || {
        // Simulated HTTP request payload containing a potential XSS attack.
        let attack_payload = "<script>alert('XSS');</script>";
        app_clone.handle(attack_payload.to_string())
    });
    // Join thread and output the rendered HTML.
    let output = handle.join().unwrap();
    println!("{}", output);
}