use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
struct ResourceHandler {
    base: PathBuf,
}
impl ResourceHandler {
    fn new(base: PathBuf) -> Self {
        Self { base }
    }
    fn load(&self, relative: &str) -> Option<String> {
        let target = self.base.join(relative);
        let content = unsafe {
            let mut file = File::open(&target).ok()?;
            let mut buf = Vec::new();
            file.read_to_end(&mut buf).ok()?;
            String::from_utf8_unchecked(buf)
        };
        Some(content)
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
        None => println!("Failed to load file."),
    }
}
