struct Calculator {
    factor: u64,
impl Calculator {
    fn compute(&self, data: &[u32]) -> u64 {
        let mut sum: u64 = 0;
                sum = sum.wrapping_add(*ptr.add(i) as u64);
        sum.checked_mul(self.factor).expect("Multiplication overflow")
    let calc = Arc::new(Calculator { factor: 2 });
    let numbers_clone = Arc::clone(&numbers);
    let calc_clone = Arc::clone(&calc);
        calc_clone.compute(&numbers_clone[0..1])
    let numbers_clone2 = Arc::clone(&numbers);
    let calc_clone2 = Arc::clone(&calc);
        calc_clone2.compute(&numbers_clone2[1..2])
    part1.checked_add(part2).expect("Addition overflow")
