use super::ParseResult;

pub fn any_char(input: &str) -> ParseResult<char> {
    match input.chars().next() {
        Some(next) => Ok((&input[next.len_utf8()..],next)),
        _ => Err(input),
    }
}