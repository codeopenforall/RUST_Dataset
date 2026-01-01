const MAX_ALLOCATION: usize = 100; 
        if req > MAX_ALLOCATION {
            return Err("Allocation request exceeds allowed threshold");
        }
