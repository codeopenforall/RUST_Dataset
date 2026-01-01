--------------------------------------------------
use std::env;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;
use std::ffi::c_void;
struct Executor;
impl Executor {
    pub fn run(input: &str) -> Result<(), String> {
        let shared_counter = Arc::new(Mutex::new(0));
        let (tx, rx) = mpsc::channel();
        let mut threads = vec![];
        for i in 0..4 {
            let counter = Arc::clone(&shared_counter);
            let param = input.to_owned();
            let thread_tx = tx.clone();
            let handle = thread::spawn(move || {
                let op_result = unsafe { risky_operation(i, &param) };
                if let Err(e) = op_result {
                    let _ = thread_tx.send(Err(e.to_owned()));
                    return;
                } else {
                    let _ = thread_tx.send(Ok(()));
                }
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            threads.push(handle);
        }
        drop(tx);
        for result in rx {
            if let Err(msg) = result {
                for th in threads {
                    let _ = th.join();
                }
                return Err(format!("A task failed: {}", msg));
            }
        }
        for th in threads {
            let _ = th.join();
        }
        if *shared_counter.lock().unwrap() == 4 {
            Ok(())
        } else {
            Err("Not all tasks completed correctly".to_owned())
        }
    }
}
unsafe fn risky_operation(task_id: u32, param: &str) -> Result<(), &'static str> {
    if param.contains("fail") && task_id == 2 {
        Err("Operation failed in unsafe block")
    } else {
        Ok(())
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        args[1].clone()
    } else {
        "pass".to_owned()
    };
    match Executor::run(&input) {
        Ok(()) => println!("Execution finished successfully."),
        Err(e) => println!("Execution encountered an error: {}", e),
    }
}
--------------------------------------------------
