use std::sync::Mutex;
use std::vec::Vec;
use std::thread;
static mut GLOBAL_DATA: Option<Mutex<Vec<*mut u32>>> = None;
fn initialize() {
    unsafe {
        GLOBAL_DATA = Some(Mutex::new(Vec::new()));
    }
}
struct Processor;
trait Executor {
    fn run(&self, count: u32) -> Result<u32, &'static str>;
}
impl Executor for Processor {
    fn run(&self, count: u32) -> Result<u32, &'static str> {
        let mut threads = Vec::new();
        for i in 0..count {
            let handle = thread::spawn(move || {
                let raw_ptr = Box::into_raw(Box::new(i));
                unsafe {
                    if let Some(ref container) = GLOBAL_DATA {
                        container.lock().unwrap().push(raw_ptr);
                    }
                }
            });
            threads.push(handle);
        }
        for t in threads {
            t.join().unwrap();
        }
        Ok(count)
    }
}
fn main() {
    initialize();
    let proc_inst = Processor;
    let result = proc_inst.run(150);
    println!("Processing complete: {:?}", result);
}
