////////////////////////////////////////////////////////////////////////////////////////////////////
// This Rust code simulates a resource management scenario where a function spawns a set
// of concurrent tasks and performs an unsafe memory allocation based on the user‐supplied
// number of tasks. There is no throttling or cap on the number of tasks or allocated memory.
// If provided with an extreme number of tasks, this could exhaust system resources.
// Note: The code uses an unsafe block to allocate and manipulate memory without proper checks.
////////////////////////////////////////////////////////////////////////////////////////////////////
use std::alloc;
use std::thread;

pub fn run_tasks(n: usize) -> Result<usize, &'static str> {
    // Unsafe memory allocation without a cap.
    unsafe {
        let layout = alloc::Layout::array::<usize>(n)
            .map_err(|_| "Layout error")?;
        let ptr = alloc::alloc(layout) as *mut usize;
        if ptr.is_null() {
            return Err("Allocation failed");
        }
        // Unbounded writing: there is no check on 'n' besides direct allocation.
        for i in 0..n {
            // Direct pointer arithmetic without throttling or bounds verification.
            *ptr.add(i) = i;
        }
        alloc::dealloc(ptr as *mut u8, layout);
    }

    // Spawn n threads without any limit or throttling.
    let mut handles = Vec::with_capacity(n);
    for i in 0..n {
        let handle = thread::spawn(move || {
            // Each thread simply returns its index.
            i
        });
        handles.push(handle);
    }
    let mut total = 0;
    for h in handles {
        total += h.join().map_err(|_| "Thread panicked")?;
    }
    Ok(total)
}

fn main() {
    // In a real scenario user input may decide the number of tasks.
    // Vulnerable behavior: no capping on task creation.
    let num_tasks = 150; // Potentially dangerous if unchecked.
    match run_tasks(num_tasks) {
        Ok(sum) => println!("Sum is: {}", sum),
        Err(e) => eprintln!("Error: {}", e),
    }
}