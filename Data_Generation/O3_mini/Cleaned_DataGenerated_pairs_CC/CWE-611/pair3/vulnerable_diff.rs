use std::fs::File;
use std::io::Read;
use std::ffi::{CString, CStr};
    unsafe fn resolve_entity(&self, entity: &str) -> String {
        let c_str = CString::new(entity).unwrap();
        let ptr = c_str.as_ptr(); 
        let path = CStr::from_ptr(ptr).to_str().unwrap();
        let mut file = File::open(path).expect("Failed to open external file");
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("Failed to read external file");
        content
    }
            let first_quote = input.find('"').unwrap() + 1;
            let second_quote = input[first_quote..].find('"').unwrap() + first_quote;
            let entity = &input[first_quote..second_quote];
            unsafe { self.resolve_entity(entity) }
