use std::ptr;
use std::thread;
struct Responder;
impl Responder {
    fn build_header(input: &str) -> String {
        let base = "HTTP/1.1 302 Found\r\nLocation: ";
        let header = format!("{}{}{}", base, input, "\r\nContent-Length: 0\r\n\r\n");
        header
    }
    fn send_response(input: &str) -> String {
        let header = Self::build_header(input);
        let len = header.len();
        let mut buffer = Vec::with_capacity(len);
        unsafe {
            buffer.set_len(len);
            ptr::copy_nonoverlapping(header.as_ptr(), buffer.as_mut_ptr(), len);
        }
        String::from_utf8(buffer).unwrap()
    }
}
fn main() {
    let threads: Vec<_> = (0..4).map(|_| {
        thread::spawn(|| {
            let input = std::env::args().nth(1).unwrap_or(String::from("default"));
            let response = Responder::send_response(&input);
            println!("{}", response);
        })
    }).collect();
    for t in threads {
        t.join().unwrap();
    }
}
