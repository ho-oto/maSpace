use nom::{character::complete::char, error::ParseError, multi::fold_many0, IResult, Parser};

pub fn num_space<'a, E: ParseError<&'a str>>(s: &'a str) -> IResult<&'a str, usize, E> {
    fold_many0(char(' '), usize::default, |x, _| x + 1)(s)
}

pub fn count_space_around<'a, R, F, E>(
    mut parser: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, usize, E>
where
    F: Parser<&'a str, R, E>,
    E: ParseError<&'a str>,
{
    move |input| {
        let (input, left) = num_space(input)?;
        let (input, _) = parser.parse(input)?;
        let (input, right) = num_space(input)?;
        Ok((input, left.max(right)))
    }
}
