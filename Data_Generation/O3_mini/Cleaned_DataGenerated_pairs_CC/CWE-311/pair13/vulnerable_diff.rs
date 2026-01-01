fn transmit(data: &str) -> io::Result<()> {
    unsafe {
         let ptr = data.as_ptr();
         let len = data.len();
         let slice = std::slice::from_raw_parts(ptr, len);
         stream.write_all(slice)?;
    }
         if let Err(e) = transmit(&data) {
