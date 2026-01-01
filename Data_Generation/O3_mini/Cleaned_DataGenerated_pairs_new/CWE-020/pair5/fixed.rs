use std::env;
use std::sync::Arc;
use std::thread;
struct Handler {
    data: Box<[u8]>,
}
impl Handler {
    fn new() -> Self {
        let mut vec = vec![1u8; 100];
        vec[50] = 255; 
        Handler {
            data: vec.into_boxed_slice(),
        }
    }
    fn execute(&self, input: &str) -> Result<(), &'static str> {
        let idx = usize::from_str_radix(input, 16).map_err(|_| "parse error")?;
        let buf_len = 10;
        if idx.checked_add(buf_len).map_or(true, |end| end > self.data.len()) {
            return Err("index out of bounds");
        }
        let mut buffer = vec![0u8; buf_len];
        unsafe {
            let src = self.data.as_ptr().add(idx);
            std::ptr::copy_nonoverlapping(src, buffer.as_mut_ptr(), buf_len);
        }
        let sum: u8 = buffer.iter().fold(0, |acc, &x| acc.wrapping_add(x));
        if sum == 0 {
            Err("data sum is zero")
        } else {
            Ok(())
        }
    }
}
fn run_app() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 { &args[1] } else { "00" };
    let handler = Arc::new(Handler::new());
    let handler_clone = Arc::clone(&handler);
    let input_clone = input.to_string();
    let handle = thread::spawn(move || {
        match handler_clone.execute(&input_clone) {
            Ok(_)  => println!("Concurrent execution succeeded."),
            Err(e) => println!("Concurrent execution failed: {}", e),
        }
    });
    match handler.execute(input) {
        Ok(_)  => println!("Main execution succeeded."),
        Err(e) => println!("Main execution failed: {}", e),
    }
    handle.join().unwrap();
}
fn main() {
    run_app();
}
