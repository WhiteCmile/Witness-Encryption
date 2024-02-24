use std::io::stdin;
use std::str::FromStr;

pub fn input_string() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to get user input");
    input.trim_end_matches('\n').to_string()
}

pub fn input_numbers<T>() -> Vec<T> 
where   T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug
{
    let input = input_string();
    let numbers = input.split_whitespace()
                        .into_iter()
                        .map(|num| num.parse().expect("Input is not a number"))
                        .collect();
    numbers
}