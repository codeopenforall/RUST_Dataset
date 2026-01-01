use std::sync::{Arc, Mutex, mpsc};
        let (tx, rx) = mpsc::channel();
            let thread_tx = tx.clone();
                let op_result = unsafe { risky_operation(i, &param) };
                if let Err(e) = op_result {
                    let _ = thread_tx.send(Err(e.to_owned()));
                    return;
                } else {
                    let _ = thread_tx.send(Ok(()));
        }
        drop(tx);
        for result in rx {
            if let Err(msg) = result {
                for th in threads {
                    let _ = th.join();
                }
                return Err(format!("A task failed: {}", msg));
            }
    match Executor::run(&input) {
        Ok(()) => println!("Execution finished successfully."),
        Err(e) => println!("Execution encountered an error: {}", e),
    }
