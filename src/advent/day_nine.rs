use super::inputs::read_file;
use anyhow::Result;

///
/// # Errors
///
/// # Panics
///
pub fn day_nine() -> Result<()> {
    let lines = read_file("inputs/day_nine_input.txt")?.flatten().peekable();

    for _line in lines {}
    Ok(())
}
