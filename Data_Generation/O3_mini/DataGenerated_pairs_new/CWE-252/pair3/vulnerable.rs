///////////////////////////
// Vulnerable Code Start
///////////////////////////
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;

fn run() {
    let counter = Arc::new(Mutex::new(0));
    let handles: Vec<_> = (0..5)
        .map(|i| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                unsafe {
                    // Determine the file path based on environment variable.
                    let path = if env::var("SIM_FAILURE").is_ok() {
                        "invalid_dir/log.txt"
                    } else {
                        "log.txt"
                    };
                    let file_result = OpenOptions::new().write(true).create(true).open(path);
                    if let Ok(mut file) = file_result {
                        // CWE-252: Ignoring the error returned by write_all.
                        // The return value is discarded instead of being checked.
                        let _ = file.write_all(format!("Thread {} writing\n", i).as_bytes());
                    }
                }
                let mut num = counter.lock().unwrap();
                *num += 1;
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
}

fn main() {
    run();
}
///////////////////////////
// Vulnerable Code End
///////////////////////////