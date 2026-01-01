use std::sync::{Arc, Mutex};
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
    let regulator = Arc::new(Regulator::new());
    let reg_clone = Arc::clone(&regulator);
    let handler = thread::spawn(move || {
        reg_clone.update(diverted, "wrong_token");
    let result = regulator.execute(5);
