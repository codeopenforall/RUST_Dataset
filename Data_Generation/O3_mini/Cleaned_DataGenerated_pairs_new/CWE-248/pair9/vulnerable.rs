use std::sync::Arc;
use std::thread;
pub struct Controller;
impl Controller {
    pub fn run(&self, input: i32) -> Result<i32, &'static str> {
        let shared = Arc::new(input);
        let cloned = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            unsafe {
                if *cloned == 0 {
                    100 / *cloned
                } else {
                    100 / *cloned
                }
            }
        });
        let result = handle.join().unwrap();
        Ok(result)
    }
}
fn main() {
    let controller = Controller;
    match controller.run(0) {
        Ok(value) => println!("Result: {}", value),
        Err(err) => println!("Error: {}", err),
    }
}
