use super::Token;

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag, take_until},
    character::complete::anychar,
    combinator::{map, map_res, opt},
    sequence::{delimited, pair, preceded, terminated},
    IResult,
};

pub fn take_open(s: &str) -> IResult<&str, Token> {
    terminated(
        map(
            alt((
                delimited(
                    tag("`"),
                    map_res(take_until("`"), tex_of_ascii_art_open),
                    tag("`"),
                ),
                map_res(anychar, tex_of_char_open),
            )),
            |x| Token::Open(x),
        ),
        opt(is_a(" ")),
    )(s)
}

pub fn take_close(s: &str) -> IResult<&str, Token> {
    preceded(
        opt(is_a(" ")),
        map(
            pair(
                alt((
                    delimited(
                        tag("`"),
                        map_res(take_until("`"), tex_of_ascii_art_close),
                        tag("`"),
                    ),
                    map_res(anychar, tex_of_char_close),
                )),
                opt(is_a("'")),
            ),
            |(x, y)| Token::Close(format!("{}{}", x, y.unwrap_or_default())),
        ),
    )(s)
}

fn tex_of_char_open(c: char) -> Result<String, ()> {
    Ok(match c {
        '(' => "(",
        '[' => ".",
        '{' => r"\{",
        '⟨' => r"\langle",
        '⌈' => r"\lceil",
        '⌊' => r"\lfloor",
        '⌜' => r"\ulcorner",
        '⌞' => r"\llcorner",
        _ => return Err(()),
    }
    .to_string())
}

fn tex_of_char_close(c: char) -> Result<String, ()> {
    Ok(match c {
        ')' => ")",
        ']' => ".",
        '}' => r"\}",
        '⟩' => r"\rangle",
        '⌉' => r"\rceil",
        '⌋' => r"\rfloor",
        '⌝' => r"\urcorner",
        '⌟' => r"\lrcorner",
        _ => return Err(()),
    }
    .to_string())
}

fn tex_of_ascii_art_open(s: &str) -> Result<String, ()> {
    Ok(match s {
        "[" => "[",
        "[<" => r"\langle",
        "[|" => r"\lvert",
        "[||" => r"\lVert",
        "[^" => r"\lceil",
        "[_" => r"\lfloor",
        "[|^" => r"\ulcorner",
        "[|_" => r"\llcorner",
        "[[]" => "]",
        "[[)" => ")",
        "[[}" => r"\}",
        "[[>" => r"\rangle",
        _ => return Err(()),
    }
    .to_string())
}

fn tex_of_ascii_art_close(s: &str) -> Result<String, ()> {
    Ok(match s {
        "]" => "]",
        ">]" => r"\rangle",
        "|]" => r"\rvert",
        "||]" => r"\rVert",
        "^]" => r"\rceil",
        "_]" => r"\rfloor",
        "^|]" => r"\urcorner",
        "_|]" => r"\lrcorner",
        "[]]" => "[",
        "(]]" => "(",
        "{]]" => r"\{",
        "<]]" => r"\langle",
        _ => return Err(()),
    }
    .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take_open() {
        fn x(a: &str) -> (&str, Token) {
            take_open(a).unwrap()
        }
        fn y(y: &str) -> Token {
            Token::Open(y.to_string())
        }
        assert_eq!(x("(123"), (r"123", y(r"(")));
        assert_eq!(x("`[<`123"), (r"123", y(r"\langle")));
    }

    #[test]
    fn test_take_close() {
        fn x(a: &str) -> (&str, Token) {
            take_close(a).unwrap()
        }
        fn y(y: &str) -> Token {
            Token::Close(y.to_string())
        }
        assert_eq!(x(")123"), (r"123", y(r")")));
        assert_eq!(x("`>]`'123"), (r"123", y(r"\rangle'")));
    }
}
