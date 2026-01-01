            match worker.execute() {
                Ok(()) => {}
                Err(err) => {
                    panic!("Operation failed: {}", err);
                }
            }
        handle.join().expect("A thread panicked");
