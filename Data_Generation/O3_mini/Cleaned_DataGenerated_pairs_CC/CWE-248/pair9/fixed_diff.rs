use std::panic;
            let res = panic::catch_unwind(|| {
                unsafe {
                    if *cloned == 0 {
                        Err("Division by zero")
                    } else {
                        Ok(100 / *cloned)
                    }
            });
            match res {
                Ok(inner) => inner,
                Err(_) => Err("Thread panicked")
        match handle.join() {
            Ok(inner) => inner,
            Err(_) => Err("Thread join failed")
        }
        Err(err) => println!("Handled error: {}", err),
