    io::{Error, Write},
    path::{Path, PathBuf},
        let base_canon = self.base.canonicalize()?;
        let target_canon = match target.canonicalize() {
            Ok(path) => path,
            Err(_) => {
                target
                    .parent()
                    .and_then(|p| p.canonicalize().ok())
                    .map(|parent| parent.join(target.file_name().unwrap()))
                    .ok_or_else(|| Error::new(std::io::ErrorKind::Other, "Invalid path"))?
            }
        };
        if !target_canon.starts_with(&base_canon) {
            return Err(Error::new(
                std::io::ErrorKind::PermissionDenied,
                "Access Denied: Path traversal detected",
            ));
        }
            let raw = target_canon.to_str().unwrap().as_ptr();
        let res = server_lock.process("../outside.txt", "safe data");
        if res.is_err() {
            println!("Access denied as expected.");
        } else {
            println!("Unexpected success.");
        }
    println!("Main operation complete (corrected version).");
