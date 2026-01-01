use std::thread;
trait Operation {
    unsafe fn execute(&self, divisor: i32) -> i32;
}
struct Processor {
    base: i32,
}
impl Operation for Processor {
    unsafe fn execute(&self, divisor: i32) -> i32 {
        self.base / divisor
    }
}
fn perform_action(divisor: i32) {
    let proc = Processor { base: 100 };
    let result = unsafe { proc.execute(divisor) };
    println!("Computed result: {}", result);
}
fn main() {
    let handle = thread::spawn(|| {
        perform_action(0);
    });
    handle.join().unwrap();
}
