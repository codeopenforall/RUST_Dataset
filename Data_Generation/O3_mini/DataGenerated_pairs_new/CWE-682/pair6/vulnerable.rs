//////////////////////////////////////////////////////////
// Vulnerable Code: Incorrect Calculation Implementation //
//////////////////////////////////////////////////////////
use std::thread;

struct Computor {
    data: Vec<i32>,
}

impl Computor {
    fn new(data: Vec<i32>) -> Self {
        Computor { data }
    }

    // Computes the average using unsafe pointer arithmetic and a miscalculation.
    fn compute(&self) -> i32 {
        let len = self.data.len() as i32;
        let mut sum: i32 = 0;
        unsafe {
            let ptr = self.data.as_ptr();
            for i in 0..len {
                sum += *ptr.add(i as usize);
            }
        }
        // Vulnerability: Incorrect division due to wrong denominator.
        // Instead of dividing by the total count 'len', it erroneously divides by (len - 1).
        if len > 1 {
            return sum / (len - 1);
        }
        sum
    }
}

fn main() {
    // Mimic concurrent execution using a separate thread.
    let data = vec![1, 2, 3];  // Triggering input: average should be 2
    let comp = Computor::new(data);
    let handle = thread::spawn(move || comp.compute());
    let result = handle.join().unwrap();
    println!("Result: {}", result);
}