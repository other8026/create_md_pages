use crate::stdin_functions::get_string_from_user::get_string_from_user;
use crate::structs::string_or_none::StringOrNone;

pub fn get_option_string_from_user(message: &'static str) -> Result<StringOrNone, String> {
    let response = get_string_from_user(message, true)?;

    if response.len() == 0 {
        Ok(StringOrNone(None))
    } else {
        Ok(StringOrNone(Some(response)))
    }
}
