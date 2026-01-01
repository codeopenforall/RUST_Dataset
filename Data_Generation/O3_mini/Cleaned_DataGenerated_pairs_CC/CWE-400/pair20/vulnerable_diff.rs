struct Conn {
struct Server {
    conns: Arc<Mutex<Vec<Conn>>>,
impl ResourceManager for Server {
    fn new(_limit: usize) -> Self {
        Server {
            conns: Arc::new(Mutex::new(Vec::new())),
        let conns = self.conns.clone();
            let mut id_counter = 0u64;
                unsafe {
                    let connection = Conn { id: id_counter };
                    let ptr: *mut Conn = Box::into_raw(Box::new(connection));
                    (*conns.lock().unwrap()).push(*Box::from_raw(ptr));
                    id_counter = id_counter.wrapping_add(1);
        self.conns.lock().unwrap().len()
pub type ResourceImpl = Server;
