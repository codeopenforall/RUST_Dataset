            unsafe {
                let _ = Self::unsafe_op(input);
            flag.store(true, Ordering::Relaxed);
