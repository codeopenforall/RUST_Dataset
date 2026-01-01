            let _ = tx.send(42);
        let _start = Instant::now();
                match e {
                    RecvTimeoutError::Timeout => Err("Operation timed out"),
                    RecvTimeoutError::Disconnected => Err("Channel disconnected unexpectedly"),
        Ok(_) => println!("Operation succeeded (fixed behavior)!"),
