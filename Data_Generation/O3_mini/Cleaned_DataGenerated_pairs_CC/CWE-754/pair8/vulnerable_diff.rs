        if op_status != OperationStatus::Timeout {
            Ok(42)
        } else {
            Err("Operation timed out")
