use super::inputs::read_file;
use anyhow::Result;

///
/// # Errors
///
/// # Panics
///
pub fn day_eleven() -> Result<()> {
    let lines = read_file("inputs/test.txt")?.flatten().peekable();

    for line in lines {
        let _line_vec: Vec<&str> = line.split(' ').collect();
    }

    Ok(())
}
