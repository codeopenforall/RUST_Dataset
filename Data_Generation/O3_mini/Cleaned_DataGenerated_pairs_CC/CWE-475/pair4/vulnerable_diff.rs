struct Handler {
impl Handler {
    fn transfer(&mut self, source: &[u8]) {
        let bytes_to_copy = input_len + 10; 
            ptr::copy_nonoverlapping(src_ptr, dest_ptr, bytes_to_copy);
    fn execute(&mut self, source: Vec<u8>) {
                locked.transfer(&source_clone);
    let mut processor = Handler::new(20); 
    processor.execute(payload);
    println!("Completed execution (vulnerable).");
