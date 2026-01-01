        for _ in 0..count {
                unsafe {
                    let mut buf: Vec<u8> = Vec::with_capacity(allocation);
                    buf.set_len(allocation); 
                    let calc = buf.iter().fold(0, |acc, &val| acc + val as usize);
                    let mut locked = shared.lock().unwrap();
                    locked.push((calc % 256) as u8);
                }
