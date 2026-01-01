const MAX_TASKS: u32 = 100;
        if count > MAX_TASKS {
            return Err("Task count exceeds allowed limit");
        }
    let result = proc_inst.run(50);
