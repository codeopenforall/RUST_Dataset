use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
struct Server {
    log_path: String,
}
impl Server {
    unsafe fn create_log(&self) -> io::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .mode(0o777) 
            .open(&self.log_path)?;
        let ptr = Box::into_raw(Box::new(file));
        (*ptr).write_all(b"Server started\n")?;
        Box::from_raw(ptr);
        Ok(())
    }
    fn run(&self) -> io::Result<()> {
        let arc_self = Arc::new(self.log_path.clone());
        let log_clone = arc_self.clone();
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(100));
            let _ = OpenOptions::new()
                .append(true)
                .open(&*log_clone)
                .and_then(|mut f| f.write_all(b"Background thread log\n"));
        });
        unsafe {
            self.create_log()?;
        }
        handle.join().unwrap();
        Ok(())
    }
}
fn main() {
    let srv = Server {
        log_path: "tempfile.txt".to_string(),
    };
    if let Err(e) = srv.run() {
        eprintln!("Error running server: {}", e);
    }
}
