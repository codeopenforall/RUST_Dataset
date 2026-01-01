use std::sync::Arc;
use std::thread;
struct Processor {}
impl Processor {
    fn new() -> Processor {
        Processor {}
    }
    fn process(&self, user_input: &str) -> String {
        format!("{}", user_input)
    }
}
fn main() {
    let proc = Arc::new(Processor::new());
    let proc_clone = Arc::clone(&proc);
    let handle = thread::spawn(move || {
        let res = proc_clone.process("%x %x %x");
        println!("{}", res);
    });
    handle.join().unwrap();
}
