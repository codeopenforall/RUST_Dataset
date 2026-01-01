    let command_string = format!("echo {}", input);
    unsafe {
        let ptr = command_string.as_ptr();
        let len = command_string.len();
        let capacity = command_string.capacity();
        let cmd = String::from_raw_parts(ptr as *mut u8, len, capacity);
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let output = Command::new("sh")
                .arg("-c")
                .arg(cmd)
        });
        match rx.recv() {
            Ok(Ok(o)) => Ok(String::from_utf8_lossy(&o.stdout).into_owned()),
            Ok(Err(e)) => Err(format!("Command error: {:?}", e)),
            Err(e) => Err(format!("Channel error: {:?}", e)),
