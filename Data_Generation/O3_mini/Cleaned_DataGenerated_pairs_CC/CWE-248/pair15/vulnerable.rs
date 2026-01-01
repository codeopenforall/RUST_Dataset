use std::{thread, panic};
struct Engine;
impl Engine {
    fn run(&self, trigger: bool) -> i32 {
        let handle = thread::spawn(move || {
            unsafe {
                if trigger {
                    panic!("uncaught panic in unsafe block");
                }
                10
            }
        });
        handle.join().unwrap()
    }
}
fn main() {
    let engine = Engine;
    let result = engine.run(true);
    println!("Result: {}", result);
}
