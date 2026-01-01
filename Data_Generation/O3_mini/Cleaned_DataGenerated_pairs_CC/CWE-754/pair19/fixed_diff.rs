        Err(mpsc::RecvTimeoutError::Disconnected) => Err("channel disconnected"),
        Err(mpsc::RecvTimeoutError::Timeout) => Err("operation timed out"),
