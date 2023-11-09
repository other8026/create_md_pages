pub mod get_bool_from_user;
pub mod get_option_bool_from_user;
pub mod get_option_string_from_user;
pub mod get_string_from_user;

use crate::structs::bool_or_none::BoolOrNone;

// convert string to bool
pub fn answer_string_to_bool(input: String) -> Result<bool, String> {
    let lowercase = input.to_lowercase();
    let trimmed = lowercase.trim();
    match trimmed {
        "yes" => Ok(true),
        "y" => Ok(true),
        "no" => Ok(false),
        "n" => Ok(false),
        _ => Err("An unrecognized value was entered.".to_string()),
    }
}

// convert string to actual Optional bool
pub fn answer_string_to_option_bool(input: String) -> Result<BoolOrNone, String> {
    let trimmed = input.trim();
    if trimmed.len() == 0 || input == "idk" {
        Ok(BoolOrNone(None))
    } else {
        Ok(BoolOrNone(Some(answer_string_to_bool(input)?)))
    }
}
