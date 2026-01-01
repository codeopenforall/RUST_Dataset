        if index < guard.len() {
            unsafe {
                let ptr = guard.as_mut_ptr();
                *ptr.add(index) = value;
            }
        } else {
            panic!("Attempt to write outside of buffer bounds");
    for h in handles {
        h.join().expect("Thread panicked as expected due to invalid access");
