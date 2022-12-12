use anyhow::Result;

use super::inputs::read_file;

///
/// # Errors
///
pub fn day_one() -> Result<()> {
    let lines = read_file("inputs/day_one_input.txt")?;
    let mut first = 0u32;
    let mut second = 0u32;
    let mut third = 0u32;
    let mut sum_food = 0u32;
    let mut elf_cnt = 0u32;
    let mut elves = Vec::new();

    for line in lines.flatten() {
        if line.is_empty() {
            elf_cnt += 1;
            if sum_food > first {
                if second < first {
                    if third < second {
                        third = second;
                        second = first;
                    } else {
                        second = first;
                    }
                } else {
                }
                first = sum_food;
            }
            if sum_food > second && sum_food < first {
                second = sum_food;
            }
            if sum_food > third && sum_food < second {
                third = sum_food;
            }
            elves.push((sum_food, elf_cnt));
            sum_food = 0;
        } else {
            let food = line.parse::<u32>().expect("number");
            sum_food += food;
        }
    }

    println!(
        " {} , {} , {}, {}",
        first,
        second,
        third,
        first + second + third
    );

    Ok(())
}
