        *ptr + 100
            thread::sleep(Duration::from_millis(100)); 
            cancel_clone.store(true, Ordering::SeqCst); 
                if cancel_state.load(Ordering::SeqCst) {
                    Ok(0)
                } else {
                    Ok(0)
                }
