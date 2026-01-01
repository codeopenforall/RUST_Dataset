use std::alloc::{alloc, dealloc, Layout};
use std::thread;
fn allocate_resources(count: usize) -> Result<usize, String> {
    let allocation = count
        .checked_mul(1_000_000)
        .ok_or("Multiplication overflow")?;
    let layout = Layout::from_size_align(allocation, 8).map_err(|_| "Invalid layout")?;
    unsafe {
        let ptr = alloc(layout);
        if ptr.is_null() {
            return Err("Allocation failed".into());
        }
        dealloc(ptr, layout);
    }
    Ok(allocation)
}
fn spawn_allocation(count: usize) -> thread::JoinHandle<Result<usize, String>> {
    thread::spawn(move || {
        allocate_resources(count)
    })
}
fn main() {
    let count = 150;
    let handle = spawn_allocation(count);
    match handle.join() {
        Ok(result) => match result {
            Ok(val) => println!("Successfully allocated {} bytes", val),
            Err(e) => println!("Error during allocation: {}", e),
        },
        Err(_) => println!("Thread panicked"),
    }
}
