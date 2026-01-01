    max_tasks: usize,
    fn new(max_tasks: usize) -> Self {
        Service { tasks: Vec::new(), max_tasks }
        if self.tasks.len() < self.max_tasks {
            let data = Box::new([0u8; 1024]);
            self.tasks.push(data);
            unsafe {
                let ptr = self.tasks.as_mut_ptr();
                if !ptr.is_null() {
                    *ptr = Box::new([1u8; 1024]);
                }
            }
pub fn run_service(iterations: usize, limit: Option<usize>) -> usize {
    let max = limit.unwrap_or(iterations);
    let service = Arc::new(Mutex::new(Service::new(max)));
    let limit = if args.len() > 2 {
         Some(100)
    let count = run_service(iterations, limit);
