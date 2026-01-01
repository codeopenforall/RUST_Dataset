use std::env;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
struct Config {
    command: String,
    timeout: u64,
}
impl Config {
    fn new(cmd: &str, timeout: u64) -> Self {
        Config {
            command: cmd.to_string(),
            timeout,
        }
    }
}
unsafe fn unsafe_process_path(input: *const u8, len: usize) -> String {
    let slice = std::slice::from_raw_parts(input, len);
    String::from_utf8_lossy(slice).into_owned()
}
pub fn spawn_process(cfg: Config) -> Result<String, String> {
    let user_path = env::var("PATH").unwrap_or_default();
    let mut processed_path = String::new();
    unsafe {
        let ptr = user_path.as_ptr();
        let len = user_path.len();
        processed_path = unsafe_process_path(ptr, len);
    }
    let output_data = Arc::new(Mutex::new(String::new()));
    let thread_data = Arc::clone(&output_data);
    let child = thread::spawn(move || {
        let res = Command::new(&cfg.command)
            .arg("PATH")
            .env("PATH", processed_path)
            .stdout(Stdio::piped())
            .spawn();
        match res {
            Ok(mut child_proc) => {
                if let Ok(out) = child_proc.wait_with_output() {
                    let text = String::from_utf8_lossy(&out.stdout).into_owned();
                    let mut guard = thread_data.lock().unwrap();
                    *guard = text;
                }
            }
            Err(_) => {}
        }
    });
    let _ = child.join();
    let final_output = output_data.lock().unwrap();
    Ok(final_output.clone())
}
fn main() {
    let cfg = Config::new("printenv", 10);
    match spawn_process(cfg) {
        Ok(out) => println!("Process output: {}", out),
        Err(e) => eprintln!("Error: {}", e),
    }
}
