        res_clone.perform();
    handle.join().map_err(|_| "Thread panicked")
