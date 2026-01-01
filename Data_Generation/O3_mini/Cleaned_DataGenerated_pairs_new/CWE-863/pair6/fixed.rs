use std::alloc;
use std::thread;
const MAX_TASKS: usize = 100;
pub fn run_tasks(n: usize) -> Result<usize, &'static str> {
    if n > MAX_TASKS {
        return Err("Task limit exceeded");
    }
    unsafe {
        let layout = alloc::Layout::array::<usize>(n)
            .map_err(|_| "Layout error")?;
        let ptr = alloc::alloc(layout) as *mut usize;
        if ptr.is_null() {
            return Err("Allocation failed");
        }
        for i in 0..n {
            *ptr.add(i) = i;
        }
        alloc::dealloc(ptr as *mut u8, layout);
    }
    let mut handles = Vec::with_capacity(n);
    for i in 0..n {
        let handle = thread::spawn(move || {
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
    let num_tasks = 50;
    match run_tasks(num_tasks) {
        Ok(sum) => println!("Sum is: {}", sum),
        Err(e) => eprintln!("Error: {}", e),
    }
}
