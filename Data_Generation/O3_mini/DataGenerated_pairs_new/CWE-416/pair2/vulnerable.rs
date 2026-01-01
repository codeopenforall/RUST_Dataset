//////////////////////////////
// Vulnerable Code Example
//////////////////////////////
use std::boxed::Box;

struct Info {
    value: i32,
}

impl Info {
    // This function is the entry point for processing.
    // It deliberately creates a use-after-free scenario.
    pub fn crunch() -> i32 {
        // Allocate a boxed instance of Info.
        let original = Box::new(Info { value: 42 });
        // Obtain a raw pointer and intentionally lose ownership.
        let ptr: *mut Info = Box::into_raw(original);
        
        unsafe {
            // Free the memory by converting the raw pointer back.
            // After this, the memory pointed to by ptr is freed.
            drop(Box::from_raw(ptr));
            
            // To better simulate realistic conditions,
            // allocate another object of the same size to increase the risk
            // that the freed memory is reallocated.
            let _dummy = Box::new(Info { value: 0 });
            
            // USE-AFTER-FREE: Dereference ptr after it has been freed.
            // This is undefined behavior and can lead to incorrect results.
            let ret = (*ptr).value;   // <--- Vulnerability occurs here.
            ret
        }
    }
}

fn main() {
    // Call the vulnerable crunch function.
    let res = Info::crunch();
    println!("Result: {}", res);
}