use std::thread;
struct Processor;
impl Processor {
    pub fn compute(input: u32) -> Result<u64, &'static str> {
        let capacity: u32 = input.checked_mul(1000).ok_or("overflow detected")?;
        let handle = thread::spawn(move || {
            let cap_u64 = capacity as u64;
            cap_u64
                .checked_mul(cap_u64.checked_sub(1).unwrap())
                .and_then(|v| v.checked_div(2))
                .unwrap_or(0)
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
