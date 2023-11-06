use crate::stdin_functions::answer_string_to_bool;
use std::io;

// get user input from stdin for bools, used by `AppCompatApp::new_from_command_line()`
pub fn get_bool_from_user(message: &'static str) -> Result<bool, String> {
    let stdin = io::stdin();
    println!("\n{}", message);
    let mut input_string = String::new();

    let mut r;

    // loop until a valid value is passed
    loop {
        stdin
            .read_line(&mut input_string)
            .map_err(|e| format!("an error occurred while reading input: {}", e.to_string()))?;
        r = answer_string_to_bool(input_string.clone());

        if r.is_err() {
            println!("Invalid entry. Try again.");
            println!("{}", message);
        } else {
            break;
        }
    }

    Ok(r?)
}
