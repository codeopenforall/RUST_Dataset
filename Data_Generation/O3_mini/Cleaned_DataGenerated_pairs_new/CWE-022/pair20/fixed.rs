use std::env;
use std::fs::{File, canonicalize};
use std::io::Read;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
struct ResourceHandler {
    base: PathBuf,
    allowed: PathBuf,
}
impl ResourceHandler {
    fn new(mut base: PathBuf) -> Self {
        let allowed = canonicalize(&base).expect("Failed to canonicalize base path");
        Self { base, allowed }
    }
    fn load(&self, relative: &str) -> Option<String> {
        let target = self.base.join(relative);
        let target_canon = canonicalize(&target).ok()?;
        if !target_canon.starts_with(&self.allowed) {
            return None;
        }
        let mut file = File::open(&target_canon).ok()?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).ok()?;
        String::from_utf8(buf).ok()
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        args[1].clone()
    } else {
        String::from("default.txt")
    };
    let handler = Arc::new(Mutex::new(ResourceHandler::new(PathBuf::from("./safe_dir"))));
    let handler_clone = Arc::clone(&handler);
    let input_clone = input.clone();
    let handle = thread::spawn(move || {
        let locked = handler_clone.lock().unwrap();
        locked.load(&input_clone)
    });
    let result = handle.join().unwrap();
    match result {
        Some(text) => println!("File content: {}", text),
        None => println!("Failed to load file or invalid access."),
    }
}
