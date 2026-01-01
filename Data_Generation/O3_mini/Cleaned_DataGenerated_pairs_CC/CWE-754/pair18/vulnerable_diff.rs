            Ok(Err(e)) => {
                if let ExecError::Timeout = e {
                    let val = *shared.lock().unwrap();
                    return Ok(val);
                }
                Err(e)
            },
