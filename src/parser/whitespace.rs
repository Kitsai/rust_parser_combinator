use super::{char::any_char, combinators::{left_right::{left, right}, multiple::{one_or_more, zero_or_more}}, Parser};

pub fn whitespace_char<'a>() -> impl Parser<'a, char> {
    any_char.pred(|c|c.is_whitespace()) 
}

pub fn space1<'a>() -> impl Parser<'a, Vec<char>> {
    one_or_more(whitespace_char())
}

pub fn space0<'a>() -> impl Parser<'a, Vec<char>> {
    zero_or_more(whitespace_char())
}

pub fn whitespace_wrap<'a, P, A>(parser: P) -> impl Parser<'a, A>
where
    P: Parser<'a, A>,
{
    right(space0(), left(parser, space0()))
}