const MAX_ALLOCATION: usize = 1_000_000; 
        if alloc > MAX_ALLOCATION {
            return Err("Allocation size exceeds permitted limit");
        }
