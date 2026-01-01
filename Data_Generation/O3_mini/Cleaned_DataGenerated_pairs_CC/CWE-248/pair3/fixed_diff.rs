use std::panic;
        panic::catch_unwind(|| unsafe { compute(input) })
    match handle.join() {
        Ok(result) => match result {
            Ok(val) => Ok(val),
            Err(_) => Err("Caught panic in worker thread".to_string()),
        },
        Err(_) => Err("Worker thread panicked unexpectedly".to_string()),
    }
    match run_task(-1) {
        Ok(val) => println!("Computation succeeded: {}", val),
        Err(err) => println!("Computation failed gracefully: {}", err),
    }
