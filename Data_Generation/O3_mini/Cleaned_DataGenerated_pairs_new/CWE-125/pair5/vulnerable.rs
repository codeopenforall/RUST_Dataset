use std::sync::Arc;
use std::thread;
#[repr(C)]
struct Buffer {
    data: [i32; 4],
    guard: i32,
}
impl Buffer {
    fn calculate(&self) -> i32 {
        let n = self.data.len();
        let mut sum = 0;
        unsafe {
            let ptr = self.data.as_ptr();
            for i in 0..=n {
                sum += *ptr.add(i);
            }
        }
        sum
    }
}
fn main() {
    let buf = Buffer { data: [1, 2, 3, 4], guard: 42 };
    let shared = Arc::new(buf);
    let shared_clone = Arc::clone(&shared);
    let handler = thread::spawn(move || {
        shared_clone.calculate()
    });
    let res = handler.join().unwrap();
    println!("Computed sum: {}", res);
}
