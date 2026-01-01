use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
struct Loader {
    path: String,
}
impl Loader {
    fn invoke(&self) -> Result<String, &'static str> {
        unsafe {
            let _raw = self.path.as_ptr() as *const u8;
            if !self.path.starts_with("/") {
                return Ok("hijacked".to_string());
            }
            let output = Command::new(self.path.clone())
                .output()
                .map_err(|_| "Command execution failed")?;
            String::from_utf8(output.stdout).map_err(|_| "Output was not valid UTF-8")
        }
    }
}
fn main() {
    let loader = Arc::new(Mutex::new(Loader { path: "insecure".to_string() }));
    let loader_clone = loader.clone();
    let handle = thread::spawn(move || {
        let instance = loader_clone.lock().unwrap();
        match instance.invoke() {
            Ok(res) => println!("Result: {}", res),
            Err(err) => eprintln!("Error: {}", err),
        }
    });
    handle.join().unwrap();
}
