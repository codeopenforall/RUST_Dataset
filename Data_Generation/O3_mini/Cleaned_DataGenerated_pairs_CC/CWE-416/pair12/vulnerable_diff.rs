#![allow(unused_unsafe)]
use std::sync::{Arc, Mutex};
    let raw = Box::into_raw(resource);
    unsafe {
        let _ = Box::from_raw(raw); 
    }
    let result = unsafe { (*raw).compute() }; 
    result
    let _handle = thread::spawn(|| {
        let raw_ptr = Box::into_raw(boxed);
        unsafe {
            let _ = Box::from_raw(raw_ptr); 
        }
        let wrong_use = unsafe { (*raw_ptr).compute() };
        println!("Thread result: {}", wrong_use);
