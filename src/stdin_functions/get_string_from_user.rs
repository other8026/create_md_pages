use std::io;

// get a user input from stdin for strings, used by `AppCompatApp::new_from_command_line()`
pub fn get_string_from_user(message: &'static str, can_be_empty: bool) -> Result<String, String> {
    let stdin = io::stdin();
    println!("\n{}", message);
    let mut input_string = String::new();

    let mut r;

    // loop in case something the entered data is invalid, which in this case is too short
    loop {
        stdin
            .read_line(&mut input_string)
            .map_err(|e| format!("an error occurred while reading input: {}", e.to_string()))?;
        r = input_string.trim().to_string();

        if !can_be_empty && r.len() == 0 {
            println!("Invalid (too short). Try again.");
            println!("{}", message);
        } else {
            break;
        }
    }

    Ok(r)
}
