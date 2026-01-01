use std::thread;
struct Container {
    payload: Vec<u8>,
}
impl Container {
    fn compute(&self) -> Option<u32> {
        if self.payload.len() < 12 {
            return None;
        }
        let mut sum = 0u32;
        unsafe {
            let ptr = self.payload.as_ptr() as *const u32;
            for i in 0..3 {
                sum = sum.wrapping_add(ptr.add(i).read());
            }
        }
        Some(sum)
    }
}
fn run() {
    let data: Vec<u8> = vec![
        1, 0, 0, 0,   
        2, 0, 0, 0,   
        3, 0, 0, 0    
    ];
    let container = Container { payload: data };
    let handle = thread::spawn(move || {
        container.compute()
    });
    match handle.join().unwrap() {
        Some(result) => println!("Computed sum: {}", result),
        None => println!("Invalid input length")
    }
}
fn main() {
    run();
}
