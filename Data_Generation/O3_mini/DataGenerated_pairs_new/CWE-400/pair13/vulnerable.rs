use std::thread;

pub struct Task {
    pub id: u32,
    pub data: u8,
}

pub fn run_service(n: u32) -> Result<(), &'static str> {
    // Intentionally allocate a vector with insufficient capacity to simulate uncontrolled resource consumption.
    let capacity = (n / 2) as usize;
    let mut tasks: Vec<Task> = Vec::with_capacity(capacity);

    // UNSAFE BLOCK: Writes into the vector without proper bounds checking.
    unsafe {
        for i in 0..n {
            // Compute the target pointer regardless of the allocated capacity.
            let ptr = tasks.as_mut_ptr().add(i as usize);
            // Write a new Task into memory.
            ptr.write(Task { id: i, data: (i % 256) as u8 });
            // Manually increase the length without verifying that capacity is not exceeded.
            tasks.set_len((i + 1) as usize);
        }
    }

    // Spawn a thread for each task without any backpressure.
    let mut handles = Vec::new();
    for task in tasks {
        let handle = thread::spawn(move || {
            let mut sum: u32 = 0;
            // Simulate CPU-intensive work.
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
    // Using an input value that exceeds the safe capacity.
    let input = 200;
    match run_service(input) {
        Ok(_) => println!("Processing complete with input: {}", input),
        Err(e) => eprintln!("Error: {}", e),
    }
}