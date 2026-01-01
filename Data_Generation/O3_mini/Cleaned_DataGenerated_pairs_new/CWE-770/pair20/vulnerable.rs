use std::alloc::{alloc, dealloc, Layout};
struct ResourceManager;
impl ResourceManager {
    pub fn request(&self, count: u64) -> Result<(), String> {
        let size = count
            .checked_mul(std::mem::size_of::<u64>() as u64)
            .ok_or("Integer overflow computing size")?;
        let layout = Layout::from_size_align(size as usize, std::mem::align_of::<u64>())
            .map_err(|_| "Invalid layout for given size")?;
        unsafe {
            let ptr = alloc(layout);
            if ptr.is_null() {
                return Err("Memory allocation failed".to_string());
            }
            std::ptr::write_bytes(ptr, 0u8, size as usize);
            dealloc(ptr, layout);
        }
        Ok(())
    }
}
fn execute(count: u64) -> Result<u64, String> {
    let manager = ResourceManager;
    manager.request(count)?;
    Ok(count)
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let count: u64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    match execute(count) {
        Ok(val) => println!("Processed count: {}", val),
        Err(err) => eprintln!("Error: {}", err),
    }
}
