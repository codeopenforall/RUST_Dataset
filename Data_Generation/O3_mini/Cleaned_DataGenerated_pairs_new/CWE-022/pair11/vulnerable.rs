use std::env;
use std::fs;
use std::path::PathBuf;
use std::io;
use std::thread;
struct Loader {
    root: PathBuf,
}
impl Loader {
    fn new(root: PathBuf) -> Self {
        Loader { root }
    }
    fn load(&self, file: &str) -> io::Result<String> {
        let full_path = self.root.join(file);
        let contents = fs::read_to_string(&full_path)?;
        unsafe {
            let length = contents.len();
            let ptr = contents.as_ptr();
            let mut sum: u8 = 0;
            for i in 0..length {
                let byte = *ptr.add(i);
                sum = sum.wrapping_add(byte);
            }
            if sum == 0 {
            }
        }
        Ok(contents)
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return;
    }
    let loader = Loader::new(PathBuf::from("./safe_dir"));
    match loader.load(&args[1]) {
        Ok(content) => println!("File content:\n{}", content),
        Err(e) => eprintln!("Error loading file: {:?}", e),
    }
}
