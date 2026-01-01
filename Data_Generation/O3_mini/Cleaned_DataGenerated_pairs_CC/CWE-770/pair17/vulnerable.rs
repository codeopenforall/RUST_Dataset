use std::process;
const LIMIT: usize = 100;
fn process_input(n: usize) -> Result<u64, &'static str> {
    let mut vec = Vec::with_capacity(n);
    unsafe { vec.set_len(n); }
    for i in 0..n {
        vec[i] = 1;
    }
    if n > LIMIT {
        unsafe {
            let ptr = vec.as_mut_ptr();
            *ptr.add(n) = 42; 
        }
    }
    let sum: u64 = vec.iter().sum();
    Ok(sum)
}
fn main() {
    let n: usize = 150;
    match process_input(n) {
        Ok(s) => println!("Sum: {}", s),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        },
    }
}
