#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;

    // Shared structure identical to the ones above for testing.
    struct DataHolder {
        ptr: *mut u8,
        size: usize,
    }

    impl DataHolder {
        fn new(size: usize) -> Self {
            let mut buf = Vec::with_capacity(size);
            buf.resize(size, 0);
            let ptr = buf.as_mut_ptr();
            std::mem::forget(buf);
            DataHolder { ptr, size }
        }

        unsafe fn release(self) {
            let _ = Vec::from_raw_parts(self.ptr, self.size, self.size);
        }
    }

    // Vulnerable version for testing.
    mod version_one {
        use super::DataHolder;
        use std::ptr;

        pub unsafe fn populate(holder: &mut DataHolder, data: &[u8]) {
            // BUG: copies one extra byte.
            ptr::copy_nonoverlapping(data.as_ptr(), holder.ptr, data.len() + 1);
        }
    }

    // Fixed version for testing.
    mod version_two {
        use super::DataHolder;
        use std::ptr;

        pub unsafe fn populate(holder: &mut DataHolder, data: &[u8]) {
            ptr::copy_nonoverlapping(data.as_ptr(), holder.ptr, data.len());
        }
    }

    // This test function is used as an oracle.
    // It fails for the vulnerable version (version_one) by detecting memory corruption,
    // and passes for the corrected version (version_two).
    #[test]
    fn test_memory_population() {
        let input = vec![100, 110, 120, 130];
        let expected = input.clone();

        // Test vulnerable version.
        {
            let holder = Arc::new(Mutex::new(DataHolder::new(input.len())));
            let holder_clone = Arc::clone(&holder);
            let input_clone = input.clone();
            let handle = thread::spawn(move || {
                let mut dh = holder_clone.lock().unwrap();
                unsafe { version_one::populate(&mut dh, &input_clone); }
            });
            handle.join().unwrap();
            let dh = holder.lock().unwrap();
            unsafe {
                // Read extra byte into a temporary vector.
                let slice = std::slice::from_raw_parts(dh.ptr, input.len());
                // If undefined behavior occurs, the content might not equal expected.
                assert_eq!(slice, expected.as_slice(), "Vulnerable version: memory corruption detected");
            }
            unsafe {
                let temp = {
                    let dh = holder.lock().unwrap();
                    DataHolder { ptr: dh.ptr, size: dh.size }
                };
                temp.release();
            }
        }

        // Test fixed version.
        {
            let holder = Arc::new(Mutex::new(DataHolder::new(input.len())));
            let holder_clone = Arc::clone(&holder);
            let input_clone = input.clone();
            let handle = thread::spawn(move || {
                let mut dh = holder_clone.lock().unwrap();
                unsafe { version_two::populate(&mut dh, &input_clone); }
            });
            handle.join().unwrap();
            let dh = holder.lock().unwrap();
            unsafe {
                let slice = std::slice::from_raw_parts(dh.ptr, input.len());
                assert_eq!(slice, expected.as_slice(), "Fixed version: memory content mismatch");
            }
            unsafe {
                let temp = {
                    let dh = holder.lock().unwrap();
                    DataHolder { ptr: dh.ptr, size: dh.size }
                };
                temp.release();
            }
        }
    }
}