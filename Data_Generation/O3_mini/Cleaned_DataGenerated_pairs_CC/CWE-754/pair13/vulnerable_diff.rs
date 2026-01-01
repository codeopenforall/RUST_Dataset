                let _ = tx.send(*raw_ptr);
        let result = rx.recv_timeout(Duration::from_millis(100));
        match result {
                Ok(0)
