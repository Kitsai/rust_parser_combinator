use super::{char::any_char, combinators::{multiple::{one_or_more, zero_or_more}, pred::pred}, Parser};

pub fn whitespace_char<'a>() -> impl Parser<'a, char> {
    pred(any_char, |c|c.is_whitespace())
}

pub fn space1<'a>() -> impl Parser<'a, Vec<char>> {
    one_or_more(whitespace_char())
}

pub fn space0<'a>() -> impl Parser<'a, Vec<char>> {
    zero_or_more(whitespace_char())
}