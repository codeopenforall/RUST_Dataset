    fn resolve(&self) -> io::Result<PathBuf> {
        let raw_path = PathBuf::from(&self.path);
        let abs_path = fs::canonicalize(raw_path)?;
        let allowed = fs::canonicalize("./allowed")?;
        if !abs_path.starts_with(&allowed) {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Access not permitted"));
        Ok(abs_path)
    let resolved_path = resolver.resolve()?;
        if let Ok(mut f) = OpenOptions::new().write(true).create(false).open(&*sp_clone) {
            let _ = f.write_all(b"safe update");
    let file_path = if args.len() > 1 { &args[1] } else { "./allowed/test.txt" };
