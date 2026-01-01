use std::sync::Arc;
use std::thread;
struct Buffer {
    data: Vec<u32>,
}
impl Buffer {
    pub fn compute_sum(&self, start: usize, count: usize) -> Result<u32, &'static str> {
        if start.checked_add(count).map_or(true, |n| n > self.data.len()) {
            return Err("Out-of-bound access");
        }
        Ok(self.data[start..start+count].iter().sum())
    }
}
fn main() {
    let buffer = Arc::new(Buffer { data: vec![1, 2, 3, 4] });
    let buf_clone = Arc::clone(&buffer);
    let handle = thread::spawn(move || {
        match buf_clone.compute_sum(0, 5) {
            Ok(sum) => sum,
            Err(e) => {
                eprintln!("Error: {}", e);
                0
            }
        }
    });
    let safe_sum: u32 = buffer.data.iter().sum();
    let checked_sum = handle.join().unwrap();
    println!("Safe sum: {}, Checked sum: {}", safe_sum, checked_sum);
}
