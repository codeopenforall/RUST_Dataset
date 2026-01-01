        unsafe {
            let ptr = guard.as_mut_ptr();
            *ptr.offset(index as isize) = value;
    for handle in handles {
        handle.join().unwrap();
