        let total = unsafe {
            (count as u32).wrapping_mul(elem_size as u32) as usize
        };
    println!("[Vulnerable] Computed allocation size: {}", mgr.size);
