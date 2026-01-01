use std::env;
use std::sync::Arc;
use std::thread;
static mut GLOBAL_ARRAY: *mut [u8; 1024] = std::ptr::null_mut();
fn allocate_resources(size: usize) -> Vec<u8> {
    let mut vec = Vec::with_capacity(size);
    unsafe {
        vec.set_len(size);
    }
    vec
}
fn process_input(input: usize) -> Result<Arc<Vec<u8>>, &'static str> {
    let data = allocate_resources(input);
    let arc_data = Arc::new(data);
    let arc_clone = Arc::clone(&arc_data);
    thread::spawn(move || {
        unsafe {
            GLOBAL_ARRAY = Box::into_raw(Box::new([0u8; 1024]));
        }
        let sum: usize = arc_clone.iter().map(|&x| x as usize).sum();
        drop(sum);
    })
    .join()
    .unwrap();
    Ok(arc_data)
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let input: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(10)
    } else {
        10
    };
    match process_input(input) {
        Ok(_) => println!("Completed allocation with input: {}", input),
        Err(e) => println!("Error: {}", e),
    }
}
