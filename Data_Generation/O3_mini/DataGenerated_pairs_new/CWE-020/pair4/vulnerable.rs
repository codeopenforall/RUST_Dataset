//////////////////////////////
// Vulnerability Example
//////////////////////////////
use std::env;

static DATA: &[u8] = b"Example static data used for unsafe string extraction vulnerability demo";

struct Processor;

impl Processor {
    // This method unsafely extracts a substring from DATA based on user-supplied offset and length.
    // No validation is done; if offset+length exceeds DATA's bounds, undefined behavior may occur.
    fn process(&self, offset: usize, len: usize) -> Result<String, String> {
        unsafe {
            // Vulnerable lines: no boundary check is performed.
            let ptr = DATA.as_ptr().add(offset);
            let slice = std::slice::from_raw_parts(ptr, len);
            let s = std::str::from_utf8_unchecked(slice);
            Ok(s.to_string())
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} offset:len", args[0]);
        return;
    }
    let parts: Vec<&str> = args[1].split(':').collect();
    if parts.len() != 2 {
        println!("Invalid input format, expected offset:len");
        return;
    }
    let offset = parts[0].parse::<usize>().unwrap_or(0);
    let len = parts[1].parse::<usize>().unwrap_or(0);

    let proc_inst = Processor;
    match proc_inst.process(offset, len) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}