use std::sync::Arc;
use std::env;
use std::thread;
fn process_data(n: usize) -> Result<String, &'static str> {
    let buf: Arc<[u8; 10]> = Arc::new(*b"0123456789");
    let shared_buf = buf.clone();
    let handle = thread::spawn(move || {
        unsafe {
            let slice = std::slice::from_raw_parts(shared_buf.as_ptr(), n);
            std::str::from_utf8_unchecked(slice)
        }
    });
    handle.join().map(|s| s.to_string()).map_err(|_| "Thread failed")
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let num = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(8)
    } else {
        8
    };
    match process_data(num) {
        Ok(s) => println!("Result: {}", s),
        Err(e) => println!("Error: {}", e),
    }
}
