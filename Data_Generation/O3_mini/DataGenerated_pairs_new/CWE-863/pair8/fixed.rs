/////////////////////// CORRECTED CODE ///////////////////////
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
    // Now this function enforces a strict limit on the number of tasks a non-privileged actor can spawn.
    fn run_tasks(actor: &Actor, num: usize) -> Result<(), String> {
        // If actor is not privileged, only allow up to 10 tasks.
        if !actor.admin && num > 10 {
            return Err("Non-privileged actors cannot spawn more than 10 tasks".to_string());
        }
        for _ in 0..num {
            // Unsafe block remains to simulate a low-level operation.
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
    // A non-privileged actor attempting to spawn too many tasks.
    let user = Actor {
        name: "bob".to_string(),
        admin: false,
    };
    // In the corrected implementation, this call will return an error.
    match Controller::run_tasks(&user, 50) {
        Ok(_) => println!("Tasks initiated."),
        Err(e) => println!("Operation error: {}", e),
    }
}