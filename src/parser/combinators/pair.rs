use crate::parser::Parser;

pub fn pair<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, (R1, R2)>
where
    P1: Parser<'a, R1>, //+ 'a,
    P2: Parser<'a, R2>,//+ 'a,
    // R1: 'a + Clone,
    // R2: 'a,
{
    move |input| 
        parser1.parse(input).and_then(|(next_input, result1)| {
            parser2.parse(next_input)
                .map(|(last_input, result2)| (last_input, (result1, result2)))
        })
    // parser1.bind(move |result1| parser2.map(move |result2| (result1.clone(), result2)))
}