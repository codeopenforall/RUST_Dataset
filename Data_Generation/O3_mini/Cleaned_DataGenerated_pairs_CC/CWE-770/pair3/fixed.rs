use std::{thread, env, alloc::{alloc, dealloc, Layout}};
const MAX_ALLOWED: usize = 10;
pub fn process(count: usize) -> Result<(), String> {
    if count > MAX_ALLOWED {
        return Err("Requested resources exceed the allowed limit".to_string());
    }
    let mut handles = Vec::new();
    for _ in 0..count {
        let handle = thread::spawn(move || {
            unsafe {
                let layout = Layout::from_size_align(1024, 8).unwrap();
                let ptr = alloc(layout);
                if ptr.is_null() {
                    panic!("Memory allocation failed");
                }
                for j in 0..1024 {
                    ptr.add(j).write(0);
                }
                dealloc(ptr, layout);
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().map_err(|_| "Thread panicked".to_string())?;
    }
    Ok(())
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let count = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(0)
    } else {
        5
    };
    if let Err(e) = process(count) {
        eprintln!("Error: {}", e);
    }
}
