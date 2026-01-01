////////////////////////////////////////////////////////////////////////////////////////////////////
// Note: This is an intentionally vulnerable implementation that reflects unsanitized user
// input into an HTML response (CWE-079). It also uses unsafe pointer copying into a fixed-size
// buffer and concurrent thread spawning to mimic complex real-world code. The unsafe block does
// not perform proper bounds checking. The unsanitized input creates a potential XSS issue.
////////////////////////////////////////////////////////////////////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;

struct Renderer {
    // A shared buffer used in an unsafe operation. The fixed size buffer is intentionally too small.
    buffer: Arc<Mutex<Vec<u8>>>,
}

impl Renderer {
    fn new() -> Self {
        // Fixed to 10 bytes, this buffer is not resized later.
        Self {
            buffer: Arc::new(Mutex::new(vec![0u8; 10])),
        }
    }

    // Returns an HTML string that unsafely embeds the provided input.
    fn render(&self, input: &str) -> String {
        // Unsafe memory copy: Copies the input bytes into a fixed buffer with no length check.
        // This operation is undefined for inputs longer than the buffer size.
        {
            let mut guard = self.buffer.lock().unwrap();
            unsafe {
                let dest = guard.as_mut_ptr();
                let src = input.as_bytes().as_ptr();
                // POTENTIALLY UNSAFE: Using input.len() without verifying against guard.len()
                std::ptr::copy_nonoverlapping(src, dest, input.len());
            }
        }
        // Construct HTML with unsanitized reflection of user input.
        let mut html = String::new();
        html.push_str("<html><body>");
        html.push_str("User input: ");
        // VULNERABILITY: Input is inserted directly into HTML without escaping.
        html.push_str(input);
        html.push_str("</body></html>");
        html
    }
}

fn main() {
    let renderer = Arc::new(Renderer::new());
    // Obtain user input from command-line arguments.
    let input = std::env::args().nth(1).unwrap_or_else(|| String::new());
    let r = renderer.clone();
    let handle = thread::spawn(move || {
        let output = r.render(&input);
        println!("{}", output);
    });
    handle.join().unwrap();
}