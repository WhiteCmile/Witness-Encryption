use std::io::stdin;
pub fn get_stdin(input: &mut String) {
    stdin().read_line(input).expect("Failed to get user input");
}