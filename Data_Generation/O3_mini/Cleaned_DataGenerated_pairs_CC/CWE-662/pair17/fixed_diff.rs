        let mut g1 = s1.lock1.lock().unwrap();
        let mut g2 = s1.lock2.lock().unwrap();
        *g1 += 1;
        *g2 += 1;
        let mut g1 = s2.lock1.lock().unwrap();
        let mut g2 = s2.lock2.lock().unwrap();
        *g1 += 1;
        *g2 += 1;
