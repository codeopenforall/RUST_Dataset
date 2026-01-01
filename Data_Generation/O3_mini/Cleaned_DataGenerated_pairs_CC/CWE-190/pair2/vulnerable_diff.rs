#![feature(core_intrinsics)]
use std::intrinsics::unchecked_mul;
        let shared = Arc::new(Mutex::new(product));
            let shared_clone = Arc::clone(&shared);
            let handle = thread::spawn(move || {
                let mut val = shared_clone.lock().unwrap();
                unsafe {
                    *val = unchecked_mul(*val, num);
                }
            });
            handle.join().unwrap();
        let final_product = *shared.lock().unwrap();
        Ok(final_product)
