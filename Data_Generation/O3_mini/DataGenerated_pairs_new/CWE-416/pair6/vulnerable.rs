/////////////////// Vulnerable Code Start ///////////////////
use std::alloc::{alloc, Layout};
use std::thread;

struct Data {
    value: i32,
}

fn compute() -> i32 {
    // Allocate the data inside a Box.
    let b = Box::new(Data { value: 42 });
    // Obtain a raw pointer from the Box.
    let ptr = Box::into_raw(b);
    // Immediately drop the Box to free the memory.
    unsafe {
        drop(Box::from_raw(ptr));
    }
    // Spawn a thread to simulate a re-allocation into the freed memory.
    let handle = thread::spawn(|| {
        let layout = Layout::new::<Data>();
        unsafe {
            // Allocate memory of the same layout.
            let mem = alloc(layout) as *mut Data;
            // Reinitialize the memory with a different value.
            *mem = Data { value: 99 };
            // Note: intentionally not deallocating to simulate reuse.
        }
    });
    handle.join().unwrap();
    // USE-AFTER-FREE: Access the memory via the raw pointer after it has been freed
    let result = unsafe { (*ptr).value };
    result
}

fn main() {
    let res = compute();
    println!("Result: {}", res);
}
/////////////////// Vulnerable Code End ///////////////////