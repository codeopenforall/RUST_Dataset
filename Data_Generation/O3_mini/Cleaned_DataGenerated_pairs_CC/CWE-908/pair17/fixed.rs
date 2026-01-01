use std::thread;
struct Config {
    name: String,
    timeout: u32, 
}
impl Config {
    fn construct() -> Self {
        Config {
            name: String::from("example"),
            timeout: 30, 
        }
    }
}
fn load_config() -> Config {
    Config::construct()
}
fn execute(config: Config) -> u32 {
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
