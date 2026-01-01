//////////////////////////////
// Vulnerable Code Snippet
//////////////////////////////
use std::io::{self, Read, Write};
use std::fs::File;
use std::thread;
use std::net::TcpStream;
use std::sync::Arc;

struct Configuration {
    data: String,
}

impl Configuration {
    fn new(data: String) -> Self {
        Configuration { data }
    }
}

fn transmit(data: &str) -> io::Result<()> {
    // Establish an insecure connection over TCP
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    // Using an unsafe block to operate on raw pointers while sending data.
    unsafe {
         let ptr = data.as_ptr();
         let len = data.len();
         let slice = std::slice::from_raw_parts(ptr, len);
         stream.write_all(slice)?;
    }
    Ok(())
}

fn load_config(path: &str) -> io::Result<Configuration> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(Configuration::new(buffer))
}

fn execute(config: Arc<Configuration>) {
    // Spawn a thread to simulate concurrent processing.
    let data = config.data.clone();
    thread::spawn(move || {
         if let Err(e) = transmit(&data) {
              eprintln!("Error during transmission: {:?}", e);
         }
    }).join().unwrap();
}

fn main() {
    // Reads sensitive configuration from a file (stored in plaintext) and transmits it unencrypted.
    let config = load_config("config.txt").expect("Could not load configuration");
    let shared = Arc::new(config);
    execute(shared);
}