    total: u32,
        let factor: u32 = 3;
        unsafe {
            let calc = (amount as u64).wrapping_mul(factor as u64);
            let credit = calc as u32;
            self.total = self.total.wrapping_add(credit);
        }
    fn get_total(&self) -> u32 {
fn simulate_transaction(amount: u32) -> u32 {
    let target = 1_500_000_000; 
