use std::mem::MaybeUninit;
use std::thread;
struct Config {
    name: String,
    timeout: u32, 
}
impl Config {
    unsafe fn construct() -> Self {
        let mut config_uninit = MaybeUninit::<Config>::uninit();
        let ptr = config_uninit.as_mut_ptr();
        std::ptr::write(&mut (*ptr).name, String::from("example"));
        config_uninit.assume_init()
    }
}
fn load_config() -> Config {
    unsafe { Config::construct() }
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
