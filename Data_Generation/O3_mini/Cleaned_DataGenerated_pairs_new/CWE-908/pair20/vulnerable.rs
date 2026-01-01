use std::mem::MaybeUninit;
use std::thread;
struct Config {
    value: i32,
    message: String,
}
fn initialize() -> Config {
    unsafe {
        let mut config: MaybeUninit<Config> = MaybeUninit::uninit();            
        let config_ptr = config.as_mut_ptr();
        (*config_ptr).value = 42;                                                 
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
