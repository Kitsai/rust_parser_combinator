use crate::{element::Element, parser::{combinators::{left_right::{left, right}, map::map, pair::pair}, identifier::identifier, literal::literal, Parser}};

use super::attribute::attributes;

fn element_start<'a>() -> impl Parser<'a, (String, Vec<(String,String)>)> {
    right(literal("<"), pair(identifier, attributes()))
}

pub fn single_element<'a>() -> impl Parser<'a, Element> {
    left(element_start(), literal("/>")). map(
        |(name, attributes)| Element {
            name,
            attributes,
            children: vec![],
        }
    )
}