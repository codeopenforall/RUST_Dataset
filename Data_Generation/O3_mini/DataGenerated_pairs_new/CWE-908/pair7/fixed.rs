use std::sync::Arc;
use std::thread;

struct Config {
    port: u16,
    host: String,
}

fn initialize_config() -> Config {
    // Securely initialize the entire configuration.
    Config {
        port: 8080,
        host: "127.0.0.1".to_string(),
    }
}

fn validate_config(cfg: &Config) -> bool {
    // Check that the host is properly set.
    cfg.host == "127.0.0.1"
}

fn main() {
    let cfg = initialize_config();
    let shared_cfg = Arc::new(cfg);
    let handle = thread::spawn({
        let shared_clone = Arc::clone(&shared_cfg);
        move || {
            // Assert ensures the configuration is properly initialized.
            assert!(
                validate_config(&shared_clone),
                "Configuration validation failed in secondary thread."
            );
            println!("Configuration validated in secondary thread.");
        }
    });
    handle.join().unwrap();
    assert!(
        validate_config(&shared_cfg),
        "Configuration validation failed in main thread."
    );
    println!("Configuration validated in main thread.");
}