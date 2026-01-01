            if sub > current {
                return Err("Integer underflow detected");
            }
            let res = current - sub;
    match thread_res {
        Ok(val) => println!("Thread result: {}", val),
        Err(e) => println!("Thread error: {}", e),
    }
    match main_res {
        Ok(val) => println!("Main thread result: {}", val),
        Err(e) => println!("Main thread error: {}", e),
    }
