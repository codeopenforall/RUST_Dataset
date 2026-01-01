use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;
struct Config {
    base_dir: PathBuf,
}
impl Config {
    fn new(base: &str) -> Self {
        let base_path = PathBuf::from(base).canonicalize().expect("Invalid base directory");
        Config {
            base_dir: base_path,
        }
    }
    fn read_resource(&self, user_input: &str) -> io::Result<String> {
        let full_path = self.base_dir.join(user_input);
        let canon_full = full_path.canonicalize()?;
        if !canon_full.starts_with(&self.base_dir) {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Access denied"));
        }
        let mut file = File::open(canon_full)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
}
fn main() {
    let cfg = Arc::new(Config::new("./resources"));
    let inputs = vec!["data.txt", "../secret.txt", "info/config.txt"];
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    for input in inputs {
        let cfg_clone = Arc::clone(&cfg);
        let results_clone = Arc::clone(&results);
        let inp = input.to_string();
        let handle = thread::spawn(move || {
            let result = cfg_clone.read_resource(&inp);
            let mut res = results_clone.lock().unwrap();
            res.push((inp, result));
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
