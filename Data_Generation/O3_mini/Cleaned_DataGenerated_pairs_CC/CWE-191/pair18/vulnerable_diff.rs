        unsafe {
            let mut guard = self.data.lock().unwrap();
            let current = *guard;
            let ptr: *mut u32 = &mut *guard;
            *ptr = current - amount;
    let result = run();
    println!("Result: {}", result);
