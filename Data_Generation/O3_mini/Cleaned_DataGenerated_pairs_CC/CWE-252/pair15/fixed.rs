--------------------------------------------------
use std::sync::{Arc, Mutex};
use std::thread;
struct Buffer {
    data: [u8; 1024],
}
impl Buffer {
    unsafe fn copy_into(&mut self, src: &[u8]) -> Result<(), &'static str> {
        if src.len() > self.data.len() {
            return Err("source too long");
        }
        std::ptr::copy_nonoverlapping(src.as_ptr(), self.data.as_mut_ptr(), src.len());
        Ok(())
    }
}
fn execute_task(input: &[u8]) -> Result<(), &'static str> {
    let shared = Arc::new(Mutex::new(Buffer { data: [0; 1024] }));
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        let mut guard = shared_clone.lock().unwrap();
        unsafe { guard.copy_into(input) }
    });
    let thread_result = handle.join().map_err(|_| "thread panicked")?;
    thread_result
}
pub fn run_processing(input: &[u8]) -> Result<(), &'static str> {
    execute_task(input)
}
fn main() {
    let input = vec![1u8; 2048];
    match run_processing(&input) {
        Ok(_) => println!("Operation completed successfully (fixed)."),
        Err(err) => {
            println!("Error encountered: {}", err);
            std::process::exit(1);
        }
    }
}
--------------------------------------------------
