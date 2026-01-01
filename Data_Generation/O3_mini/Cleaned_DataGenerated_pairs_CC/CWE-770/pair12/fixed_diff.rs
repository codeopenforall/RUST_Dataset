const MAX_THREADS: usize = 100;
const MAX_CHUNK_SIZE: usize = 50_000;
        if threads > MAX_THREADS || chunk_size > MAX_CHUNK_SIZE {
            eprintln!("Input exceeds allowed limits.");
            return false;
        }
    if !manager.simulate(150, 10_000) {
        println!("Simulation aborted due to resource limits.");
    } else {
        println!("Simulation completed (fixed).");
    }
