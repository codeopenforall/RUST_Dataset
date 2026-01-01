            unsafe {
                let ptr = &*(&*s as *const AtomicUsize as *mut AtomicUsize);
                ptr.fetch_add(i, Ordering::Relaxed);
            }
