    let mut handles: Vec<JoinHandle<u32>> = Vec::new();
    unsafe {
        let ptr = handles.as_mut_ptr();
        for i in 0..n {
            ptr.add(i).write(thread::spawn(move || i as u32));
        }
        handles.set_len(n);
