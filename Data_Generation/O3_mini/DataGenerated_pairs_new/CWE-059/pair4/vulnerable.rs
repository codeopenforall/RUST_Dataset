//////////////////// Vulnerable Code ////////////////////
use std::env;
use std::fs::{self, File};
use std::io::Read;
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

fn process(input: &str) {
    // Convert input into a C-style string and then unsafely copy its bytes.
    let cstr = std::ffi::CString::new(input).unwrap();
    let ptr = cstr.as_ptr() as *const u8;
    let raw_path: Vec<u8> = unsafe {
        // UNSAFE: Reading raw pointer without boundary checks.
        std::slice::from_raw_parts(ptr, input.len()).to_vec()
    };
    let file_path = String::from_utf8(raw_path).unwrap();
    let target = Path::new(&file_path);

    // Check file metadata without canonicalization.
    if let Ok(metadata) = fs::symlink_metadata(target) {
        // POTENTIALLY UNSAFE: Directly following symbolic links without validation.
        if metadata.file_type().is_symlink() {  // Vulnerability starts here.
            if let Ok(real_path) = fs::read_link(target) {
                println!("Following symbolic link to: {:?}", real_path);
                unsafe {
                    // UNSAFE: Reading file content without proper sandboxing.
                    let mut file = File::open(&real_path).expect("Cannot open real file!");
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).expect("Could not read file");
                    println!("File contents: {}", contents);
                }
            }
        } else {
            println!("Regular file detected: {:?}", target);
        }
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