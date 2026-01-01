use std::mem::MaybeUninit;
use std::sync::Arc;
use std::thread;
struct Config {
    port: u16,
    host: String,
}
fn initialize_config() -> Config {
    let mut uninit_config: MaybeUninit<Config> = MaybeUninit::uninit();
    let config_ptr = uninit_config.as_mut_ptr();
    unsafe {
        (*config_ptr).port = 8080;
        let cfg = uninit_config.assume_init();
        cfg
    }
}
fn validate_config(cfg: &Config) -> bool {
    cfg.host == "127.0.0.1"
}
fn main() {
    let cfg = initialize_config();
    let shared_cfg = Arc::new(cfg);
    let handle = thread::spawn({
        let shared_clone = Arc::clone(&shared_cfg);
        move || {
            if validate_config(&shared_clone) {
                println!("Configuration validated in secondary thread.");
            } else {
                println!("Configuration validation FAILED in secondary thread.");
            }
        }
    });
    handle.join().unwrap();
    if validate_config(&shared_cfg) {
        println!("Configuration validated in main thread.");
    } else {
        println!("Configuration validation FAILED in main thread.");
    }
}
