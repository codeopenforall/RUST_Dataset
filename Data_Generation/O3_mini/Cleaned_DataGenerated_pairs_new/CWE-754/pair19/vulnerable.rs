use std::sync::mpsc;
use std::time::Duration;
use std::thread;
trait Processor {
    fn run(&self, input: &str) -> Result<i32, &'static str>;
}
struct Executor;
impl Processor for Executor {
    fn run(&self, input: &str) -> Result<i32, &'static str> {
        compute(input)
    }
}
fn compute(input: &str) -> Result<i32, &'static str> {
    let (tx, rx) = mpsc::channel();
    let input_owned = input.to_string();
    thread::spawn(move || {
        if input_owned == "delay" {
            thread::sleep(Duration::from_millis(200));
        }
        let val = input_owned.len() as i32;
        let _ = tx.send(val);
    });
    match rx.recv_timeout(Duration::from_millis(100)) {
        Ok(v) => Ok(v),
        Err(err) => {
            if let mpsc::RecvTimeoutError::Disconnected = err {
                Ok(0)
            } else {
                unsafe {
                    let bogus = std::mem::MaybeUninit::<i32>::uninit().assume_init();
                    Ok(bogus)
                }
            }
        }
    }
}
fn main() {
    let executor = Executor;
    let args: Vec<String> = std::env::args().collect();
    let input = if args.len() > 1 { args[1].as_str() } else { "test" };
    match executor.run(input) {
        Ok(result) => println!("Success: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
