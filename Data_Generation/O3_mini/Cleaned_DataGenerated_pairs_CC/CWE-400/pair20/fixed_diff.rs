struct Connection {
struct Service {
    connections: Arc<Mutex<Vec<Connection>>>,
    max_connections: usize,
impl ResourceManager for Service {
    fn new(limit: usize) -> Self {
        Service {
            connections: Arc::new(Mutex::new(Vec::new())),
            max_connections: limit,
        let connections = self.connections.clone();
        let max = self.max_connections;
            let mut id_generator = 0u64;
                {
                    let mut cons = connections.lock().unwrap();
                    if cons.len() < max {
                        cons.push(Connection { id: id_generator });
                    }
                id_generator = id_generator.wrapping_add(1);
                thread::sleep(Duration::from_millis(10));
        self.connections.lock().unwrap().len()
pub type ResourceImpl = Service;
