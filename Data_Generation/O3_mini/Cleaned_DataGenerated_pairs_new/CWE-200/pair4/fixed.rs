use std::thread;
use std::sync::Arc;
use std::error::Error;
struct AppConfig {
    secret: String,
}
impl AppConfig {
    fn safe_repr(&self) -> String {
        "AppConfig { secret: \"******\" }".to_string()
    }
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
            unsafe {
                let ptr = self.config.secret.as_ptr();
                let _dummy = std::slice::from_raw_parts(ptr, 4);
            }
            Err("Processing error: invalid configuration detected".into())
        } else {
            Ok(format!("Processed: {}", request))
        }
    }
}
fn main() {
    let server = Server::new("SuperSecretKey123");
    let requests = vec!["valid_request", ""];
    let mut handles = vec![];
    for req in requests {
        let shared_cfg = server.config.clone();
        let r = req.to_string();
        let handle = thread::spawn(move || {
            if r.is_empty() {
                let config_snapshot = AppConfig { 
                    secret: shared_cfg.secret.clone() 
                };
                unsafe {
                    let secret_ptr = config_snapshot.secret.as_ptr();
                    let _unsafe_slice = std::slice::from_raw_parts(secret_ptr, 5);
                }
                panic!("Error: condition failed with configuration: {}", config_snapshot.safe_repr());
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
