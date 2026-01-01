use std::alloc::{alloc, dealloc, Layout};
struct Block {
    ptr: *mut u32,
    len: usize,
}
impl Block {
    unsafe fn from_slice(slice: &[u32]) -> Self {
        let len = slice.len();
        let layout = Layout::array::<u32>(len).unwrap();
        let ptr = alloc(layout) as *mut u32;
        if ptr.is_null() {
            panic!("Allocation failed");
        }
        for i in 0..len {
            *ptr.add(i) = slice[i];
        }
        Block { ptr, len }
    }
    unsafe fn compute_sum(&self) -> u32 {
        let mut total = 0;
        for i in 0..self.len {
            total += *self.ptr.add(i);
        }
        total
    }
}
fn process_data(data: &[u32]) -> u32 {
    unsafe {
        let block = Block::from_slice(data);
        let result = block.compute_sum();
        let layout = Layout::array::<u32>(block.len).unwrap();
        dealloc(block.ptr as *mut u8, layout);
        result
    }
}
fn main() {
    let input = vec![1, 2, 3, 4];
    let res = process_data(&input);
    println!("Resulting sum: {}", res);
}
