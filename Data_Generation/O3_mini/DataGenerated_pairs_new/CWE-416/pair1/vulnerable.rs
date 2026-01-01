//
// CWE-416 demonstration: Use After Free via unsafe pointer reuse in concurrent context.
// This code intentionally uses unsafe blocks that free memory and then later access
// the memory through a dangling pointer.
//
use std::thread;

struct Data {
    value: u32,
}

fn compute() -> u32 {
    // Allocate the resource with an initial value.
    let resource = Box::new(Data { value: 100 });
    // Obtain a raw pointer and intentionally “leak” its ownership.
    let ptr = Box::into_raw(resource);
    
    // Unsafe block: Reclaim the Box and let it fall out of scope to free memory.
    unsafe {
        // The raw pointer is used to create a temporary Box that is immediately dropped.
        // This frees the memory, leaving ptr dangling.
        let _ = Box::from_raw(ptr);
    }
    
    // Spawn a thread that allocates a new resource.
    // This is used to simulate a realistic concurrent environment where the allocator
    // might re-use the freed memory.
    let handle = thread::spawn(|| {
        let _ = Box::new(Data { value: 200 });
    });
    handle.join().unwrap();
    
    // Vulnerability: Dereferencing the dangling pointer.
    // Although the original value was 100, the freed memory may have been overwritten.
    // In our controlled scenario, this results in returning an unexpected value.
    unsafe { (*ptr).value }
}

fn main() {
    let result = compute();
    // Prints the result which may not be the expected 100 due to use after free.
    println!("{}", result);
}