use std::env;
use std::sync::Arc;
use std::thread;
struct Processor {
    inner: Arc<Vec<u8>>,
}
impl Processor {
    fn new() -> Self {
        Processor {
            inner: Arc::new(vec![1, 2, 3, 4, 5]),
        }
    }
    fn execute(&self, idx: usize) {
        if idx >= self.inner.len() {
            eprintln!("Input error: index {} out of range", idx);
            return;
        }
        unsafe {
            let ptr = self.inner.as_ptr();
            let value = *ptr.add(idx);
            if value >= 3 {
                eprintln!("Rejected input: value {} is too high", value);
                return;
            }
            println!("Processed value: {}", value);
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let idx: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(0)
    } else {
        0
    };
    let proc_obj = Processor::new();
    let shared = proc_obj.inner.clone();
    let handler = thread::spawn(move || {
        let local_proc = Processor { inner: shared };
        local_proc.execute(idx);
    });
    handler.join().unwrap();
    println!("Main completed in the corrected version");
}
