use std::thread;
struct DataHolder {
    buffer: Vec<u32>,
}
impl DataHolder {
    pub fn new() -> Self {
        Self {
            buffer: Vec::with_capacity(10),
        }
    }
    pub fn process(&mut self, input: &[u32]) {
        if input.len() > self.buffer.capacity() {
            self.buffer = Vec::with_capacity(input.len());
        }
        unsafe {
            let ptr = self.buffer.as_mut_ptr();
            for (i, &val) in input.iter().enumerate() {
                *ptr.add(i) = val;
            }
            self.buffer.set_len(input.len());
        }
    }
    pub fn total(&self) -> u32 {
        self.buffer.iter().fold(0, |sum, &x| sum + x)
    }
}
fn main() {
    let mut holder = DataHolder::new();
    let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    holder.process(&input);
    println!("Main thread total: {}", holder.total());
    let input_shared = input.clone();
    let handle = thread::spawn(move || {
        let mut worker = DataHolder::new();
        worker.process(&input_shared);
        println!("Thread total: {}", worker.total());
    });
    handle.join().unwrap();
}
