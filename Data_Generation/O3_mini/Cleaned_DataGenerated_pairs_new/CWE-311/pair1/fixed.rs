use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
struct Config {
    secret: String,
}
impl Config {
    unsafe fn ptr_reveal(&self) -> *const u8 {
        self.secret.as_ptr()
    }
}
fn encrypt(data: &str, key: u8) -> Vec<u8> {
    data.bytes().map(|b| b ^ key).collect()
}
fn decrypt(data: &[u8], key: u8) -> String {
    let decrypted: Vec<u8> = data.iter().map(|b| b ^ key).collect();
    String::from_utf8_lossy(&decrypted).into_owned()
}
fn process(conf: Arc<Mutex<Config>>) {
    let conf_clone = Arc::clone(&conf);
    thread::spawn(move || {
        let conf_lock = conf_clone.lock().unwrap();
        let secret_ptr = unsafe { conf_lock.ptr_reveal() };
        let secret_len = conf_lock.secret.len();
        let secret_slice = unsafe { std::slice::from_raw_parts(secret_ptr, secret_len) };
        for byte in secret_slice {
            print!("{:02x}", byte);
        }
        println!();
    })
    .join()
    .unwrap();
}
fn main() {
    let config = Config {
        secret: "my_super_secret".to_owned(),
    };
    let arc_conf = Arc::new(Mutex::new(config));
    let key: u8 = 0xAA;
    let plain_secret = {
        let conf_lock = arc_conf.lock().unwrap();
        let ptr = unsafe { conf_lock.ptr_reveal() };
        let len = conf_lock.secret.len();
        unsafe {
            String::from_utf8_lossy(std::slice::from_raw_parts(ptr, len)).into_owned()
        }
    };
    let encrypted_data = encrypt(&plain_secret, key);
    let mut file = File::create("secret.txt").expect("cannot create file");
    file.write_all(&encrypted_data)
        .expect("cannot write data");
    process(arc_conf);
    let decrypted = decrypt(&encrypted_data, key);
    assert_eq!(decrypted, plain_secret, "Decryption failed");
}
