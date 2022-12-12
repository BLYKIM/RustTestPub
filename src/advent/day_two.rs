use anyhow::Result;

use super::inputs::read_file;

///
/// # Errors
///
pub fn day_two() -> Result<()> {
    let lines = read_file("inputs/day_two_input.txt")?;

    let mut total = 0u32;
    for line in lines.flatten() {
        let abc = line.as_str();
        // A rock       1
        // B paper      2
        // C scissor    3
        //
        // X lose       0
        // Y draw       3
        // Z win        6

        // rock
        if abc.starts_with('A') {
            if abc.ends_with('X') {
                total += 3;
            }
            if abc.ends_with('Y') {
                total += 4;
            }
            if abc.ends_with('Z') {
                total += 8;
            }
        }

        // paper
        if abc.starts_with('B') {
            if abc.ends_with('X') {
                total += 1;
            }
            if abc.ends_with('Y') {
                total += 5;
            }
            if abc.ends_with('Z') {
                total += 9;
            }
        }

        // scissor
        if abc.starts_with('C') {
            if abc.ends_with('X') {
                total += 2;
            }
            if abc.ends_with('Y') {
                total += 6;
            }
            if abc.ends_with('Z') {
                total += 7;
            }
        }
    }
    println!("{}", total);

    Ok(())
}
