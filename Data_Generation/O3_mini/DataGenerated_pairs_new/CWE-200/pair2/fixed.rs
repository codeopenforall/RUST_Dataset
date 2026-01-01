////////////////////////////////////////////////////////////
// Corrected Code (Sensitive Information Masked)
////////////////////////////////////////////////////////////
use std::fmt;
use std::thread;

struct Config {
    name: String,
    api_key: String, // Sensitive secret stored internally
}

impl Config {
    fn new() -> Self {
        Config {
            name: "ServerAlice".into(),
            api_key: "SECRET123".into(),
        }
    }
}

// Custom Debug that masks the sensitive API key.
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
    // Safe logging: use the custom Debug implementation that masks the sensitive value.
    let log_output = format!("{:?}", conf);
    // Concurrency: A thread also logs the configuration, benefiting from the safe Debug.
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