use std::{thread};
        unsafe {
            trigger();
    handle.join().map_err(|_| "Thread panicked")?;
    run_app().unwrap();
