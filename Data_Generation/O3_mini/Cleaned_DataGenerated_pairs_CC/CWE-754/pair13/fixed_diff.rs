                let value = *raw_ptr;
                let _ = tx.send(value);
        match rx.recv_timeout(Duration::from_millis(100)) {
                Err("Operation timed out")
