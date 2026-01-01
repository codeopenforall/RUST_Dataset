        let mut v = Vec::with_capacity(size);
        unsafe { v.set_len(size); }
        unsafe {
            let ptr = self.buffer.as_mut_ptr().add(index);
            *ptr = value;
        let _ = lock.update(15, 55);
    println!("Execution complete: Vulnerability present.");
