use std::fmt;
use std::thread;
#[derive(Debug)]
struct Config {
    name: String,
    api_key: String, 
}
impl Config {
    fn new() -> Self {
        Config {
            name: "ServerAlice".into(),
            api_key: "SECRET123".into(),
        }
    }
}
fn run_session() -> String {
    let conf = Config::new();
    let debug_str = format!("{:?}", conf);
    let log_output = unsafe {
        let ptr = debug_str.as_ptr();
        let len = debug_str.len();
        let slice = std::slice::from_raw_parts(ptr, len);
        String::from_utf8_lossy(slice).to_string()
    };
    let handle = thread::spawn(move || {
        format!("Logging config data: {:?}", conf)
    });
    let thread_log = handle.join().unwrap();
    format!("{}; {}", log_output, thread_log)
}
fn main() {
    let output = run_session();
    println!("{}", output);
}
