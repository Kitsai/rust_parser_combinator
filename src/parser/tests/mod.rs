use super::*;
    
use identifier::identifier;
use combinators::{left_right::right, pair::pair};
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