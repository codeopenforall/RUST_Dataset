struct Processor {
    factor: u32,
impl Processor {
    fn compute(&self, data: &[u32]) -> u32 {
        let mut sum: u32 = 0;
                sum = sum.wrapping_add(*ptr.add(i));
            sum = sum * self.factor;
        sum
    let proc = Arc::new(Processor { factor: 2 });
    let data_clone = Arc::clone(&numbers);
    let proc_clone = Arc::clone(&proc);
        proc_clone.compute(&data_clone[0..1])
    let data_clone2 = Arc::clone(&numbers);
    let proc_clone2 = Arc::clone(&proc);
        proc_clone2.compute(&data_clone2[1..2])
    let combined = part1.wrapping_add(part2);
    combined as u64
