/////////////////////// Vulnerable Implementation ///////////////////////
use std::ptr;
use std::thread;

struct Responder;

impl Responder {
    // Builds an HTTP response header using untrusted input.
    // This does not sanitize the input and is vulnerable to CRLF injection.
    fn build_header(input: &str) -> String {
        let base = "HTTP/1.1 302 Found\r\nLocation: ";
        // POTENTIAL FLAW: Directly incorporating untrusted data into the header.
        let header = format!("{}{}{}", base, input, "\r\nContent-Length: 0\r\n\r\n");
        header
    }

    // Simulates sending a response by copying the header into a buffer using unsafe code.
    fn send_response(input: &str) -> String {
        let header = Self::build_header(input);
        let len = header.len();
        let mut buffer = Vec::with_capacity(len);
        unsafe {
            // Unsafe block used for raw pointer copying.
            buffer.set_len(len);
            ptr::copy_nonoverlapping(header.as_ptr(), buffer.as_mut_ptr(), len);
        }
        String::from_utf8(buffer).unwrap()
    }
}

fn main() {
    // Spawning multiple threads to mimic concurrent header creation.
    let threads: Vec<_> = (0..4).map(|_| {
        thread::spawn(|| {
            // Untrusted input is taken from command-line arguments.
            let input = std::env::args().nth(1).unwrap_or(String::from("default"));
            let response = Responder::send_response(&input);
            println!("{}", response);
        })
    }).collect();
    for t in threads {
        t.join().unwrap();
    }
}