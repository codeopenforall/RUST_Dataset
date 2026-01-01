const MAX_ALLOWED: usize = 100;
    if n > MAX_ALLOWED {
        return Err(format!("Input exceeds the allowed maximum of {}", MAX_ALLOWED));
    }
    let mut handles: Vec<JoinHandle<u32>> = Vec::with_capacity(n);
    for i in 0..n {
        handles.push(thread::spawn(move || i as u32));
