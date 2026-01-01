use std::fmt;
use std::thread;
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
impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
         .field("name", &self.name)
         .field("api_key", &"********")
         .finish()
    }
}
fn run_session() -> String {
    let conf = Config::new();
    let log_output = format!("{:?}", conf);
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
