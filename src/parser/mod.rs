pub type ParseResult<'a, Output> = Result<(&'a str, Output), &'a str>;
pub trait Parser<'a, Output> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output>;
}

impl<'a, F, Output> Parser<'a, Output> for F
where
    F: Fn(&'a str) -> ParseResult<Output>,
{
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output> {
        self(input)
    }
}

pub mod identifier;
pub mod combinators;
pub mod literal;

#[cfg(test)]
mod tests;