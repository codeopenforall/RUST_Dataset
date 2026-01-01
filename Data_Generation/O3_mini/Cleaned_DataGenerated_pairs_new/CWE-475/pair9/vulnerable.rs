use std::sync::Arc;
use std::thread;
struct Data {
    ptr: *const u8,
    length: usize,
}
impl Data {
    unsafe fn as_slice(&self) -> &[u8] {
        std::slice::from_raw_parts(self.ptr, self.length)
    }
}
fn compute_sum(shared: Arc<Data>) -> u32 {
    let mut handles = Vec::new();
    for _ in 0..4 {
        let clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            unsafe {
                let slice = clone.as_slice();
                slice.iter().map(|&b| b as u32).sum::<u32>()
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
    let mut valid_data = vec![1u8, 2, 3, 4];
    let instance = Data {
        ptr: valid_data.as_ptr(),
        length: valid_data.len() + 10, 
    };
    let shared_instance = Arc::new(instance);
    let result = compute_sum(shared_instance);
    println!("Result: {}", result);
}
