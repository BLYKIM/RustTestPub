use std::collections::HashMap;

use super::inputs::read_file;
use anyhow::Result;

///
/// # Errors
///
/// # Panics
///
#[allow(unused_assignments)]
pub fn day_nine() -> Result<()> {
    let lines = read_file("inputs/day_nine_input.txt")?.flatten().peekable();
    let mut rope = vec![(0, 0); 10];
    let rope_len = rope.len();
    let mut visited: HashMap<(i32, i32), bool> = HashMap::new();
    visited.insert((0, 0), true); // start position

    // let mut testcnt = 0;
    for line in lines {
        let step: Vec<&str> = line.split(' ').collect();
        let dist = step[1].parse::<u32>()?;
        let mut diag_move = rope.clone();

        match *step.first().unwrap() {
            "U" => {
                // println!("up {}", dist);
                for _ in 0..dist {
                    diag_move = rope.clone();
                    rope.first_mut().unwrap().1 += 1;

                    for i in 1..rope_len {
                        match is_adj(rope[i - 1], rope[i]) {
                            AdjInfo::DownSide => rope[i] = (rope[i - 1].0, rope[i - 1].1 - 1),
                            AdjInfo::UpSide => rope[i] = (rope[i - 1].0, rope[i - 1].1 + 1),
                            AdjInfo::LeftSide => rope[i] = (rope[i - 1].0 - 1, rope[i - 1].1),
                            AdjInfo::RightSide => rope[i] = (rope[i - 1].0 + 1, rope[i - 1].1),
                            AdjInfo::Diagonal => rope[i] = diag_move[i - 1],
                            AdjInfo::Adj => {}
                        }
                        if i == 9 {
                            visited.insert(rope[i], true);
                        }
                    }
                    // println!("{:?}", rope);
                }
            }
            "D" => {
                // println!("down {}", dist);
                for _ in 0..dist {
                    diag_move = rope.clone();
                    rope.first_mut().unwrap().1 -= 1;

                    for i in 1..rope_len {
                        match is_adj(rope[i - 1], rope[i]) {
                            AdjInfo::DownSide => rope[i] = (rope[i - 1].0, rope[i - 1].1 - 1),
                            AdjInfo::UpSide => rope[i] = (rope[i - 1].0, rope[i - 1].1 + 1),
                            AdjInfo::LeftSide => rope[i] = (rope[i - 1].0 - 1, rope[i - 1].1),
                            AdjInfo::RightSide => rope[i] = (rope[i - 1].0 + 1, rope[i - 1].1),
                            AdjInfo::Diagonal => rope[i] = diag_move[i - 1],
                            AdjInfo::Adj => {}
                        }
                        if i == 9 {
                            visited.insert(rope[i], true);
                        }
                    }
                    // println!("{:?}", rope);
                }
            }
            "L" => {
                // println!("left {}", dist);
                for _ in 0..dist {
                    diag_move = rope.clone();
                    rope.first_mut().unwrap().0 -= 1;

                    for i in 1..rope_len {
                        match is_adj(rope[i - 1], rope[i]) {
                            // AdjInfo::FourDir => rope[i] = next[i - 1], // only 4 direction
                            AdjInfo::DownSide => rope[i] = (rope[i - 1].0, rope[i - 1].1 - 1),
                            AdjInfo::UpSide => rope[i] = (rope[i - 1].0, rope[i - 1].1 + 1),
                            AdjInfo::LeftSide => rope[i] = (rope[i - 1].0 - 1, rope[i - 1].1),
                            AdjInfo::RightSide => rope[i] = (rope[i - 1].0 + 1, rope[i - 1].1),
                            AdjInfo::Diagonal => rope[i] = diag_move[i - 1],
                            AdjInfo::Adj => {}
                        }
                        if i == 9 {
                            visited.insert(rope[i], true);
                        }
                    }
                    // println!("{:?}", rope);
                }
            }
            _ => {
                // println!("right {}", dist);
                for _ in 0..dist {
                    diag_move = rope.clone();
                    rope.first_mut().unwrap().0 += 1;

                    for i in 1..rope_len {
                        match is_adj(rope[i - 1], rope[i]) {
                            // AdjInfo::FourDir => rope[i] = next[i - 1], // only 4 direction
                            AdjInfo::DownSide => rope[i] = (rope[i - 1].0, rope[i - 1].1 - 1),
                            AdjInfo::UpSide => rope[i] = (rope[i - 1].0, rope[i - 1].1 + 1),
                            AdjInfo::LeftSide => rope[i] = (rope[i - 1].0 - 1, rope[i - 1].1),
                            AdjInfo::RightSide => rope[i] = (rope[i - 1].0 + 1, rope[i - 1].1),
                            AdjInfo::Diagonal => rope[i] = diag_move[i - 1],
                            AdjInfo::Adj => {}
                        }
                        if i == 9 {
                            visited.insert(rope[i], true);
                        }
                    }
                    // println!("{:?}", rope);
                }
            }
        }

        // testcnt += 1;
        // if testcnt == 2 {
        //     break;
        // }
    }
    println!("head pos: {:?}", rope[0]);
    println!("tail pos: {:?}", rope[9]);
    println!("visited: {}", visited.len());
    Ok(())
}

enum AdjInfo {
    Adj,
    DownSide,
    UpSide,
    LeftSide,
    RightSide,
    Diagonal,
}

fn is_adj(head: (i32, i32), tail: (i32, i32)) -> AdjInfo {
    if head.0 - tail.0 <= 1
        && head.0 - tail.0 >= -1
        && head.1 - tail.1 <= 1
        && head.1 - tail.1 >= -1
    {
        // adj
        return AdjInfo::Adj;
    } else if (head.1 - tail.1 == -2 || head.1 - tail.1 == 2)
        && (head.0 - tail.0 == -2 || head.0 - tail.0 == 2)
    {
        return AdjInfo::Diagonal;
    } else if head.0 - tail.0 == 2 {
        // x-1
        return AdjInfo::LeftSide;
    } else if head.0 - tail.0 == -2 {
        // x+1
        return AdjInfo::RightSide;
    } else if head.1 - tail.1 == 2 {
        // y-1
        return AdjInfo::DownSide;
    }
    // y+1
    AdjInfo::UpSide
}
