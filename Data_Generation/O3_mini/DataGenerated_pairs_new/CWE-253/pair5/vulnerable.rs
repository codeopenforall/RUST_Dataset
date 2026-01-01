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
                    Err(v) => { // vulnerability: misinterpreting error as success
                        v
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
    // Using an input below threshold triggers an error condition,
    // but the error is misinterpreted and returned as valid.
    let res = proc.run(50);
    println!("Result: {}", res);
}