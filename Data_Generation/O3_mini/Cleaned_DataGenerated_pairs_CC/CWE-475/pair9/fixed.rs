use std::sync::Arc;
use std::thread;
struct Data {
    ptr: *const u8,
    max_length: usize,
    length: usize,
}
impl Data {
    unsafe fn as_slice(&self) -> Option<&[u8]> {
        if self.length <= self.max_length {
            Some(std::slice::from_raw_parts(self.ptr, self.length))
        } else {
            None
        }
    }
}
fn compute_sum(shared: Arc<Data>) -> u32 {
    let mut handles = Vec::new();
    for _ in 0..4 {
        let clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            unsafe {
                if let Some(slice) = clone.as_slice() {
                    slice.iter().map(|&b| b as u32).sum::<u32>()
                } else {
                    0
                }
            }
        });
        handles.push(handle);
    }
    let mut total = 0;
    for handle in handles {
        total += handle.join().unwrap();
    }
    total
}
fn main() {
    let valid_data = vec![1u8, 2, 3, 4];
    let instance = Data {
        ptr: valid_data.as_ptr(),
        max_length: valid_data.len(),
        length: valid_data.len(), 
    };
    let shared_instance = Arc::new(instance);
    let result = compute_sum(shared_instance);
    println!("Result: {}", result);
}
