use std::fs::{File, symlink_metadata};
use std::io::{Read, Seek, SeekFrom, Write};
use std::env;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
struct Settings {
    content: Vec<u8>,
}
impl Settings {
    unsafe fn build(input: Vec<u8>) -> Self {
        let base = input.as_ptr() as *const u8;
        let _first = *base; 
        Self { content: input }
    }
}
fn is_valid_path(target: &Path, allowed: &Path) -> bool {
    if let Ok(canonical) = target.canonicalize() {
        canonical.starts_with(allowed)
    } else {
        false
    }
}
fn process_input(path: &str, allowed: &Path) -> Result<Settings, String> {
    let file_path = Path::new(path);
    let metadata = symlink_metadata(file_path).map_err(|e| e.to_string())?;
    if metadata.file_type().is_symlink() || !is_valid_path(file_path, allowed) {
         return Err("Insecure file path detected".to_string());
    }
    let mut file = File::open(file_path).map_err(|e| e.to_string())?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
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
    let allowed_directory = Path::new("/safe_dir");
    match process_input(&arguments[1], allowed_directory) {
         Ok(settings) => println!("Settings loaded, {} bytes", settings.content.len()),
         Err(err) => println!("Error: {}", err)
    }
}
