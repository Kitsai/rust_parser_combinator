use crate::parser::{combinators::{left_right::right, multiple::zero_or_more, pair::pair}, identifier::identifier, literal::literal, whitespace::space1, Parser};

use super::quoted_string;

pub fn attribute_pair<'a>() -> impl Parser<'a, (String,String)> {
    pair(identifier, right(literal("="), quoted_string()))
}

pub fn attributes<'a>() -> impl Parser<'a, Vec<(String,String)>> {
    zero_or_more(right(space1(), attribute_pair()))
}