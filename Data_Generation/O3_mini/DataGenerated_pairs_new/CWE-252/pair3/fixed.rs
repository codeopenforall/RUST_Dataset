///////////////////////////
// Fixed Code Start
///////////////////////////
use std::env;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;

fn run() -> io::Result<()> {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for i in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || -> io::Result<()> {
            unsafe {
                // Determine the file path based on environment variable.
                let path = if env::var("SIM_FAILURE").is_ok() {
                    "invalid_dir/log.txt"
                } else {
                    "log.txt"
                };
                let mut file = OpenOptions::new().write(true).create(true).open(path)?;
                // Now the error from write_all is properly checked and propagated.
                file.write_all(format!("Thread {} writing\n", i).as_bytes())?;
            }
            let mut num = counter.lock().unwrap();
            *num += 1;
            Ok(())
        });
        handles.push(handle);
    }
    for handle in handles {
        // Propagate thread panics as IO errors.
        handle.join().map_err(|_| io::Error::new(io::ErrorKind::Other, "Thread panicked"))??;
    }
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Critical error: {}", e);
        std::process::exit(1);
    }
}
///////////////////////////
// Fixed Code End
///////////////////////////