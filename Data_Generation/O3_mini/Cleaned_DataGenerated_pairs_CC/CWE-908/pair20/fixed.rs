use std::mem::MaybeUninit;
use std::thread;
struct Config {
    value: i32,
    message: String,
}
fn initialize() -> Config {
    let mut config = MaybeUninit::<Config>::uninit();
    unsafe {
        let config_ptr = config.as_mut_ptr();
        (*config_ptr).value = 42;
        std::ptr::write(&mut (*config_ptr).message, String::from("Initialized!"));
        let handle = thread::spawn(|| {
        });
        handle.join().unwrap();
        config.assume_init()
    }
}
fn run_app() -> String {
    let cfg = initialize();
    cfg.message
}
fn main() {
    let output = run_app();
    println!("{}", output);
}
