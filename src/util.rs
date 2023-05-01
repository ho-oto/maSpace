use nom::{
    character::complete::char, error::ParseError, multi::fold_many0, sequence::tuple, IResult,
    Parser,
};

pub fn num_space<'a, E: ParseError<&'a str>>(s: &'a str) -> IResult<&'a str, usize, E> {
    fold_many0(char(' '), || 0, |x, _| x + 1)(s)
}

pub fn max_space_around<'a, R, F, E>(s: &'a str, parser: F) -> IResult<&'a str, usize, E>
where
    F: Parser<&'a str, R, E>,
    E: ParseError<&'a str>,
{
    let (s, (left, _, right)) = tuple((num_space, parser, num_space))(s)?;
    Ok((s, left.max(right)))
}
