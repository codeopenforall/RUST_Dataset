use std::env;
use std::path::PathBuf;
    pub fn resolve_path(&self, cmd: &str) -> Result<PathBuf, String> {
        let mut exe_path = env::current_exe().map_err(|e| e.to_string())?;
        exe_path.pop(); 
        exe_path.push(cmd);
        if exe_path.exists() {
            Ok(exe_path)
        } else {
            Err(format!("Executable at absolute path {:?} not found", exe_path))
        }
    }
        let abs_cmd = self.resolve_path(cmd)?;
        let result = Command::new(abs_cmd)
            .args(args)
            .output();
