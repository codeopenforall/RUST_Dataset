#[derive(Debug, PartialEq)]
    fn run_operation(&self, role: Role) -> Result<i32, &'static str> {
        if role != Role::Admin {
            return Err("Unauthorized access: only admins can perform this operation");
        }
    match mgr.run_operation(Role::Admin) {
