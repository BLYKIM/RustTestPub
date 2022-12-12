use super::inputs::read_file;
use anyhow::Result;

///
/// # Errors
///
/// # Panics
///
pub fn day_seven() -> Result<()> {
    let _lines = read_file("inputs/day_four_input.txt")?.flatten().peekable();

    Ok(())
}
