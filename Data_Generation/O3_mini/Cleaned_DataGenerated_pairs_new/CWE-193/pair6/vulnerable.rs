use std::vec::Vec;
struct Processor;
impl Processor {
    pub fn execute(&self, input: &[u32]) -> u32 {
        let n = input.len();
        let mut storage: Vec<u32> = Vec::with_capacity(n);
        unsafe {
            let ptr = storage.as_mut_ptr();
            for i in 0..=n {  
                if i < n {
                    *ptr.add(i) = input[i];
                } else {
                    *ptr.add(i) = 999;
                }
            }
            storage.set_len(n);
            let computed = std::slice::from_raw_parts(storage.as_ptr(), n + 1)
                            .iter()
                            .sum::<u32>();
            computed
        }
    }
}
fn main() {
    let proc = Processor;
    let data = vec![1, 2, 3, 4];
    let result = proc.execute(&data);
    println!("Result: {}", result);
}
