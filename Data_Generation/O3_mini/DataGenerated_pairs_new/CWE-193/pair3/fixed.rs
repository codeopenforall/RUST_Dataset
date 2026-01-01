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
    // Correctly create a slice limited to the bounds of "data".
    unsafe {
        let ptr = holder.data.as_ptr();
        let slice = std::slice::from_raw_parts(ptr, holder.data.len());
        slice.iter().map(|&x| x as u32).sum()
    }
}

pub fn run() {
    let holder = Holder::new();
    let handle = thread::spawn(move || compute_total(&holder));
    let total = handle.join().unwrap();
    println!("Total: {}", total);
}

fn main() {
    run();
}