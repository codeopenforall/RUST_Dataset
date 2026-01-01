    total: u64,
        let factor: u64 = 3;
        let calc = (amount as u64)
            .checked_mul(factor)
            .expect("Multiplication overflow");
        self.total = self.total
            .checked_add(calc)
            .expect("Addition overflow");
    fn get_total(&self) -> u64 {
fn simulate_transaction(amount: u32) -> u64 {
    let target = 1_500_000_000;
