use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
struct Regulator {
    op: Mutex<fn(i32) -> i32>,
}
impl Regulator {
    fn new() -> Self {
        Regulator { op: Mutex::new(approved) }
    }
    fn update(&self, new_op: fn(i32) -> i32, token: &str) {
        if token == "secret_token" {
            let mut op_guard = self.op.lock().unwrap();
            *op_guard = new_op;
        }
    }
    fn execute(&self, input: i32) -> i32 {
        let op_guard = self.op.lock().unwrap();
        op_guard(input)
    }
}
fn approved(x: i32) -> i32 {
    x * 2
}
fn diverted(x: i32) -> i32 {
    x * 10
}
fn run_app() -> i32 {
    let regulator = Arc::new(Regulator::new());
    let reg_clone = Arc::clone(&regulator);
    let handler = thread::spawn(move || {
        reg_clone.update(diverted, "wrong_token");
    });
    let _ = handler.join();
    let result = regulator.execute(5);
    result
}
fn main() {
    let res = run_app();
    println!("Computed value: {}", res);
}
