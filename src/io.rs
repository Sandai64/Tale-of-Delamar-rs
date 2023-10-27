// Bring stdout().flush() into scope
use std::io::Write;

/// Handles user input & returns it.
/// Note: This method flushes stdout.
pub fn stdin_input(prompt: &str, trim: Option<bool>) -> String
{
    print!("{}", String::from(prompt));
    std::io::stdout().flush().unwrap();

    let mut user_input: String = String::new();
    std::io::stdin().read_line(&mut user_input).expect("err: failed to read input.");

    if trim.unwrap_or(false)
    {
        user_input = String::from(user_input.trim());
    }

    return user_input;
}
