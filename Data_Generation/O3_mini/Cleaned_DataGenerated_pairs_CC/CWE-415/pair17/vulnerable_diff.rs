use std::sync::{Arc, Barrier};
struct Manager {
    ptr: *mut i32,
impl Manager {
        Manager {
            ptr: Box::into_raw(boxed),
        unsafe { *self.ptr }
impl Drop for Manager {
        unsafe {
            Box::from_raw(self.ptr);
    let manager = Manager::new(500);
    let dup_ptr = manager.ptr; 
        unsafe {
            Box::from_raw(dup_ptr);
    println!("Value: {}", manager.get());
