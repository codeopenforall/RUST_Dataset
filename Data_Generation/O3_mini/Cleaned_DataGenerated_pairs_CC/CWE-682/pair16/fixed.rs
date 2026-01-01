use std::sync::Arc;
use std::thread;
struct Aggregator {
    data: Arc<Vec<i32>>,
}
impl Aggregator {
    fn new(data: Vec<i32>) -> Self {
        Aggregator { data: Arc::new(data) }
    }
    fn calculate(&self) -> i32 {
        let nthreads = 4;
        let len = self.data.len();
        let chunk_size = (len + nthreads - 1) / nthreads;
        let mut handles = Vec::new();
        for i in 0..nthreads {
            let data = Arc::clone(&self.data);
            let start = i * chunk_size;
            let end = ((i + 1) * chunk_size).min(len);
            let handle = thread::spawn(move || {
                let mut local_sum = 0;
                unsafe {
                    let ptr = data.as_ptr().add(start);
                    for j in 0..(end - start) {
                        local_sum += *ptr.add(j);
                    }
                }
                local_sum
            });
            handles.push(handle);
        }
        let total: i32 = handles.into_iter().map(|h| h.join().unwrap()).sum();
        (total * 100) / (len as i32)
    }
}
fn main() {
    let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; 
    let aggregator = Aggregator::new(values);
    let result = aggregator.calculate();
    println!("Metric: {}", result);
}
