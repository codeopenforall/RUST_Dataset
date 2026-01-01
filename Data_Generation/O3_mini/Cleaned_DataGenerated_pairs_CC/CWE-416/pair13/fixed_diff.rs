use std::sync::Arc;
    let data = Arc::new(Data { value: 512 });
    let data_clone = Arc::clone(&data);
    data_clone.value
