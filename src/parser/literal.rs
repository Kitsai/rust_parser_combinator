use super::Parser;

pub fn literal<'a>(expected: &'static str) -> impl Parser<'a, ()> {
    move |input: &'a str| if input.starts_with(expected) {
        Ok((&input[expected.len()..], ()))
    } else {
        Err(input)
    }
}