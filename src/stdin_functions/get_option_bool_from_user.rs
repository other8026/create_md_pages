use crate::stdin_functions::answer_string_to_option_bool;
use crate::structs::bool_or_none::BoolOrNone;
use std::io;

// get a user input from stdin for optional bools, used by `AppCompatApp::new_from_command_line()`
pub fn get_option_bool_from_user(message: &'static str) -> Result<BoolOrNone, String> {
    let stdin = io::stdin();
    println!("\n{}", message);
    let mut input_string = String::new();
    stdin
        .read_line(&mut input_string)
        .map_err(|e| format!("an error occurred while reading input: {}", e.to_string()))?;
    let r = answer_string_to_option_bool(input_string)?;
    Ok(r)
}
