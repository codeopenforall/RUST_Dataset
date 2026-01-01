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
        let canonical_base = fs::canonicalize(base).unwrap_or_else(|_| PathBuf::from(base));
        DirProcessor {
            base: canonical_base,
        }
    }
    fn process(&self, input: &str) -> Result<Vec<String>, String> {
        let joined = self.base.join(input);
        let canonical_path = fs::canonicalize(&joined).map_err(|e| e.to_string())?;
        if !canonical_path.starts_with(&self.base) {
            return Err("Invalid path: Access outside base directory denied".to_string());
        }
        let mut files: Vec<String> = Vec::new();
        let entries = fs::read_dir(&canonical_path).map_err(|e| e.to_string())?;
        for entry in entries {
            let entry = entry.map_err(|e| e.to_string())?;
            let file_name = entry.file_name().into_string().unwrap_or_default();
            files.push(file_name);
        }
        Ok(files)
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
