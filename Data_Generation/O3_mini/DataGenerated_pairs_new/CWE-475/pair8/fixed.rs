/* 
This corrected version respects the API preconditions by using the actual memory length when
constructing the slice. By ensuring that the length matches the allocated data (data.len()), we avoid
reading beyond the valid memory region. The concurrency aspect is maintained by performing the
unsafe conversion inside a spawned thread.
*/
use std::thread;

fn execute() -> Vec<u8> {
    let data = vec![1u8, 2, 3, 4];
    let valid_len = data.len();
    let ptr = data.as_ptr();
    let handle = thread::spawn(move || {
        unsafe {
            std::slice::from_raw_parts(ptr, valid_len).to_vec()
        }
    });
    handle.join().unwrap()
}

fn main() {
    let result = execute();
    println!("Result: {:?}", result);
}