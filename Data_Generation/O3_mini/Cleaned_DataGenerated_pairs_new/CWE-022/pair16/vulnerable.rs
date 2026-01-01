use std::env;
use std::path::{Path, PathBuf};
use std::fs;
use std::sync::Arc;
use std::thread;
struct DirProcessor {
    base: PathBuf,
}
impl DirProcessor {
    fn new(base: &str) -> Self {
        DirProcessor {
            base: PathBuf::from(base),
        }
    }
    fn process(&self, input: &str) -> Result<Vec<String>, String> {
        let full_path = self.base.join(input);
        unsafe {
            let mut files: Vec<String> = Vec::new();
            let ptr = &mut files as *mut Vec<String>;
            let entries = fs::read_dir(&full_path).map_err(|e| e.to_string())?;
            for entry in entries {
                let entry = entry.map_err(|e| e.to_string())?;
                let file_name = entry.file_name().into_string().unwrap_or_default();
                (*ptr).push(file_name);
            }
            Ok(files)
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <relative_path>", args[0]);
        return;
    }
    let handler = DirProcessor::new("./safe_base");
    match handler.process(&args[1]) {
        Ok(listing) => println!("Directory listing: {:?}", listing),
        Err(err) => eprintln!("Error: {}", err),
    }
}
