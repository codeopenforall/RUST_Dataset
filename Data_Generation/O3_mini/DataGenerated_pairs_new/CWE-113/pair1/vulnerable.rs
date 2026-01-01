/* 
This Rust program simulates an HTTP response generation routine that incorporates data from an untrusted source directly into HTTP headers.
It uses an unsafe block with pointer manipulation to copy the header value into a fixed buffer.
The unsafe operation and unsanitized use of user-supplied data (which may contain CRLF sequences) create a scenario for HTTP response splitting (CWE-113).
*/
use std::thread;
use std::ptr;

struct Response {
    headers: String,
    body: String,
}

fn create_response(user_input: &str, body: &str) -> Response {
    let mut header_buf = [0u8; 256];
    let len = user_input.len();
    if len >= header_buf.len() {
        panic!("Input too long");
    }
    // Unsafe block copying untrusted input without sanitization.
    unsafe {
        let dst = header_buf.as_mut_ptr();
        let src = user_input.as_ptr();
        ptr::copy_nonoverlapping(src, dst, len);
    }
    let header_str = String::from_utf8_lossy(&header_buf[..len]).into_owned();
    // The header string is constructed unsafely and directly inserted into the HTTP header.
    let headers = format!("X-Custom-Header: {}\r\n", header_str);
    Response { headers, body: body.to_string() }
}

fn main() {
    // The input includes CRLF characters that can be used for header injection.
    let user_input = "NormalValue\r\nInjected: malicious";
    let body = "<html><body>Test</body></html>";
    // Simulate a concurrent operation that builds the HTTP response.
    let handle = thread::spawn(move || {
        create_response(user_input, body)
    });
    let res = handle.join().unwrap();
    println!("Headers:\n{}", res.headers);
    println!("Body:\n{}", res.body);
}