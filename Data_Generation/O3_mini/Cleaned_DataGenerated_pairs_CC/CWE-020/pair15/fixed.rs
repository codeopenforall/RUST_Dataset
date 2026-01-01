use std::sync::Arc;
use std::thread;
struct Config {
    data: String,
}
impl Config {
    fn parse(input: &str) -> Result<Self, &'static str> {
        if input.len() < 2 {
            return Err("Input too short");
        }
        let header = &input[..2];
        let count: usize = header.parse().map_err(|_| "Header parse error")?;
        if input.len() < 2 + count {
            return Err("Payload length mismatch");
        }
        let slice = &input.as_bytes()[2..2 + count];
        let payload = std::str::from_utf8(slice).map_err(|_| "Invalid UTF-8 in payload")?;
        Ok(Config {
            data: payload.to_string(),
        })
    }
    fn process(&self) {
        let shared = Arc::new(self.data.clone());
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            println!("Processed data: {}", shared_clone);
        });
        handle.join().unwrap();
    }
}
fn main() {
    let input = std::env::args().nth(1).unwrap_or_else(|| "05hello".to_string());
    match Config::parse(&input) {
        Ok(cfg) => {
            cfg.process();
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
