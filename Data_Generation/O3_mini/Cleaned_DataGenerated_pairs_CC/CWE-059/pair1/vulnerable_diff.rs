use std::fs::File;
use std::io::{Read, Seek, Write};
use std::path::Path;
fn process_input(path: &str, _allowed: &Path) -> Result<Settings, String> {
    let file_path = Path::new(path);
    let mut file = File::open(file_path).map_err(|e| e.to_string())?;
    let file_container = Arc::new(Mutex::new(file));
    let file_container_clone = Arc::clone(&file_container);
    let handle = thread::spawn(move || {
         let mut inner_file = file_container_clone.lock().unwrap();
         let _ = inner_file.seek(std::io::SeekFrom::Start(0));
    });
    handle.join().unwrap();
    let mut buffer = Vec::new();
    {
        let mut locked = file_container.lock().unwrap();
        locked.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
