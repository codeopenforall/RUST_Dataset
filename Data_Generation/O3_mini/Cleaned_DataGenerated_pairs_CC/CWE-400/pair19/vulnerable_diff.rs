    fn new() -> Self {
        Service { tasks: Vec::new() }
        let data = Box::new([0u8; 1024]);
        self.tasks.push(data);
        unsafe {
            let ptr = self.tasks.as_mut_ptr();
            *ptr = Box::new([1u8; 1024]);
pub fn run_service(iterations: usize, _limit: Option<usize>) -> usize {
    let service = Arc::new(Mutex::new(Service::new()));
    let _limit = if args.len() > 2 {
         None
    let count = run_service(iterations, _limit);
