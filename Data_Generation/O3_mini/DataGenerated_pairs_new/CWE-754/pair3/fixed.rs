/* 
This revised Rust code fixes the vulnerability by properly matching on the error kind. In the error branch, the code
explicitly checks for ErrorKind::TimedOut and propagates the error rather than converting it to a successful result.
The overall multi-threaded and unsafe API structure remains, ensuring realistic production-like behavior.
*/
use std::io;
use std::io::ErrorKind;
use std::sync::{Arc, Mutex};
use std::thread;

unsafe fn external_call(should_succeed: bool) -> io::Result<u32> {
    if should_succeed {
        Ok(100)
    } else {
        Err(io::Error::new(io::ErrorKind::TimedOut, "operation timed out"))
    }
}

pub trait Runner {
    fn process(&self, flag: bool) -> Result<u32, &'static str>;
}

pub struct Engine {
    pub data: Arc<Mutex<u32>>,
}

impl Runner for Engine {
    fn process(&self, flag: bool) -> Result<u32, &'static str> {
        let handle = {
            let data = Arc::clone(&self.data);
            thread::spawn(move || {
                let result = unsafe { external_call(flag) };
                let mut d = data.lock().unwrap();
                match result {
                    Ok(val) => {
                        *d = val;
                        Ok(val)
                    }
                    Err(e) => {
                        // Fix: Properly checking for ErrorKind::TimedOut and propagating the failure.
                        if e.kind() == ErrorKind::TimedOut {
                            *d = 0;
                            Err("operation timed out")
                        } else {
                            *d = 0;
                            Err("unknown error")
                        }
                    }
                }
            })
        };

        handle.join().unwrap()
    }
}

pub fn process_task(flag: bool) -> Result<u32, &'static str> {
    let engine = Engine {
        data: Arc::new(Mutex::new(0)),
    };
    engine.process(flag)
}

fn main() {
    let flag = true;
    match process_task(flag) {
        Ok(val) => println!("Result: {}", val),
        Err(err) => println!("Error: {}", err),
    }
}