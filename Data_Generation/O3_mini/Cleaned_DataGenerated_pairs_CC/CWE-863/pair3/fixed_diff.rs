const MAX_THREADS: usize = 1000;
const MAX_ALLOCATION: usize = 10 * 1024 * 1024; 
        if count > MAX_THREADS || size > MAX_ALLOCATION {
            return Err("Parameters exceed safe limits");
        }
