static mut GLOBAL: *mut Data = std::ptr::null_mut();
fn init() {
    let data = Box::new(Data { count: 0 });
    unsafe {
        GLOBAL = Box::into_raw(data);
    }
}
    init();
        handles.push(thread::spawn(|| {
            unsafe {
                let data = GLOBAL.as_mut().expect("Not initialized");
                if data.count % 2 == 0 {
                    thread::sleep(Duration::from_millis(10));
                    data.count += 1;
                } else {
                    thread::sleep(Duration::from_millis(10));
                    data.count += 2;
                }
    unsafe {
        let final_data = &*GLOBAL;
        final_data.count
    }
    println!("Final count: {}", result);
