use crate::parser::Parser;

pub fn pred<'a, P, A, F>(parser: P, predicate: F) -> impl Parser<'a, A>
where 
    P: Parser<'a, A>,
    F: Fn(&A) -> bool,
{
    move |input| {
        if let Ok((next_input, value)) = parser.parse(input) {
            if predicate(&value) {
                return Ok((next_input,value));
            }
        }
        Err(input)
    }
}