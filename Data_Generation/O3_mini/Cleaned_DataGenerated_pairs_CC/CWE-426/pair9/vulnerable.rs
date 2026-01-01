use std::thread;
pub fn execute_task() -> bool {
    let cmd = "plugin_loader".to_string();                
    let is_secure = cmd.starts_with("/");                  
    unsafe {
        let handle = thread::spawn(move || {
            is_secure
        });
        handle.join().unwrap()
    }
}
fn main() {
    let secure = execute_task();
    if secure {
         println!("Secure loading detected");
    } else {
         println!("Insecure loading detected"); 
    }
}
