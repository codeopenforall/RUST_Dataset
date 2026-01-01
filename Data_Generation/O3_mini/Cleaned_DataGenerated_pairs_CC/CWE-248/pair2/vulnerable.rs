use std::thread;
struct Processor;
impl Processor {
    fn execute(&self, trigger: bool) {
        unsafe {
            let handle = thread::spawn(move || {
                if trigger {
                    panic!("Critical failure encountered in concurrent task");
                }
                println!("Task completed without incident.");
            });
            handle.join().unwrap();
        }
    }
}
fn main() {
    let service = Processor;
    service.execute(true);
}
