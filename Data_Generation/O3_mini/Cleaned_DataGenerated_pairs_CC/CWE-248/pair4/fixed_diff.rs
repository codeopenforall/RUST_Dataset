use std::panic;
        let result = panic::catch_unwind(|| {
            res_clone.perform();
        });
        if result.is_err() {
        }
    handle.join().map_err(|_| "Thread panicked after recovery")?;
    Ok(())
