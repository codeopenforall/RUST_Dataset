use std::panic;
use std::thread;
struct Processor {}
trait Execute {
    fn run(&self);
}
impl Execute for Processor {
    fn run(&self) {
        let handler = thread::spawn(|| {
            unsafe {
                panic!("Error: Unexpected panic in unsafe operation");
            }
        });
        handler.join().unwrap();
    }
}
fn main() {
    let unit = Processor {};
    unit.run();
    println!("Processing completed (vulnerable version).");
}
