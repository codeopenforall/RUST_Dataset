use std::thread;
struct Computor {
    data: Vec<i32>,
}
impl Computor {
    fn new(data: Vec<i32>) -> Self {
        Computor { data }
    }
    fn compute(&self) -> i32 {
        let len = self.data.len() as i32;
        let mut sum: i32 = 0;
        unsafe {
            let ptr = self.data.as_ptr();
            for i in 0..len {
                sum += *ptr.add(i as usize);
            }
        }
        if len > 1 {
            return sum / (len - 1);
        }
        sum
    }
}
fn main() {
    let data = vec![1, 2, 3];  
    let comp = Computor::new(data);
    let handle = thread::spawn(move || comp.compute());
    let result = handle.join().unwrap();
    println!("Result: {}", result);
}
