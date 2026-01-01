////////////////////////////////////////////////////////////////////////////////////////////////////
// Vulnerability: Incorrect Calculation via Operator Precedence in a Concurrent Aggregator
// CWE-682: Incorrect Calculation (Logic/Math Error)
//
// This program concurrently processes an array of integers using multiple threads and unsafe 
// pointer arithmetic to sum segments of the data. The aggregation function then calculates a 
// metric as a percentage value using integer arithmetic. However, due to a logic error in the 
// ordering of operations, the computation divides the sum by the count before multiplying by 100. 
// This results in premature truncation (loss of precision) because the integer division is applied 
// first, causing an incorrect metric when the average is less than 1 or has significant digits lost.
// For example, with an input of [1,2,3,...,10], the vulnerable computation calculates (55/10)*100 = 500 
// instead of the correct (55*100)/10 = 550.
////////////////////////////////////////////////////////////////////////////////////////////////////

use std::sync::Arc;
use std::thread;

struct Aggregator {
    data: Arc<Vec<i32>>,
}

impl Aggregator {
    fn new(data: Vec<i32>) -> Self {
        Aggregator { data: Arc::new(data) }
    }

    // Incorrectly computes the percentage metric.
    fn calculate(&self) -> i32 {
        let nthreads = 4;
        let len = self.data.len();
        let chunk_size = (len + nthreads - 1) / nthreads;
        let mut handles = Vec::new();

        for i in 0..nthreads {
            let data = Arc::clone(&self.data);
            let start = i * chunk_size;
            let end = ((i + 1) * chunk_size).min(len);
            let handle = thread::spawn(move || {
                let mut local_sum = 0;
                unsafe {
                    // Unsafe block iterating via raw pointer over a slice subrange
                    let ptr = data.as_ptr().add(start);
                    for j in 0..(end - start) {
                        local_sum += *ptr.add(j);
                    }
                }
                local_sum
            });
            handles.push(handle);
        }

        let total: i32 = handles.into_iter().map(|h| h.join().unwrap()).sum();
        
        // Vulnerable calculation: performs integer division before multiplication.
        // Correct formula should multiply before division.
        (total / (len as i32)) * 100
    }
}

fn main() {
    let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; 
    // Expected metric: (55 * 100) / 10 = 550, but vulnerability leads to (55 / 10) * 100 = 500.
    let aggregator = Aggregator::new(values);
    let result = aggregator.calculate();
    println!("Metric: {}", result);
}