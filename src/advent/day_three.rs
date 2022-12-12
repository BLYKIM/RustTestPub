use super::inputs::read_file;
use anyhow::Result;

///
/// # Errors
///
pub fn day_three() -> Result<()> {
    let mut lines = read_file("inputs/day_three_input.txt")?
        .flatten()
        .peekable();
    // let mut sum_prior = 0u32;
    let mut sum_badges = 0u32;

    // for line in lines {
    //     let len = line.len();
    //     let (first, second) = line.split_at(len / 2);
    //     'l: for ch1 in first.chars() {
    //         for ch2 in second.chars() {
    //             if ch1 == ch2 {
    //                 let prior = char_to_num(ch1);
    //                 sum_prior += prior;
    //                 break 'l;
    //             }
    //         }
    //     }
    // }

    // println!("{}", sum_prior);
    loop {
        if lines.peek().is_none() {
            break;
        }
        if let Some(line1) = lines.next() {
            if let Some(line2) = lines.next() {
                if let Some(line3) = lines.next() {
                    'l: for ch1 in line1.chars() {
                        for ch2 in line2.chars() {
                            for ch3 in line3.chars() {
                                if ch1 == ch2 && ch1 == ch3 {
                                    let badge = char_to_num(ch1);
                                    sum_badges += badge;
                                    break 'l;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{}", sum_badges);

    Ok(())
}

fn char_to_num(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 96
    } else {
        c as u32 - 38
    }
}
