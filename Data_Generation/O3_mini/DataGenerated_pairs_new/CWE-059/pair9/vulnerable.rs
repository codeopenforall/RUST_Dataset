//////////////////////////////////////////////////////////////
// Vulnerable implementation demonstrating CWE-059
// This program simulates a flaw in symbolic link resolution
// where the provided file path is resolved without proper
// validation. An unsafe block converts a Rust string into a
// C string and then reconstructs it without checking for any
// malicious content (e.g. paths containing ".."). Furthermore,
// the resolved path is used directly to simulate file access,
// allowing attackers to bypass directory restrictions.
//////////////////////////////////////////////////////////////

use std::ffi::CString;
use std::str;
use std::thread;

pub fn resolve_path(input: String) -> String {
    // Spawn a thread that uses an unsafe block to simulate
    // low-level processing of the input.
    let handle = thread::spawn(move || {
        unsafe {
            let c_str = CString::new(input).unwrap();
            let ptr = c_str.as_ptr();
            // Vulnerability: The code directly uses the length of the
            // original byte array without validating or sanitizing the
            // underlying data.
            let len = c_str.to_bytes().len();
            let slice = std::slice::from_raw_parts(ptr as *const u8, len);
            str::from_utf8(slice).unwrap().to_string()
        }
    });
    handle.join().unwrap()
}

pub fn process_file(path: &str) -> Result<String, String> {
    // Vulnerability: There is no check to ensure that the resolved
    // path does not contain disallowed components (e.g., "..")
    let resolved = resolve_path(path.to_string());
    // Simulated file access: the program grants access regardless
    // of whether the path is safe.
    Ok(format!("Access granted to file: {}", resolved))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = if args.len() > 1 { &args[1] } else { "./safe/default.txt" };
    match process_file(input) {
        Ok(msg) => println!("{}", msg),
        Err(e) => eprintln!("Error: {}", e),
    }
}