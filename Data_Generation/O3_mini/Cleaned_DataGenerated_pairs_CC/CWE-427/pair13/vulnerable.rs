use std::env;
use std::process::Command;
use std::thread;
use std::sync::{Arc, Mutex};
struct Executor {
    user_search: String,
}
impl Executor {
    fn run(&self) {
        let raw_ptr = self.user_search.as_ptr();
        let len = self.user_search.len();
        let shared_ptr = Arc::new(Mutex::new(raw_ptr));
        let handle = {
            let sp = Arc::clone(&shared_ptr);
            thread::spawn(move || {
                unsafe {
                    let stored_ptr = *sp.lock().unwrap();
                    let slice = std::slice::from_raw_parts(stored_ptr, len);
                    let path_str = std::str::from_utf8_unchecked(slice);
                    println!("User supplied search path: {}", path_str);
                }
            })
        };
        handle.join().unwrap();
        env::set_var("PATH", &self.user_search);
        let output = Command::new("echo")
            .arg("Executing command")
            .output()
            .expect("failed to execute process");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}
fn main() {
    let user_search = env::var("BAD_PATH")
        .unwrap_or_else(|_| "/usr/bin:/bin".to_string());
    let exec = Executor { user_search };
    exec.run();
}
