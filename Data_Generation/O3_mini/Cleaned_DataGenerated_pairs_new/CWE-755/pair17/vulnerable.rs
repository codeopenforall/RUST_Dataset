use std::env;
use std::sync::Arc;
use std::thread;
struct Config {
    value: i32,
}
impl Config {
    unsafe fn from_str(data: &str) -> Self {
        let val = data.trim().parse::<i32>().unwrap_or_default();
        Config { value: val }
    }
}
fn process_input(input: String) -> Result<i32, String> {
    let data = Arc::new(input);
    let data_clone = Arc::clone(&data);
    let handle = thread::spawn(move || {
        let raw = data_clone.as_str();
        unsafe { Config::from_str(raw) }
    });
    let config = handle.join().map_err(|_| "Thread panicked".to_string())?;
    Ok(config.value)
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        args[1].clone()
    } else {
        "bad".to_string()
    };
    match process_input(input) {
        Ok(val) => {
            println!("Computed value: {}", val);
            std::process::exit(if val != 0 { 0 } else { 1 });
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
