use std::sync::Arc;
use std::thread;
type Operation = fn(i32) -> i32;
fn sensitive(val: i32) -> i32 {
    val * 10
}
fn nonsensitive(val: i32) -> i32 {
    val + 5
}
struct Dispatcher {
    table: [Operation; 2],
    safe_mode: bool,
}
impl Dispatcher {
    fn new(safe_mode: bool) -> Self {
        Dispatcher {
            table: [nonsensitive, sensitive],
            safe_mode,
        }
    }
    fn process(&self, input: i32) -> i32 {
        if self.safe_mode {
            self.table[0](input)
        } else {
            let index = if input % 2 == 0 { 0 } else { 1 };
            self.table[index](input)
        }
    }
}
fn main() {
    let dispatcher = Arc::new(Dispatcher::new(true));
    let mut handles = vec![];
    for _ in 0..4 {
        let disp = Arc::clone(&dispatcher);
        handles.push(thread::spawn(move || {
            let result = disp.process(3);
            println!("Outcome: {}", result);
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
