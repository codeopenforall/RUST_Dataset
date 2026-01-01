/* Vulnerable version demonstrating an out‐of‐bounds write via an unsafe memory copy.
   This code intentionally writes one extra element (with value 99) outside of the
   allocated memory bounds and then reads it as part of a summing loop. */
use std::alloc::{alloc, dealloc, Layout};

struct Block {
    ptr: *mut u32,
    len: usize,
}

impl Block {
    // Copies data into allocated memory and intentionally writes out-of-bound.
    unsafe fn from_slice(slice: &[u32]) -> Self {
        let len = slice.len();
        let layout = Layout::array::<u32>(len).unwrap();
        let ptr = alloc(layout) as *mut u32;
        if ptr.is_null() {
            panic!("Allocation failed");
        }

        // Copy each element from the slice.
        for i in 0..len {
            *ptr.add(i) = slice[i];
        }
        // Vulnerability: Writing one additional element outside of allocated bounds.
        *ptr.add(len) = 99; // CWE-119: Improper Restriction of Operations within Memory Bounds.

        Block { ptr, len }
    }

    // Computes the sum over len+1 elements, thus reading the out-of-bound element.
    unsafe fn compute_sum(&self) -> u32 {
        let mut total = 0;
        // Vulnerability: Loop iterates over len + 1 elements, including the extra value.
        for i in 0..=self.len {
            total += *self.ptr.add(i);
        }
        total
    }
}

fn process_data(data: &[u32]) -> u32 {
    unsafe {
        let block = Block::from_slice(data);
        let result = block.compute_sum();
        // Free the originally allocated memory. Note: The layout is computed for self.len elements.
        let layout = Layout::array::<u32>(block.len).unwrap();
        dealloc(block.ptr as *mut u8, layout);
        result
    }
}

fn main() {
    let input = vec![1, 2, 3, 4];
    // Expected correct sum would be 10, but due to the out-of-bound write and read,
    // the computed sum becomes 10 + 99 = 109.
    let res = process_data(&input);
    println!("Resulting sum: {}", res);
}