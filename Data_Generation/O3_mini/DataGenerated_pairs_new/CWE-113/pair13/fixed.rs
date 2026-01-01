/////////////////////// Corrected Implementation ///////////////////////
use std::ptr;
use std::thread;

struct Responder;

impl Responder {
    // Sanitizes untrusted input by removing CR and LF characters.
    fn sanitize(input: &str) -> String {
        input.chars().filter(|&c| c != '\r' && c != '\n').collect()
    }

    // Builds an HTTP response header using sanitized input.
    fn build_header(input: &str) -> String {
        let base = "HTTP/1.1 302 Found\r\nLocation: ";
        // FIX: Sanitize the user-controlled input to prevent header injection.
        let safe_input = Self::sanitize(input);
        let header = format!("{}{}{}", base, safe_input, "\r\nContent-Length: 0\r\n\r\n");
        header
    }

    // Simulates sending a response by safely copying the header into a buffer.
    fn send_response(input: &str) -> String {
        let header = Self::build_header(input);
        let len = header.len();
        let mut buffer = Vec::with_capacity(len);
        unsafe {
            // Same unsafe copy, now with sanitized input.
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