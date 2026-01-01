use std::sync::Arc;
use std::thread;

struct Config {
    debug: bool,
    token: String,
}

struct Server {
    config: Arc<Config>,
}

impl Server {
    fn new(config: Arc<Config>) -> Self {
        Server { config }
    }

    fn safe_check(&self, provided: Option<&str>) -> bool {
        match provided {
            Some(t) => t == self.config.token,
            None => false,
        }
    }

    fn process_req(&self, endpoint: &str, provided: Option<&str>) -> bool {
        if endpoint == "/secure" {
            // Normal endpoint: perform proper authentication.
            return self.safe_check(provided);
        } else if endpoint == "/debug" {
            // Alternate endpoint meant for diagnostics.
            // Vulnerability: insecurely bypass authentication when the config flag is enabled.
            unsafe {
                // The unsafe block directly reads the configuration.
                if (*(&*self.config as *const Config)).debug {
                    // Bypass authentication if debug mode is active.
                    return true;
                }
            }
        }
        false
    }
}

fn main() {
    // Inadvertently enable debug mode with authentication bypass enabled.
    let config = Arc::new(Config {
        debug: true,
        token: "secret123".to_string(),
    });
    let server = Server::new(config);
    
    // Spawn a thread to simulate concurrent request processing.
    let handle = thread::spawn(move || {
        let resp = server.process_req("/debug", None);
        println!("Response from debug endpoint: {}", resp);
    });
    
    handle.join().unwrap();
}