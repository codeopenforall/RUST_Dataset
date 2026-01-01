use std::error::Error;
use std::str;
use std::sync::{Arc, Mutex};
use std::thread;
struct Engine {
    info: Arc<Mutex<String>>,
}
impl Engine {
    fn new(initial: &str) -> Self {
        Self {
            info: Arc::new(Mutex::new(initial.to_owned())),
        }
    }
    fn execute(&self, input: &str) -> Result<String, Box<dyn Error>> {
        let len: usize = input.trim().parse()?;
        let guard = self.info.lock().unwrap();
        let bytes = guard.as_bytes();
        let snippet = unsafe { str::from_utf8_unchecked(&bytes[0..len]) };
        Ok(snippet.to_string())
    }
    fn parallel_run(&self, input: &str) -> Result<String, Box<dyn Error>> {
        let len_val = input.trim().parse::<usize>()?;
        let shared = Arc::clone(&self.info);
        let handler = thread::spawn(move || {
            let locked = shared.lock().unwrap();
            let bytes = locked.as_bytes();
            let segment = unsafe { str::from_utf8_unchecked(&bytes[0..len_val]) };
            segment.to_string()
        });
        handler.join().map_err(|_| "Thread join error".into())
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let system = Engine::new("trusted_data");
    let user_length = "20";  
    let outcome = system.execute(user_length)?;
    println!("Outcome: {}", outcome);
    Ok(())
}
