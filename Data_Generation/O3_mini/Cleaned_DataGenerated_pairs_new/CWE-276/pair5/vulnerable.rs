use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::sync::{Arc, Mutex};
use std::thread;
use std::io::{self, Write};
struct Server {
    path: String,
}
trait Task {
    fn configure(&self) -> io::Result<()>;
}
impl Task for Server {
    fn configure(&self) -> io::Result<()> {
        unsafe {
            let ptr = self.path.as_ptr();
            let _ = ptr;
        }
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .mode(0o777)
            .open(&self.path)?;
        drop(file);
        Ok(())
    }
}
fn spawn_service(srv: Arc<Server>) -> io::Result<()> {
    let guard = Arc::new(Mutex::new(()));
    let srv_clone = srv.clone();
    let guard_clone = guard.clone();
    let handle = thread::spawn(move || {
        let _lock = guard_clone.lock().unwrap();
        srv_clone.configure().unwrap();
    });
    handle.join().unwrap();
    Ok(())
}
fn main() {
    let srv = Arc::new(Server { path: String::from("example.txt") });
    spawn_service(srv).unwrap();
}
