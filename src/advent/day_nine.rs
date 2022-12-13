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
    let mut head = (0, 0);
    let mut tail = head;
    let mut visited: HashMap<(i32, i32), bool> = HashMap::new();
    visited.insert(tail, true);
    // let mut testcnt = 0;
    for line in lines {
        let step: Vec<&str> = line.split(' ').collect();
        let dist = step[1].parse::<u32>()?;
        let mut head_before = head;

        match step[0] {
            "U" => {
                // println!("up {}", dist);
                for _ in 0..dist {
                    head_before = head;
                    head.1 += 1;

                    if !is_adj(head, tail) {
                        tail = head_before;
                        visited.insert(tail, true);
                    }
                }
            }
            "D" => {
                // println!("down {}", dist);
                for _ in 0..dist {
                    head_before = head;
                    head.1 -= 1;

                    if !is_adj(head, tail) {
                        tail = head_before;
                        visited.insert(tail, true);
                    }
                }
            }
            "L" => {
                // println!("left {}", dist);
                for _ in 0..dist {
                    head_before = head;
                    head.0 -= 1;

                    if !is_adj(head, tail) {
                        tail = head_before;
                        visited.insert(tail, true);
                    }
                }
            }
            _ => {
                // println!("right {}", dist);
                for _ in 0..dist {
                    head_before = head;
                    head.0 += 1;

                    if !is_adj(head, tail) {
                        tail = head_before;
                        visited.insert(tail, true);
                    }
                }
            }
        }
        // testcnt += 1;
        // if testcnt == 10 {
        //     break;
        // }
    }
    println!("head pos: {:?}", head);
    println!("tail pos: {:?}", tail);
    println!("visited: {}", visited.len());
    Ok(())
}

fn is_adj(head: (i32, i32), tail: (i32, i32)) -> bool {
    // println!("{:?}, {:?}", head, tail);
    if head.0 - tail.0 <= 1 && head.0 - tail.0 > -1 && head.1 - tail.1 <= 1 && head.1 - tail.1 >= -1
    {
        // adj
        return true;
    }
    // tail move
    false
}
