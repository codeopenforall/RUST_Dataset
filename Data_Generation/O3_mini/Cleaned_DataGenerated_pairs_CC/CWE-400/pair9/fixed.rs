use std::thread;
struct Worker;
impl Worker {
    pub fn execute(&self, data: Vec<u32>) -> u32 {
        let handle = thread::spawn(move || {
            data.iter().sum()
        });
        handle.join().unwrap()
    }
}
pub fn run_fn(data: Vec<u32>) -> u32 {
    let worker = Worker;
    worker.execute(data)
}
fn main() {
    let worker = Worker;
    let input = vec![1u32; 150]; 
    let res = worker.execute(input);
    println!("Computed sum: {}", res);
}
