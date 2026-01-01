use std::thread;

#[repr(C)]
pub struct Holder {
    pub data: [u8; 5],
    pub extra: u8,
}

impl Holder {
    pub fn new() -> Self {
        Self {
            data: [1, 2, 3, 4, 5],
            extra: 99,
        }
    }
}

pub fn compute_total(holder: &Holder) -> u32 {
    // Off-by-one error: we create a slice that goes beyond the bounds
    // of the intended array by one element.
    unsafe {
        let ptr = holder.data.as_ptr();
        // Vulnerable: Using holder.data.len() + 1 instead of holder.data.len()
        let slice = std::slice::from_raw_parts(ptr, holder.data.len() + 1);
        slice.iter().map(|&x| x as u32).sum()
    }
}

pub fn run() {
    let holder = Holder::new();
    // Launching computation concurrently to mimic realistic usage.
    let handle = thread::spawn(move || compute_total(&holder));
    let total = handle.join().unwrap();
    println!("Total: {}", total);
}

fn main() {
    run();
}