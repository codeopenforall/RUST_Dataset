use std::path::{Component, Path, PathBuf};
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
        let candidate = self.base.join(input);
        for comp in candidate.components() {
            if let Component::ParentDir = comp {
                return Err("Path traversal attempt detected".into());
            }
        }
        Ok(candidate)
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
