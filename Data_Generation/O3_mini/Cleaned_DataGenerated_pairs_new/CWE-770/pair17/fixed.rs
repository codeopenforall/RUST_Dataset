use std::process;
const LIMIT: usize = 100;
fn process_input(n: usize) -> Result<u64, &'static str> {
    if n > LIMIT {
        return Err("Input exceeds allowed limit");
    }
    let mut vec = Vec::with_capacity(n);
    unsafe { vec.set_len(n); }
    for i in 0..n {
        vec[i] = 1;
    }
    let sum: u64 = vec.iter().sum();
    Ok(sum)
}
fn main() {
    let n: usize = 50;
    match process_input(n) {
        Ok(s) => println!("Sum: {}", s),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        },
    }
}
