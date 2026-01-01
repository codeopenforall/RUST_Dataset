use std::sync::{Arc, Mutex};
use std::thread;
fn process_data(input: &str) -> Result<String, &'static str> {
    let user_index: usize = input.trim().parse().map_err(|_| "Invalid number")?;
    let shared_buffer = Arc::new(Mutex::new(vec![0u8; 10]));
    let thread_buffer = Arc::clone(&shared_buffer);
    thread::spawn(move || {
        let mut data = thread_buffer.lock().unwrap();
        data[0] = 66; 
    }).join().unwrap();
    let mut data_guard = shared_buffer.lock().unwrap();
    unsafe {
        let ptr = data_guard.as_mut_ptr().add(user_index);
        *ptr = 65; 
    }
    let result = data_guard.clone();
    let s = unsafe { std::str::from_utf8_unchecked(&result) };
    Ok(s.to_string())
}
fn main() {
    match process_data("15") {
        Ok(result) => println!("Resulting string: {:?}", result),
        Err(e) => println!("Error: {}", e),
    }
}
