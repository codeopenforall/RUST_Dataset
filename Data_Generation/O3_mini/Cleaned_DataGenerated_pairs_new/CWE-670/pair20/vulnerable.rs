use std::sync::{Arc, Mutex};
use std::thread;
struct TaskManager {
    tasks: Vec<i32>,
}
impl TaskManager {
    fn new(size: usize) -> Self {
        TaskManager { tasks: vec![0; size] }
    }
    fn execute(&mut self, idx: usize, value: i32, perform: bool) -> Result<(), &'static str> {
        unsafe {
            match perform {
                true => {
                    if idx < self.tasks.len() - 1 {
                        let ptr = self.tasks.as_mut_ptr().add(idx + 1);
                        *ptr = value;
                        Ok(())
                    } else {
                        Err("index out of bounds")
                    }
                },
                false => {
                    if idx < self.tasks.len() {
                        let ptr = self.tasks.as_mut_ptr().add(idx);
                        *ptr = value;
                        Ok(())
                    } else {
                        Err("index out of bounds")
                    }
                },
            }
        }
    }
    fn get_tasks(&self) -> &[i32] {
        &self.tasks
    }
}
fn main() {
    let mgr = Arc::new(Mutex::new(TaskManager::new(3)));
    let mut handles = vec![];
    {
        let mgr_clone = Arc::clone(&mgr);
        handles.push(thread::spawn(move || {
            let mut mgr = mgr_clone.lock().unwrap();
            mgr.execute(0, 100, false).unwrap();
        }));
    }
    {
        let mgr_clone = Arc::clone(&mgr);
        handles.push(thread::spawn(move || {
            let mut mgr = mgr_clone.lock().unwrap();
            mgr.execute(0, 200, true).unwrap();
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let mgr = mgr.lock().unwrap();
    println!("Tasks: {:?}", mgr.get_tasks());
}
