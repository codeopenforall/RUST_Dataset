use std::sync::{Arc, Mutex};
use std::thread;
fn critical(input: u32) -> Result<(), &'static str> {
    if input < 100 {
        Ok(())
    } else {
        Err("Input too high")
    }
}
fn execute(input: u32) -> Result<u32, &'static str> {
    unsafe {
        critical(input)?;
    }
    let outcome = Arc::new(Mutex::new(0));
    let outcome_thread = Arc::clone(&outcome);
    let h = thread::spawn(move || {
        let mut val = outcome_thread.lock().unwrap();
        *val = 42;
    });
    h.join().unwrap();
    Ok(*outcome.lock().unwrap())
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_val: u32 = if args.len() > 1 {
        args[1].parse().unwrap_or(0)
    } else {
        0
    };
    match execute(input_val) {
        Ok(v) => println!("Success: {}", v),
        Err(e) => println!("Failure: {}", e),
    }
}
