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
    unsafe {
        let dst = header_buf.as_mut_ptr();
        let src = user_input.as_ptr();
        ptr::copy_nonoverlapping(src, dst, len);
    }
    let header_str = String::from_utf8_lossy(&header_buf[..len]).into_owned();
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
