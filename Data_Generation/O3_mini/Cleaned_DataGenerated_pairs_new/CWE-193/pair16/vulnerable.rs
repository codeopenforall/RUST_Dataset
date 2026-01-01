use std::ptr;
use std::thread;
struct Processor;
impl Processor {
    pub fn process(data: &[u32]) -> Vec<u32> {
        let n = data.len();
        let mut out: Vec<u32> = Vec::with_capacity(n);
        unsafe {
            let ptr_out = out.as_mut_ptr();
            for i in 0..n {
                ptr_out.add(i).write(data[i].wrapping_add(1));
            }
            out.set_len(n + 1);
        }
        out
    }
}
fn main() {
    let data = vec![10, 20, 30, 40];
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let data_thread = data.clone();
            thread::spawn(move || {
                let result = Processor::process(&data_thread);
                println!("Result: {:?}", result);
            })
        })
        .collect();
    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}
