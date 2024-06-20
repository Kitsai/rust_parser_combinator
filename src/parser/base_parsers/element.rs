use std::vec;

use crate::{element::Element, parser::{combinators::{either::either, left_right::{left, right}, multiple::zero_or_more, pair::pair}, identifier::identifier, literal::literal, whitespace::whitespace_wrap, Parser}};

use super::attribute::attributes;

fn element_start<'a>() -> impl Parser<'a, (String, Vec<(String,String)>)> {
    right(literal("<"), pair(identifier, attributes()))
}

fn open_element<'a>() -> impl Parser<'a, Element> {
    left(element_start(), literal(">")).map(|(name,attributes)| Element {
        name,
        attributes,
        children: vec![],
    })
}

fn close_element<'a>(expected_name: String) -> impl Parser<'a, String> {
    right(literal("</"), left(identifier, literal(">")))
        .pred(move |name| name == &expected_name)
}

pub fn parent_element<'a>() -> impl Parser<'a, Element> {
    open_element().bind(|el| {
        left(zero_or_more(element()), close_element(el.name.clone())).map(move|children| {
            let mut el = el.clone();
            el.children = children;
            el
        })
    })
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

pub fn element<'a>() -> impl Parser<'a, Element> {
    whitespace_wrap(either(single_element(), parent_element()))
}