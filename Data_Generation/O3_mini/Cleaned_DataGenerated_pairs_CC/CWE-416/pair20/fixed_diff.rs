use std::sync::Arc;
    handle: Arc<Object>,
    fn new(handle: Arc<Object>) -> Self {
        Handler { handle }
        self.handle.data
    let obj = Arc::new(Object::new(123));
    let handler = Handler::new(Arc::clone(&obj));
    drop(obj);
