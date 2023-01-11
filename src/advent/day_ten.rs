use super::inputs::read_file;
use anyhow::Result;

///
/// # Errors
///
/// # Panics
///
pub fn day_ten() -> Result<()> {
    let lines = read_file("inputs/day_ten_input.txt")?.flatten().peekable();

    let mut x = 1;
    let mut cycle = 0;
    // let mut strength_vec: Vec<i32> = Vec::new();
    let mut position = 0;

    for line in lines {
        let line_vec: Vec<&str> = line.split(' ').collect(); // noop, addx
        match *line_vec.first().unwrap() {
            "noop" => {
                cycle += 1;
                drawing(position, x);
                position += 1;
                // if check_strength(cycle) {
                //     println!(
                //         "During the {}th cycle, X has {}, strength is {}",
                //         cycle,
                //         x,
                //         cycle * x
                //     );
                //     // strength_vec.push(cycle * x);
                // } else {
                //     // println!("{}th cycle, X has {}", cycle, x);
                // }
            }
            "addx" => {
                for _ in 0..2 {
                    cycle += 1;
                    drawing(position, x);
                    position += 1;
                    // if check_strength(cycle) {
                    //     println!(
                    //         "During the {}th cycle, X has {}, strength is {}",
                    //         cycle,
                    //         x,
                    //         cycle * x
                    //     );
                    //     // strength_vec.push(cycle * x);
                    // } else {
                    //     // println!("{}th cycle, X has {}", cycle, x);
                    // }
                }
                x += line_vec[1].parse::<i32>().unwrap();
            }
            _ => {}
        }
    }
    println!("End: {cycle}th cycle, X has {x}");
    // let mut sum = 0;
    // for i in strength_vec {
    //     sum += i;
    // }

    // println!("Sum of signal strengths: {}", sum);

    Ok(())
}

// fn check_strength(cycle: i32) -> bool {
//     match cycle {
//         20 | 60 | 100 | 140 | 180 | 220 => true,
//         _ => false,
//     }
// }

fn drawing(mut position: i32, x: i32) {
    position %= 40;
    if position == 0 {
        println!();
    }
    if position < x - 1 || position > x + 1 {
        print!(".");
    } else {
        print!("#");
    }
}
