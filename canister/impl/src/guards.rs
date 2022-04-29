use crate::read_state;

pub fn caller_is_admin() -> Result<(), String> {
    read_state(|state| {
        let caller = state.env.caller();
        if state.data.admins.contains(&caller) {
            Ok(())
        } else {
            Err("Caller is not an admin".to_string())
        }
    })
}
