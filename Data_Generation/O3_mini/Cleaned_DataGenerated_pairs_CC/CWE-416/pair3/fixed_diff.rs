use std::sync::Arc;
    data: Arc<Data>,
        Self { data: Arc::new(Data { value: val }) }
    fn read(&self) -> u32 {
        self.data.value
    let data_clone = holder.data.clone();
    let handle = thread::spawn(move || {
        data_clone.value
