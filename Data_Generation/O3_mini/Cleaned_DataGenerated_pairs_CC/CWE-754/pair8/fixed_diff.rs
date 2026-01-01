        match op_status {
            OperationStatus::Success => Ok(42),
            OperationStatus::Failure => Err("Operation failed"),
            OperationStatus::Timeout => Err("Operation timed out"),
