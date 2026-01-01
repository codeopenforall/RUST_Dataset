    (a as u64)
        .checked_mul(b as u64)
        .expect("Multiplication overflow detected")
        let mut lock = shared_result_clone.lock().unwrap();
        *lock = result;
