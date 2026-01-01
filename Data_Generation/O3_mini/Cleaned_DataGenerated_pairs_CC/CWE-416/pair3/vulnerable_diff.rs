use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;
    ptr: *mut Data,
        let boxed = Box::new(Data { value: val });
        Self { ptr: Box::into_raw(boxed) }
    unsafe fn read(&self) -> u32 {
        (*self.ptr).value
impl Drop for Holder {
    fn drop(&mut self) {
        unsafe {
            if !self.ptr.is_null() {
                Box::from_raw(self.ptr);
            }
        }
    }
}
static GLOBAL_PTR: AtomicPtr<Data> = AtomicPtr::new(ptr::null_mut());
    GLOBAL_PTR.store(holder.ptr, Ordering::SeqCst);
    drop(holder);
    let handle = thread::spawn(|| unsafe {
        let p = GLOBAL_PTR.load(Ordering::SeqCst);
        (*p).value
