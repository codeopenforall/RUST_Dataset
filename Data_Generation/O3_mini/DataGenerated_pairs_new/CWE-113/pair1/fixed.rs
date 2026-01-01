/* 
This Rust program provides a corrected HTTP response generation mechanism.
It sanitizes untrusted input by removing any CR ('\r') and LF ('\n') characters before using it in HTTP headers.
In addition to preserving the original functionality, it prevents HTTP response splitting attacks (CWE-113).
*/
use std::thread;
use std::ptr;

struct Response {
    headers: String,
    body: String,
}

fn create_response(user_input: &str, body: &str) -> Response {
    // Sanitize input by filtering out CR and LF characters.
    let sanitized: String = user_input.chars()
        .filter(|&c| c != '\r' && c != '\n')
        .collect();

    let mut header_buf = [0u8; 256];
    let len = sanitized.len();
    if len >= header_buf.len() {
        panic!("Input too long");
    }
    // Unsafe block is still used to copy the sanitized input.
    unsafe {
        let dst = header_buf.as_mut_ptr();
        let src = sanitized.as_ptr();
        ptr::copy_nonoverlapping(src, dst, len);
    }
    let header_str = String::from_utf8_lossy(&header_buf[..len]).into_owned();
    // Build the header using the sanitized input.
    let headers = format!("X-Custom-Header: {}\r\n", header_str);
    Response { headers, body: body.to_string() }
}

fn main() {
    let user_input = "NormalValue\r\nInjected: malicious";
    let body = "<html><body>Test</body></html>";
    let handle = thread::spawn(move || {
        create_response(user_input, body)
    });
    let res = handle.join().unwrap();
    println!("Headers:\n{}", res.headers);
    println!("Body:\n{}", res.body);
}