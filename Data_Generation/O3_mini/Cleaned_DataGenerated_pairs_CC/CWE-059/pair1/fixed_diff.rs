use std::fs::{File, symlink_metadata};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
fn is_valid_path(target: &Path, allowed: &Path) -> bool {
    if let Ok(canonical) = target.canonicalize() {
        canonical.starts_with(allowed)
    } else {
        false
}
fn process_input(path: &str, allowed: &Path) -> Result<Settings, String> {
    let file_path = Path::new(path);
    let metadata = symlink_metadata(file_path).map_err(|e| e.to_string())?;
    if metadata.file_type().is_symlink() || !is_valid_path(file_path, allowed) {
         return Err("Insecure file path detected".to_string());
    }
    let mut file = File::open(file_path).map_err(|e| e.to_string())?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
