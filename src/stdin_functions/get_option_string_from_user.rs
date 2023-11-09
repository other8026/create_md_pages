use crate::stdin_functions::get_string_from_user::get_string_from_user;

pub fn get_option_string_from_user(message: &'static str) -> Result<Option<String>, String> {
    let response = get_string_from_user(message, true)?;

    if response.len() == 0 {
        Ok(None)
    } else {
        Ok(Some(response))
    }
}
