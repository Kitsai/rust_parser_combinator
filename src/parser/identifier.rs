use regex::Regex;

use super::ParseResult;

pub fn identifier(input: &str) -> ParseResult<String> {
    let re = Regex::new(r"^[A-Za-z][A-Za-z0-9-]*").unwrap();
    match re.find(input) {
        Some(matched) => {
            let matched = matched.as_str();
            let next_index = matched.len();
            Ok((&input[next_index..], matched.to_string()))
        }
        None => Err(input),
    }

    // let mut matched = String::new();
    // let mut chars = input.chars();

    // match chars.next() {
    //     Some(next) if next.is_alphabetic() => matched.push(next),
    //     _ => return Err(input),
    // }

    // while let Some(next) = chars.next() {
    //     if next.is_alphanumeric() || next == '-' {
    //         matched.push(next);
    //     } else {
    //         break;
    //     }
    // }

    // let next_index = matched.len();
    // Ok((&input[next_index..], matched))
}