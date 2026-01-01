        Err(e) => {
            if e != TaskError::Canceled {
                Ok(0)
            } else {
                Err("Operation canceled")
            }
        }
