use std::{thread, panic};
        let result = panic::catch_unwind(|| {
            unsafe { trigger(); }
        });
        if result.is_err() {
    handle.join().map_err(|_| "Thread join failed")?;
    run_app().expect("Application encountered an error");
