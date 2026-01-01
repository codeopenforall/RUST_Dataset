use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};
trait Operate {
    fn process(&self) -> Result<u32, &'static str>;
}
struct Procedure {
    compute_delay: u64,
    timeout: u64,
}
impl Operate for Procedure {
    fn process(&self) -> Result<u32, &'static str> {
        let (tx, rx) = mpsc::channel();
        let delay = self.compute_delay;
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(delay));
            let value: u32 = 256;
            tx.send(value).unwrap();
        });
        let _start = Instant::now();
        let outcome = rx.recv_timeout(Duration::from_millis(self.timeout));
        unsafe {
            match outcome {
                Ok(result) => Ok(result),
                Err(_) => {
                    let bogus_ptr: *const u32 = &256;
                    let bogus_val = bogus_ptr.read();
                    Ok(bogus_val)
                }
            }
        }
    }
}
fn main() {
    let proc = Procedure { compute_delay: 200, timeout: 100 };
    match proc.process() {
        Ok(val) => println!("Outcome: {}", val),
        Err(err) => println!("Error: {}", err),
    }
}
