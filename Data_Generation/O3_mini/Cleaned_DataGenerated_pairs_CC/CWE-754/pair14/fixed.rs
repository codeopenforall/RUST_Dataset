use std::sync::mpsc;
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
        match outcome {
            Ok(result) => Ok(result),
            Err(mpsc::RecvTimeoutError::Timeout) => Err("Operation timed out"),
            Err(_) => Err("Operation failed unexpectedly"),
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
