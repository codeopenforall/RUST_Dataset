use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;

struct SharedData {
    count: usize,
}

impl SharedData {
    fn new() -> Self {
        SharedData { count: 0 }
    }
}

unsafe fn simulate_write(file: &mut std::fs::File, count: usize) -> io::Result<()> {
    if count == 3 {
        return Err(io::Error::new(io::ErrorKind::Other, "Simulated write failure"));
    }
    file.write_all(format!("count: {}\n", count).as_bytes())
}

trait Operations {
    fn increase(&self) -> io::Result<()>;
}

impl Operations for Arc<Mutex<SharedData>> {
    fn increase(&self) -> io::Result<()> {
        let mut data = self.lock().unwrap();
        data.count += 1;
        let mut file = OpenOptions::new().append(true).create(true).open("log.txt")?;
        unsafe {
            // FIX: Propagate any error from simulate_write rather than ignoring it.
            simulate_write(&mut file, data.count)?;
        }
        Ok(())
    }
}

fn main() {
    let shared = Arc::new(Mutex::new(SharedData::new()));
    let mut handles = vec![];

    for _ in 0..5 {
        let handle_shared = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                // Panic on error to expose any write failures.
                handle_shared.increase().unwrap();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final count: {}", shared.lock().unwrap().count);
}