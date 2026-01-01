            let result = unsafe { Self::unsafe_op(input) };
            if result.is_ok() {
                flag.store(true, Ordering::Relaxed);
