use std::thread;

struct Container {
    payload: Vec<u8>,
}

impl Container {
    // This method computes a sum by reinterpreting the underlying byte vector as three u32 values.
    // It does not validate that the payload length is sufficient (i.e. at least 12 bytes), which violates
    // the API precondition for safely reinterpreting the memory layout, leading to undefined behavior.
    fn compute(&self) -> Option<u32> {
        let mut sum = 0u32;
        unsafe {
            let ptr = self.payload.as_ptr() as *const u32;
            // Vulnerability: always reading three u32 values regardless of the actual payload length.
            for i in 0..3 {
                // If payload length is less than 12, ptr.add(i).read() accesses out of bounds.
                sum = sum.wrapping_add(ptr.add(i).read());
            }
        }
        Some(sum)
    }
}

fn run() {
    // Prepare data that is intentionally too short (8 bytes only).
    let data = vec![1u8, 2, 3, 4, 5, 6, 7, 8];
    let container = Container { payload: data };

    // Spawn a thread to perform the computation concurrently.
    let handle = thread::spawn(move || {
        // Calling compute() here will invoke undefined behavior by reading memory out-of-bound.
        container.compute()
    });
    let result = handle.join().unwrap();
    println!("Computed sum: {:?}", result);
}

fn main() {
    run();
}