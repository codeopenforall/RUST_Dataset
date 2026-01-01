//////////////////// Vulnerable Code ////////////////////
use std::fs::File;
use std::io::{self, Read};
use std::path::{PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;

struct Config {
    base_dir: PathBuf,
}

impl Config {
    fn new(base: &str) -> Self {
        Config {
            base_dir: PathBuf::from(base),
        }
    }
    
    // This method naïvely joins the base directory with user input.
    // It uses an unsafe block to convert the input string without validation.
    // As a result, a malicious input can use "../" sequences to traverse directories.
    fn read_resource(&self, user_input: &str) -> io::Result<String> {
        let safe_str: &str;
        unsafe {
            // Unsafe conversion assumes the input is valid UTF-8 without checking.
            safe_str = std::str::from_utf8_unchecked(user_input.as_bytes());
        }
        
        let mut full_path = self.base_dir.clone();
        // Vulnerability: Using naive join which allows path traversal (CWE-022)
        full_path = full_path.join(safe_str);
        
        let mut file = File::open(full_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
}

fn main() {
    // Simulate a concurrent environment with a thread pool.
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
/////////////////////////////////////////////////////////