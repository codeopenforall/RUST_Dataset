            Err(mpsc::RecvTimeoutError::Timeout) => Err("Operation timed out"),
