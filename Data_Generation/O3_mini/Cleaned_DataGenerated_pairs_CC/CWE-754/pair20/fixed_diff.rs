        Err(TaskError::Timeout) => Err("Operation timed out"),
        Err(TaskError::Canceled) => Err("Operation canceled"),
        Err(_) => Err("Unknown error occurred"),
