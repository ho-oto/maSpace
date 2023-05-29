use maspace::maspace_to_tex;

use std::io;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .context("failed to read stdin")?;
    let result = maspace_to_tex(&buffer)?;
    println!("{}", result);
    Ok(())
}
