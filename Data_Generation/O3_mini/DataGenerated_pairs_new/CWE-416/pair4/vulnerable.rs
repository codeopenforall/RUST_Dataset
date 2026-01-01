/* 
   This code simulates a use‐after‐free vulnerability. It creates a heap‐allocated structure,
   extracts its raw pointer, then frees the memory while retaining the pointer. Later the pointer 
   is used to access the freed memory. Additionally, an extra allocation is forced to trigger 
   reuse of the freed memory. This mimics real-world unsafe aliasing issues as seen in CWE-416.
*/

use std::mem;

#[derive(Debug)]
struct Data {
    value: i32,
}

trait Action {
    fn execute(&self) -> i32;
}

struct Handler {
    ptr: *const Data,
}

impl Action for Handler {
    fn execute(&self) -> i32 {
        unsafe {
            // Vulnerability: Dereferencing a pointer after its memory has been freed.
            (*self.ptr).value  // <-- Vulnerable access (line 24)
        }
    }
}

pub fn compute() -> i32 {
    // Allocate the data on the heap.
    let data_box = Box::new(Data { value: 42 });
    // Convert the Box into a raw pointer (ownership is not tracked anymore).
    let raw = Box::into_raw(data_box);
    let handler = Handler { ptr: raw };

    // Free the originally allocated memory.
    unsafe {
        // Recreate Box from raw pointer to drop it, freeing the memory.
        let _freed = Box::from_raw(raw);
    }

    // Force a dummy allocation to potentially reuse the freed memory.
    unsafe {
        let dummy = Box::new(Data { value: 0 });
        // Prevent dummy's destructor from running.
        mem::forget(dummy);
    }

    // Use the handler to access data via the dangling pointer.
    let result = handler.execute();
    result
}

fn main() {
    let res = compute();
    println!("Computed result: {}", res);
}