use crate::element::Element;

use super::*;
    
use super::char::any_char;
use base_parsers::{attribute::attributes, element::single_element, quoted_string};
use identifier::identifier;
use combinators::{left_right::right, multiple::*, pair::pair, pred::pred};
use literal::literal;

#[test]
fn literal_parser() {
    let parse_joe = literal("Hello Joe!");
    assert_eq!(
        Ok(("",())),
        parse_joe.parse("Hello Joe!")
    );

    assert_eq!(
        Ok((" Hello Robert!",())),
        parse_joe.parse("Hello Joe! Hello Robert!")
    );

    assert_eq!(
        Err("Hello Mike!"),
        parse_joe.parse("Hello Mike!")
    );
}

#[test]
fn identifier_parser() {
    assert_eq!(
        Ok(("", "i-am-an-identifier".to_string())),
        identifier("i-am-an-identifier")
    );
    assert_eq!(
        Ok((" entirely an identifier", "not".to_string())),
        identifier("not entirely an identifier")
    );
    assert_eq!(
        Err("!not at all an identifier"),
        identifier("!not at all an identifier")
    );
}

#[test]
fn pair_combinator() {
    let tag_opener = pair(literal("<"),identifier);

    assert_eq!(
        Ok(("/>", ((),"my-first-element".to_string()))),
        tag_opener.parse("<my-first-element/>")
    );
    assert_eq!(Err("ooops"), tag_opener.parse("ooops"));
    assert_eq!(Err("!oops"), tag_opener.parse("<!oops"));
}

#[test]
fn right_combinator() {
    let tag_opener = right(literal("<"), identifier);
    assert_eq!(
        Ok(("/>","my-first-element".to_string())),
        tag_opener.parse("<my-first-element/>")
    );
    assert_eq!(Err("ooops"), tag_opener.parse("ooops"));
    assert_eq!(Err("!oops"), tag_opener.parse("<!oops"));
}

#[test]
fn one_or_more_combinator() {
    let parser = one_or_more(literal("ha"));
    assert_eq!(Ok(("",vec![(),(),()])), parser.parse("hahaha"));
    assert_eq!(Err("ahah"), parser.parse("ahah"));
    assert_eq!(Err(""), parser.parse(""));
}

#[test]
fn zero_or_more_combinator() {
    let parser = zero_or_more(literal("ha"));
    assert_eq!(Ok(("",vec![(),(),()])), parser.parse("hahaha"));
    assert_eq!(Ok(("ahah", vec![])), parser.parse("ahah"));
    assert_eq!(Ok(("", vec![])), parser.parse(""));
}

#[test]
fn predicate_combinator() {
    let parser = pred(any_char, |c| *c == 'o');
    assert_eq!(Ok(("mg", 'o')), parser.parse("omg"));
    assert_eq!(Err("lol"), parser.parse("lol"));
}

#[test]
fn quoted_string_parser() {
    assert_eq!(
        Ok(("", "Hello Joe!".to_string())),
        quoted_string().parse("\"Hello Joe!\"")
    )
}

#[test]
fn attribute_parser() {
    assert_eq!(
        Ok((
            "",
            vec![
                ("one".to_string(), "1".to_string()),
                ("two".to_string(), "2".to_string()),
            ]
        )),
        attributes().parse(" one=\"1\" two=\"2\"")
    )
}

#[test]
fn single_element_parser() {
    assert_eq!(
        Ok((
            "",
            Element {
                name: "div".to_string(),
                attributes: vec![("class".to_string(), "float".to_string())],
                children: vec![]
            }
        )),
        single_element().parse("<div class=\"float\"/>")
    )
}

mod final_test;