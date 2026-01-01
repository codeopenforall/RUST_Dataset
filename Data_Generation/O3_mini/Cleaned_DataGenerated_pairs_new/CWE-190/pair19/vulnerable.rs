use std::thread;
struct Processor;
impl Processor {
    pub fn compute(input: u32) -> Result<u64, &'static str> {
        let capacity: u32 = unsafe {
            input.wrapping_mul(1000)
        };
        let handle = thread::spawn(move || {
            let cap_u64 = capacity as u64;
            let sum = (cap_u64 * cap_u64.wrapping_sub(1)) / 2;
            sum
        });
        match handle.join() {
            Ok(result) => Ok(result),
            Err(_) => Err("Thread panicked"),
        }
    }
}
fn main() {
    let input: u32 = 5_000_000;
    match Processor::compute(input) {
        Ok(result) => {
            println!("Computed result: {}", result);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
