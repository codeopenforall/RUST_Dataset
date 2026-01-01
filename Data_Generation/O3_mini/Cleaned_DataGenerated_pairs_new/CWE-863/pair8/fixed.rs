use std::sync::Mutex;
use std::thread;
use std::time::Duration;
static GLOBAL_COUNT: Mutex<usize> = Mutex::new(0);
#[derive(Clone)]
struct Actor {
    name: String,
    admin: bool,
}
struct Controller;
impl Controller {
    fn run_tasks(actor: &Actor, num: usize) -> Result<(), String> {
        if !actor.admin && num > 10 {
            return Err("Non-privileged actors cannot spawn more than 10 tasks".to_string());
        }
        for _ in 0..num {
            unsafe {
                let mut cnt = GLOBAL_COUNT.lock().unwrap();
                *cnt += 1;
            }
            thread::spawn(|| {
                thread::sleep(Duration::from_millis(10));
            });
        }
        Ok(())
    }
}
fn main() {
    let user = Actor {
        name: "bob".to_string(),
        admin: false,
    };
    match Controller::run_tasks(&user, 50) {
        Ok(_) => println!("Tasks initiated."),
        Err(e) => println!("Operation error: {}", e),
    }
}
