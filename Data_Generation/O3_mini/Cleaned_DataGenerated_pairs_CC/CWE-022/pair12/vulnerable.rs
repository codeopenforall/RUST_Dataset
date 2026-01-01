use std::{fs, path::PathBuf, sync::{Arc, Mutex}};
struct Loader {
    base: PathBuf,
}
impl Loader {
    fn new() -> Self {
        Self { base: PathBuf::from("data") }
    }
    fn load(&self, file: &str) -> Result<String, String> {
        let path = self.base.join(file);
        unsafe {
            let cpath = std::ffi::CString::new(path.to_string_lossy().into_owned())
                .map_err(|e| e.to_string())?;
            let ptr = cpath.as_ptr();
            let slice = std::slice::from_raw_parts(ptr as *const u8, cpath.to_bytes().len());
            let path_str = String::from_utf8_lossy(slice).into_owned();
            fs::read_to_string(path_str).map_err(|e| e.to_string())
        }
    }
}
fn main() {
    let loader = Arc::new(Mutex::new(Loader::new()));
    let handles: Vec<_> = (0..2)
        .map(|_| {
            let loader = Arc::clone(&loader);
            std::thread::spawn(move || {
                let result = loader.lock().unwrap().load("../secret.txt");
                match result {
                    Ok(content) => println!("Loaded content: {}", content),
                    Err(err) => println!("Error: {}", err),
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
}
