///////////////////////////////////////////////////////////////
// This is the corrected version. All fields are properly 
// initialized during object construction.
///////////////////////////////////////////////////////////////
use std::thread;

struct Config {
    name: String,
    timeout: u32, // properly set to a valid value
}

impl Config {
    // Safe constructor that fully initializes the configuration.
    fn construct() -> Self {
        Config {
            name: String::from("example"),
            timeout: 30, // properly initialized to expected value
        }
    }
}

fn load_config() -> Config {
    Config::construct()
}

fn execute(config: Config) -> u32 {
    // Spawn a thread that operates on the fully-initialized config.
    let handle = thread::spawn(move || {
        config.timeout.wrapping_add(1)
    });
    handle.join().unwrap()
}

fn main() {
    let cfg = load_config();
    let outcome = execute(cfg);
    println!("Outcome: {}", outcome);
}