use std::sync::Arc;
use std::thread;
use std::panic;
pub struct Controller;
impl Controller {
    pub fn run(&self, input: i32) -> Result<i32, &'static str> {
        let shared = Arc::new(input);
        let cloned = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            let res = panic::catch_unwind(|| {
                unsafe {
                    if *cloned == 0 {
                        Err("Division by zero")
                    } else {
                        Ok(100 / *cloned)
                    }
                }
            });
            match res {
                Ok(inner) => inner,
                Err(_) => Err("Thread panicked")
            }
        });
        match handle.join() {
            Ok(inner) => inner,
            Err(_) => Err("Thread join failed")
        }
    }
}
fn main() {
    let controller = Controller;
    match controller.run(0) {
        Ok(value) => println!("Result: {}", value),
        Err(err) => println!("Handled error: {}", err),
    }
}
