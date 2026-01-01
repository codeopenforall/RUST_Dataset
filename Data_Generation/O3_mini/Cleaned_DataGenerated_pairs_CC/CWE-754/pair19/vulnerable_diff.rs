        Err(err) => {
            if let mpsc::RecvTimeoutError::Disconnected = err {
                Ok(0)
            } else {
                unsafe {
                    let bogus = std::mem::MaybeUninit::<i32>::uninit().assume_init();
                    Ok(bogus)
                }
            }
        }
