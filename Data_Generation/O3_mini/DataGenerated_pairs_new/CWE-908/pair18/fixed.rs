/* 
   In this corrected version, the configuration is fully initialized before being used.
   All fields of the Config struct are properly assigned, eliminating the use of uninitialized memory.
*/
use std::sync::Arc;
use std::thread;

struct Config {
    threshold: u32,
    desc: String,
}

fn load_config() -> Config {
    // Fully initialize the configuration.
    Config {
        threshold: 10,
        desc: String::from("default config"),
    }
}

fn main() {
    // Load and fully initialize the configuration.
    let conf = Arc::new(load_config());

    // Spawn a thread that concurrently inspects the configuration.
    let conf_clone = Arc::clone(&conf);
    let handle = thread::spawn(move || {
        // Verify that the configuration field 'desc' is properly set.
        if conf_clone.desc != "default config" {
            panic!("Configuration description does not match expected value!");
        }
    });

    // Wait for the thread to complete.
    handle.join().expect("Thread panicked");

    // Use the configuration.
    println!("Threshold: {} - Desc: {}", conf.threshold, conf.desc);
}