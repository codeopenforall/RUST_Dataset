    fn withdraw(&mut self, amount: u32) {
        unsafe {
            subtract_unchecked(&mut self.balance as *mut u32, amount);
}
unsafe fn subtract_unchecked(val: *mut u32, sub: u32) {
    *val = (*val).wrapping_sub(sub);
        account.withdraw(1);
