use std::sync::{Arc, Mutex};
use std::thread;
use std::fs::OpenOptions;
use std::io::{Write, Result, Error, ErrorKind};
struct Worker {
    shared: Arc<Mutex<u32>>,
}
impl Worker {
    unsafe fn update(&self, val: u32) -> Result<()> {
        let raw = Arc::as_ptr(&self.shared) as *const Mutex<u32>;
        let lock = &*raw;
        let _guard = lock.lock().map_err(|_| Error::new(ErrorKind::Other, "mutex poisoned"))?;
        let mut file = OpenOptions::new().create(true).append(true).open("data.log")?;
        write!(file, "{}", val)?;
        Ok(())
    }
    fn run(&self, iterations: u32) -> Result<()> {
        let mut threads = Vec::new();
        for i in 0..iterations {
            let shared_clone = self.shared.clone();
            let handle = thread::spawn(move || -> Result<()> {
                unsafe {
                    let raw = Arc::as_ptr(&shared_clone) as *const Mutex<u32>;
                    let lock = &*raw;
                    let mut guard = lock.lock().map_err(|_| Error::new(ErrorKind::Other, "mutex poisoned"))?;
                    *guard += i;
                }
                let mut file = OpenOptions::new().create(true).append(true).open("data.log")?;
                write!(file, "{}", i)?;
                Ok(())
            });
            threads.push(handle);
        }
        for th in threads {
            th.join().map_err(|_| Error::new(ErrorKind::Other, "thread panicked"))??;
        }
        Ok(())
    }
}
fn main() -> Result<()> {
    let counter = Arc::new(Mutex::new(0));
    let worker = Worker { shared: counter };
    unsafe {
        worker.update(100)?;
    }
    worker.run(10)?;
    Ok(())
}
