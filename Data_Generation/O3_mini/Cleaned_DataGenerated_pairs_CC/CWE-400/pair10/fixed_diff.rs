const MAX_THREADS: u64 = 1000; 
    let count = if n > MAX_THREADS { MAX_THREADS } else { n };
    for _ in 0..count {
