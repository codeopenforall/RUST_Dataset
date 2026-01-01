use std::mem::MaybeUninit;
use std::sync::Arc;
use std::thread;

struct Config {
    port: u16,
    host: String,
}

fn initialize_config() -> Config {
    // Vulnerable: partially initializes the configuration structure with unsafe code.
    let mut uninit_config: MaybeUninit<Config> = MaybeUninit::uninit();
    let config_ptr = uninit_config.as_mut_ptr();
    unsafe {
        // Only initialize the port field, leaving host uninitialized.
        (*config_ptr).port = 8080;
        // Intentionally omit initializing 'host', which remains uninitialized.
        // Assume the struct is fully initialized.
        let cfg = uninit_config.assume_init();
        cfg
    }
}

fn validate_config(cfg: &Config) -> bool {
    // Validates that the configuration has the expected host.
    cfg.host == "127.0.0.1"
}

fn main() {
    let cfg = initialize_config();
    let shared_cfg = Arc::new(cfg);
    let handle = thread::spawn({
        let shared_clone = Arc::clone(&shared_cfg);
        move || {
            // Use the configuration in a separate thread.
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