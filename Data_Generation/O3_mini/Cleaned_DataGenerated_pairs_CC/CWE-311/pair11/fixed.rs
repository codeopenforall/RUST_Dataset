const SECRET_VALUE: &str = "myS3cr3t!";
const KEY: u8 = 0xAB; 
struct Config {
    secret: Box<String>,
}
fn load_configuration() -> Config {
    Config {
        secret: Box::new(String::from(SECRET_VALUE)),
    }
}
fn encrypt_data(secret: &String) -> String {
    secret
        .bytes()
        .map(|b| format!("{:02x}", b ^ KEY))
        .collect::<String>()
}
pub fn get_transmitted_data() -> String {
    let config = load_configuration();
    let handle = std::thread::spawn(move || encrypt_data(&config.secret));
    handle.join().expect("Thread panicked")
}
fn main() {
    let transmitted = get_transmitted_data();
    println!("Transmitted data: {}", transmitted);
}
