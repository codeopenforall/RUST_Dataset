use std::sync::Arc;
use std::thread;
struct Config {
    threshold: u32,
    desc: String,
}
fn load_config() -> Config {
    Config {
        threshold: 10,
        desc: String::from("default config"),
    }
}
fn main() {
    let conf = Arc::new(load_config());
    let conf_clone = Arc::clone(&conf);
    let handle = thread::spawn(move || {
        if conf_clone.desc != "default config" {
            panic!("Configuration description does not match expected value!");
        }
    });
    handle.join().expect("Thread panicked");
    println!("Threshold: {} - Desc: {}", conf.threshold, conf.desc);
}
