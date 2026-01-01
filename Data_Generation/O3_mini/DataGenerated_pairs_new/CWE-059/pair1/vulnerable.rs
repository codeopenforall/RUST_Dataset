use std::fs::File;
use std::io::{Read, Seek, Write};
use std::env;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

struct Settings {
    content: Vec<u8>,
}

impl Settings {
    unsafe fn build(input: Vec<u8>) -> Self {
        // Unsafe usage: reading via a raw pointer bypassing safety checks.
        let base = input.as_ptr() as *const u8;
        let _first = *base; // Read first byte unsafely.
        Self { content: input }
    }
}

fn process_input(path: &str, _allowed: &Path) -> Result<Settings, String> {
    // Vulnerability: Fails to validate whether the supplied file is a symlink or escapes the allowed directory.
    let file_path = Path::new(path);
    let mut file = File::open(file_path).map_err(|e| e.to_string())?;
    
    // Simulate a time-of-check to time-of-use race condition through concurrency.
    let file_container = Arc::new(Mutex::new(file));
    let file_container_clone = Arc::clone(&file_container);
    let handle = thread::spawn(move || {
         let mut inner_file = file_container_clone.lock().unwrap();
         // Simulated race: reset file pointer unsafely.
         let _ = inner_file.seek(std::io::SeekFrom::Start(0));
    });
    handle.join().unwrap();

    let mut buffer = Vec::new();
    {
        let mut locked = file_container.lock().unwrap();
        locked.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
    }
    unsafe {
         Ok(Settings::build(buffer))
    }
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() < 2 {
         println!("Usage: {} <file_path>", arguments[0]);
         return;
    }
    // In this version, no safe directory check is performed.
    let allowed_directory = Path::new("/safe_dir");
    match process_input(&arguments[1], allowed_directory) {
         Ok(settings) => println!("Settings loaded, {} bytes", settings.content.len()),
         Err(err) => println!("Error: {}", err)
    }
}