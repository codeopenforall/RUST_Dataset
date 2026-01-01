//////////////////// Fixed Code ////////////////////
use std::env;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

fn process(input: &str) {
    // Allowed directory for file access.
    let allowed_dir = Path::new("/safe/directory");

    // Convert input into a C-style string and then unsafely copy its bytes.
    let cstr = std::ffi::CString::new(input).unwrap();
    let ptr = cstr.as_ptr() as *const u8;
    let raw_path: Vec<u8> = unsafe {
        std::slice::from_raw_parts(ptr, input.len()).to_vec()
    };
    let file_path = String::from_utf8(raw_path).unwrap();
    let given_path = Path::new(&file_path);

    // Canonicalize the path to resolve all symbolic links.
    let resolved_path = fs::canonicalize(given_path).expect("Failed to resolve file path");

    // Enforce that the resolved file is within the allowed directory.
    if !resolved_path.starts_with(allowed_dir) {
        panic!("Access denied: File is outside the allowed directory");
    }

    if resolved_path.is_file() {
        let mut file = File::open(&resolved_path).expect("Cannot open file!");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Could not read file");
        println!("File contents: {}", contents);
    } else {
        println!("Not a regular file: {:?}", resolved_path);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file_path>", args[0]);
        return;
    }
    let file_arg = args[1].clone();
    let lock = Arc::new(Mutex::new(()));
    let lock_clone = Arc::clone(&lock);
    let handler = thread::spawn(move || {
        let _guard = lock_clone.lock().unwrap();
        process(&file_arg);
    });
    let _ = handler.join();
}