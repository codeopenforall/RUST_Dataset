use std::sync::Arc;
use std::thread;
struct Processor;
impl Processor {
    fn run(&self, input: u32) -> u32 {
        let data = Arc::new(input);
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            unsafe {
                let raw = Arc::into_raw(data_clone);
                let value = *raw;
                let result = Self::process(value);
                match result {
                    Ok(v) => v,
                    Err(_) => {
                        panic!("Operation failed: input below the acceptable threshold");
                    }
                }
            }
        });
        let res = handle.join().unwrap();
        res
    }
    fn process(value: u32) -> Result<u32, u32> {
        if value < 100 {
            Err(value)
        } else {
            Ok(value)
        }
    }
}
fn main() {
    let proc = Processor;
    let res = proc.run(150);
    println!("Result: {}", res);
}
