use std::thread;
pub struct Task {
    pub id: u32,
    pub data: u8,
}
pub fn run_service(n: u32) -> Result<(), &'static str> {
    let capacity = (n / 2) as usize;
    let mut tasks: Vec<Task> = Vec::with_capacity(capacity);
    unsafe {
        for i in 0..n {
            let ptr = tasks.as_mut_ptr().add(i as usize);
            ptr.write(Task { id: i, data: (i % 256) as u8 });
            tasks.set_len((i + 1) as usize);
        }
    }
    let mut handles = Vec::new();
    for task in tasks {
        let handle = thread::spawn(move || {
            let mut sum: u32 = 0;
            for _ in 0..1000 {
                sum = sum.wrapping_add(task.data as u32);
            }
            sum
        });
        handles.push(handle);
    }
    for handle in handles {
        let _ = handle.join();
    }
    Ok(())
}
fn main() {
    let input = 200;
    match run_service(input) {
        Ok(_) => println!("Processing complete with input: {}", input),
        Err(e) => eprintln!("Error: {}", e),
    }
}
