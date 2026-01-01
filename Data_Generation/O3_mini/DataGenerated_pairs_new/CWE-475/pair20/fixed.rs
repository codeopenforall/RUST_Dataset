use std::thread;

struct Container {
    payload: Vec<u8>,
}

impl Container {
    // This method safely computes a sum by first ensuring that the payload length is adequate (at least 12 bytes).
    // If the input does not meet the precondition, it returns None; otherwise it performs an unsafe conversion which is now valid.
    fn compute(&self) -> Option<u32> {
        if self.payload.len() < 12 {
            return None;
        }
        let mut sum = 0u32;
        unsafe {
            // Since the payload length is validated, it is safe to reinterpret it as a u32 slice.
            let ptr = self.payload.as_ptr() as *const u32;
            for i in 0..3 {
                sum = sum.wrapping_add(ptr.add(i).read());
            }
        }
        Some(sum)
    }
}

fn run() {
    // Provide valid input: exactly 12 bytes so that three u32 values can be read safely.
    let data: Vec<u8> = vec![
        1, 0, 0, 0,   // Represents u32 value 1
        2, 0, 0, 0,   // Represents u32 value 2
        3, 0, 0, 0    // Represents u32 value 3
    ];
    let container = Container { payload: data };

    // Spawn a thread to execute the computation concurrently.
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