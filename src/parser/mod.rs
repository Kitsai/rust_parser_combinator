fn match_literal(expected: &'static str)
-> impl Fn(&str) -> Result<(&str, ()), &str> 
{
    move | input| match input.get(0..expected.len()) {
        Some(next) if next == expected => {
            Ok((&input[expected.len()..],()))
        }
        _ => Err(input),
    } 
}