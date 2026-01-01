/*
This is the test oracle that validates the authorization controls.
It creates a resource with an initial secret value of 42, then attempts to update it
using a user with a Regular role. In both versions the interface is identical.
The test expects that an unauthorized update will NOT change the secret value.
For the vulnerable version, the test will fail because the secret gets modified.
For the corrected version, the test will pass, leaving the secret unchanged.
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unauthorized_update() {
        let data = Data { secret: 42 };
        // A Regular user who should not have update privileges.
        let user = User { id: 10, role: Role::Regular };

        // Attempt an update with an unauthorized user.
        data.update(&user, 999);

        // The secret must remain unchanged (42) if proper authorization is enforced.
        assert_eq!(data.get(), 42, "Unauthorized update occurred: secret value changed");
    }
}