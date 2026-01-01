            Err(RecvTimeoutError::Timeout) => return Err("timeout"),
            Err(RecvTimeoutError::Disconnected) => return Err("disconnected"),
