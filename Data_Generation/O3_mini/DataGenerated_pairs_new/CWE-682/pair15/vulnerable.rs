////////////////////////////
// Vulnerable Code Sample //
////////////////////////////
use std::sync::Arc;
use std::thread;

struct SensorData {
    readings: Vec<u32>,
}

impl SensorData {
    fn process(&self) -> u32 {
        let factor = 3u32;
        let divisor = 2u32;
        // Demonstrating unsafe pointer usage to simulate real‐world patterns.
        let unsafe_factor = unsafe { *(&factor as *const u32) };

        let data = Arc::new(self.readings.clone());
        let mut handles = Vec::new();
        let chunk_size = (data.len() + 1) / 2;

        // Atomic accumulator for concurrent summation.
        let sum_arc = Arc::new(std::sync::atomic::AtomicU32::new(0));

        // Spawn threads to process chunks concurrently.
        for chunk in data.chunks(chunk_size) {
            let local_data = chunk.to_vec();
            let sum_ref = Arc::clone(&sum_arc);
            let unsafe_factor_cpy = unsafe_factor; // propagate unsafe value
            let divisor_cpy = divisor;
            let handle = thread::spawn(move || {
                let mut local_sum = 0;
                for reading in local_data {
                    // Vulnerability: Incorrect calculation due to misplaced operator precedence.
                    // Instead of (reading * unsafe_factor_cpy) / divisor_cpy, the division is performed first.
                    local_sum += reading * (unsafe_factor_cpy / divisor_cpy);
                }
                sum_ref.fetch_add(local_sum, std::sync::atomic::Ordering::SeqCst);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
        sum_arc.load(std::sync::atomic::Ordering::SeqCst)
    }
}

fn main() {
    let sensor = SensorData { readings: vec![10, 20, 30, 40] };
    let result = sensor.process();
    println!("Result: {}", result);
}