use std::ffi::CString;
use std::path::{Path, PathBuf};
use std::thread;
struct Manager {
    base: PathBuf,
}
impl Manager {
    fn new(base: &Path) -> Self {
        Manager {
            base: base.to_path_buf(),
        }
    }
    fn get_path(&self, input: &str) -> Result<PathBuf, String> {
        let joined = self.base.join(input);
        unsafe {
            let as_str = joined
                .to_str()
                .ok_or("Error: Invalid encoding in path")?;
            let c_string = CString::new(as_str)
                .map_err(|_| "Error: CString conversion failed")?;
            let raw = c_string.into_raw();
            let recovered = CString::from_raw(raw);
            let result_str = recovered.to_str().map_err(|_| "Error: UTF-8 conversion failed")?;
            Ok(PathBuf::from(result_str))
        }
    }
}
fn main() {
    let safe = std::path::Path::new("safe_dir");
    let instance = Manager::new(safe);
    let input = "../secret.txt";
    let handle = thread::spawn({
        let inst = instance;
        move || {
            match inst.get_path(input) {
                Ok(path) => println!("Computed path: {:?}", path),
                Err(e) => println!("Error: {}", e),
            }
        }
    });
    handle.join().unwrap();
}
