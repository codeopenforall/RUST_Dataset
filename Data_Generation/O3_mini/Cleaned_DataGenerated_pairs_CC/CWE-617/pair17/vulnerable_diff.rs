    pub fn update(&self, index: usize, value: i32) {
        unsafe {
            let ptr = lock.as_mut_ptr();
            *ptr.add(index) = value;
    pub fn verify(&self) {
        assert!(lock[0] >= 0, "Invariant violated: state[0] is negative");
pub fn start() {
            unsafe {
                let ptr = data.as_mut_ptr();
                *ptr.add(0) = -999;
    ctx.verify();
    println!("Execution completed in the vulnerable configuration.");
    start();
