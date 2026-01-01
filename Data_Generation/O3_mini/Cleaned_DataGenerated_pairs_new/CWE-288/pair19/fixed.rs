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
        if endpoint == "/secure" || endpoint == "/debug" {
            return self.safe_check(provided);
        }
        false
    }
}
fn main() {
    let config = Arc::new(Config {
        debug: false,
        token: "secret123".to_string(),
    });
    let server = Server::new(config);
    let handle = thread::spawn(move || {
        let resp = server.process_req("/debug", Some("secret123"));
        println!("Response from debug endpoint: {}", resp);
    });
    handle.join().unwrap();
}
