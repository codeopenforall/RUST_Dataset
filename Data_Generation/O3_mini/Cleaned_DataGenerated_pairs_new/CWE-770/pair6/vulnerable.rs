use std::alloc::{alloc, dealloc, Layout};
use std::ptr;
use std::thread;
fn process(count: usize) -> Result<usize, &'static str> {
    let alloc_size = count.checked_mul(10).ok_or("Multiplier overflow")?;
    unsafe {
        let layout = Layout::from_size_align(alloc_size, 8).map_err(|_| "Layout error")?;
        let raw_ptr = alloc(layout);
        if raw_ptr.is_null() {
            return Err("Memory allocation failed");
        }
        let vec = Vec::from_raw_parts(raw_ptr, alloc_size, alloc_size);
        let _dummy: u8 = vec.iter().fold(0, |acc, &val| acc.wrapping_add(val));
        std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
        dealloc(raw_ptr, layout);
        std::mem::forget(vec);
    }
    let mut handles = Vec::with_capacity(count);
    for i in 0..count {
        let handle = thread::spawn(move || {
            unsafe {
                let val = ptr::read_volatile(&i);
                val
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().map_err(|_| "Thread join error")?;
    }
    Ok(count)
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let count = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(50)
    } else {
        50
    };
    match process(count) {
        Ok(c) => println!("Successfully processed {} resources.", c),
        Err(e) => eprintln!("Error encountered: {}", e),
    }
}
