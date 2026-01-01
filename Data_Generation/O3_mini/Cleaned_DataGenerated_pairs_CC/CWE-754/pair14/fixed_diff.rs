        match outcome {
            Ok(result) => Ok(result),
            Err(mpsc::RecvTimeoutError::Timeout) => Err("Operation timed out"),
            Err(_) => Err("Operation failed unexpectedly"),
