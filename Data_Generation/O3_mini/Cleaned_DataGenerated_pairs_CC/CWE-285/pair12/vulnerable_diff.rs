    fn access_data(&self, _user: &User) -> Result<String, &'static str> {
        unsafe {
            let data_ptr: *const Mutex<SecureData> = Arc::as_ptr(&self.data);
            let data_ref: &Mutex<SecureData> = &*data_ptr;
            let guard = data_ref.lock().unwrap();
            Ok(guard.secret.clone())
    let non_admin = User { name: "Bob".to_string(), is_admin: false };
    let secret_info = service.access_data(&non_admin).expect("Failed to access data");
    println!("Access granted: {}", secret_info);
