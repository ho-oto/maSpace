pub fn get_tex_open_from_char(c: char) -> Result<String, ()> {
    Ok(match c {
        '(' => "(",
        '[' => ".",
        '{' | '⟨' | '⌈' | '⌊' | '⎰' | '⌜' | '⌞' | '⟦' => return Err(()),
        _ => return Err(()),
    }
    .to_string())
}
