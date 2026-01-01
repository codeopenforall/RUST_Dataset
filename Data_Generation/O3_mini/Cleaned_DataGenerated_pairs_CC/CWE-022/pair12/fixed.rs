use std::{fs, path::{Path, PathBuf}, sync::{Arc, Mutex}};
struct Loader {
    base: PathBuf,
}
impl Loader {
    fn new() -> Self {
        Self { base: PathBuf::from("data") }
    }
    fn load(&self, file: &str) -> Result<String, String> {
        let candidate = self.base.join(file);
        let base_canonical = fs::canonicalize(&self.base).map_err(|e| e.to_string())?;
        let candidate_canonical = fs::canonicalize(&candidate).map_err(|e| e.to_string())?;
        if !candidate_canonical.starts_with(&base_canonical) {
            return Err("Access to files outside permitted directory is not allowed".to_owned());
        }
        fs::read_to_string(candidate).map_err(|e| e.to_string())
    }
}
fn main() {
    let loader = Arc::new(Mutex::new(Loader::new()));
    let handles: Vec<_> = (0..2)
        .map(|_| {
            let loader = Arc::clone(&loader);
            std::thread::spawn(move || {
                let result = loader.lock().unwrap().load("../secret.txt");
                match result {
                    Ok(content) => println!("Loaded content: {}", content),
                    Err(err) => println!("Error: {}", err),
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
}
