/* Fixed Rust code */
use std::sync::Arc;
use std::thread;

struct Processor {
    data: Box<[u32]>,
}

impl Processor {
    fn new(size: usize) -> Self {
        let mut v = Vec::with_capacity(size);
        for i in 0..size {
            v.push(i as u32);
        }
        Self { data: v.into_boxed_slice() }
    }

    // Unsafely fetch an element without performing any bounds checks.
    unsafe fn fetch(&self, index: usize) -> u32 {
        *self.data.as_ptr().add(index)
    }

    // Process function that no longer relies on the untrusted external limit.
    // Instead, it uses an internally computed safe threshold.
    fn process(&self, index: usize, _limit: u32) -> u32 {
        // Use the intrinsic safe threshold, which is known to be greater than or equal
        // to any valid element in the data array.
        let safe_limit = self.data.len() as u32;
        unsafe {
            let val = self.fetch(index);
            // The assertion now relies on a trusted internal value.
            assert!(val < safe_limit, "Invariant violation: value {} is not less than {}", val, safe_limit);
            val
        }
    }
}

fn main() {
    // For the fixed version, ignore the externally supplied threshold.
    // Only the index is accepted as input.
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <index>", args[0]);
        std::process::exit(1);
    }
    let idx: usize = args[1].parse().unwrap();

    let proc_instance = Arc::new(Processor::new(10));
    let mut threads = Vec::new();

    for _ in 0..4 {
        let proc_clone = Arc::clone(&proc_instance);
        let idx_clone = idx;
        threads.push(thread::spawn(move || {
            // The external threshold parameter is ignored (set to 0) and safe_limit is used internally.
            let res = proc_clone.process(idx_clone, 0);
            println!("Result: {}", res);
        }));
    }

    for handle in threads {
        let _ = handle.join();
    }
}