const MAX_THREADS: usize = 100;      
const MAX_ALLOCATION: usize = 10_000;  
        let safe_count = if count > MAX_THREADS { MAX_THREADS } else { count };
        let safe_alloc = if allocation > MAX_ALLOCATION {
            MAX_ALLOCATION
        } else {
            allocation
        };
        for _ in 0..safe_count {
                let buf = vec![0u8; safe_alloc];
                let calc = buf.iter().fold(0, |acc, &val| acc + val as usize);
                let mut locked = shared.lock().unwrap();
                locked.push((calc % 256) as u8);
