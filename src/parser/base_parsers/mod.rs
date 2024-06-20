use super::{char::any_char, combinators::{left_right::{left, right}, multiple::zero_or_more}, literal::literal, Parser};

pub fn quoted_string<'a>() -> impl Parser<'a, String> {
        right(
           literal("\""),
           left(
                zero_or_more(any_char.pred(|c| *c != '"')),
                literal("\""),
           ) 
        )
        .map(|chars| chars.into_iter().collect())
}

pub mod attribute;
pub mod element;