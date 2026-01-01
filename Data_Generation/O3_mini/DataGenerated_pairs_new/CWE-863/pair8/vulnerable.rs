/////////////////////// VULNERABLE CODE ///////////////////////
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
    // This function spawns a number of tasks even if the actor is not privileged.
    // It lacks any limits on the number of tasks that can be started.
    fn run_tasks(actor: &Actor, num: usize) -> Result<(), String> {
        // Vulnerability: Missing authorization check on num for non-privileged actors.
        for _ in 0..num {
            // Unsafe block used to simulate some low-level global state update.
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
    // A non-privileged actor requesting a large number of tasks.
    let user = Actor {
        name: "bob".to_string(),
        admin: false,
    };
    // For demonstration, a non-admin actor is allowed to spawn 50 tasks.
    match Controller::run_tasks(&user, 50) {
        Ok(_) => println!("Tasks initiated."),
        Err(e) => println!("Operation error: {}", e),
    }
}