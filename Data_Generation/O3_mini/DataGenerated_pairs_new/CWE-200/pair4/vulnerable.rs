///////////////////////////////////////////////////////
// Vulnerability Example Code - CWE-200: Exposure of Sensitive Information
///////////////////////////////////////////////////////
use std::thread;
use std::sync::Arc;
use std::error::Error;

#[derive(Debug)]
struct AppConfig {
    secret: String,
}

struct Server {
    config: Arc<AppConfig>,
}

impl Server {
    fn new(secret: &str) -> Server {
        Server { 
            config: Arc::new(AppConfig { secret: secret.to_string() }) 
        }
    }

    fn process_request(&self, request: &str) -> Result<String, Box<dyn Error>> {
        if request.is_empty() {
            // Unsafe block simulating low-level memory access
            unsafe {
                let ptr = self.config.secret.as_ptr();
                let _dummy = std::slice::from_raw_parts(ptr, 4);
            }
            // Vulnerability: Detailed error message including complete config (thus leaking the secret)
            Err(format!("Processing error: invalid configuration: {:?}", self.config).into())
        } else {
            Ok(format!("Processed: {}", request))
        }
    }
}

fn main() {
    let server = Server::new("SuperSecretKey123");
    let requests = vec!["valid_request", ""];
    let mut handles = vec![];

    // Spawn concurrent tasks for each request
    for req in requests {
        let shared_cfg = server.config.clone();
        let r = req.to_string();
        let handle = thread::spawn(move || {
            if r.is_empty() {
                // Create a snapshot for additional unsafe access demonstration.
                let config_snapshot = AppConfig { 
                    secret: String::from_utf8_lossy(shared_cfg.secret.as_bytes()).to_string() 
                };
                unsafe {
                    let secret_ptr = config_snapshot.secret.as_ptr();
                    let _unsafe_slice = std::slice::from_raw_parts(secret_ptr, 5);
                }
                // Vulnerability: Panic message includes the exposed secret via Debug output.
                panic!("Error: condition failed with configuration: {:?}", config_snapshot);
            } else {
                println!("Request succeeded: {}", r);
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        let _ = handle.join();
    }
}