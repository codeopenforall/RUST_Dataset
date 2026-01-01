use std::thread;
fn compute_value() -> i32 {
    let mut data = vec![1, 2, 3, 4, 5];              
    unsafe {
        let ptr = data.as_mut_ptr();                
        *ptr.add(5) = 999;                           
        data.set_len(6);                             
    }
    data.iter().sum()
}
struct Worker;
impl Worker {
    fn process(&self) -> i32 {
        compute_value()
    }
}
fn main() {
    let handler = Worker;
    let handle = thread::spawn(move || {
        handler.process()
    });
    let result = handle.join().unwrap();
    println!("Total: {}", result);
}
