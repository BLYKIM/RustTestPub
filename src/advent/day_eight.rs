use super::inputs::read_file;
use anyhow::Result;

///
/// # Errors
///
/// # Panics
///
pub fn day_eight() -> Result<()> {
    let lines = read_file("inputs/day_eight_input.txt")?
        .flatten()
        .peekable();
    let mut line_vec: Vec<Vec<u32>> = Vec::new();
    // let mut visible_tree = 0usize;
    // let mut visible: bool = true;
    let mut highest = (0, 0, 0); // score, x, y
    let mut direction = (0, 0, 0, 0); // left, right, up, down
    for line in lines {
        line_vec.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
    }
    let len = line_vec.len();

    for i in 0..len {
        '_l: for j in 0..len {
            // edges
            if i < 1 || j < 1 || i >= len - 1 || j >= len - 1 {
                // // println!("edge ({},{})", i + 1, j + 1);
                // visible_tree += 1;
                continue;
            }
            let tree_line = line_vec.get(i).unwrap();
            let target_tree = tree_line.get(j).unwrap();

            // part 2
            for left in (0..j).rev() {
                if let Some(left_tree) = tree_line.get(left) {
                    if left_tree >= target_tree {
                        direction.0 = j - left;
                        break;
                    } else if left == 0 {
                        direction.0 = j;
                    }
                }
            }
            for right in (j + 1)..=len {
                if let Some(right_tree) = tree_line.get(right) {
                    if right_tree >= target_tree {
                        direction.1 = right - j;
                        break;
                    } else if right == (len - 1) {
                        direction.1 = len - 1 - j;
                    }
                }
            }
            for up in (0..i).rev() {
                if let Some(line) = line_vec.get(up) {
                    if let Some(up_tree) = line.get(j) {
                        if up_tree >= target_tree {
                            direction.2 = i - up;
                            break;
                        } else if up == 0 {
                            direction.2 = i;
                        }
                    }
                }
            }
            for down in (i + 1)..=len {
                if let Some(line) = line_vec.get(down) {
                    if let Some(down_tree) = line.get(j) {
                        if down_tree >= target_tree {
                            direction.3 = down - i;
                            break;
                        } else if down == (len - 1) {
                            direction.3 = len - 1 - i;
                        }
                    }
                }
            }

            let score = direction.0 * direction.1 * direction.2 * direction.3;
            if highest.0 < score {
                // println!("{:?}", direction);
                highest = (score, i + 1, j + 1);
            }

            //         // part 1
            //         for left in (0..=(j - 1)).rev() {
            //             if let Some(left_tree) = tree_line.get(left) {
            //                 if left_tree >= target_tree {
            //                     visible = false;
            //                     break;
            //                 } else if left == 0 {
            //                     // println!("left ({},{})", i + 1, j + 1);
            //                     visible = true;
            //                     visible_tree += 1;
            //                     continue 'l;
            //                 }
            //             }
            //         }
            //         if !visible {
            //             for right in (j + 1)..=len {
            //                 if let Some(right_tree) = tree_line.get(right) {
            //                     if right_tree >= target_tree {
            //                         visible = false;
            //                         break;
            //                     } else if right == (len - 1) {
            //                         // println!("right ({},{})", i + 1, j + 1);
            //                         visible = true;
            //                         visible_tree += 1;
            //                         continue 'l;
            //                     }
            //                 }
            //             }
            //         }
            //         if !visible {
            //             for up in (0..=(i - 1)).rev() {
            //                 if let Some(line) = line_vec.get(up) {
            //                     if let Some(up_tree) = line.get(j) {
            //                         if up_tree >= target_tree {
            //                             visible = false;
            //                             break;
            //                         } else if up == 0 {
            //                             // println!("up ({},{})", i + 1, j + 1);
            //                             visible = true;
            //                             visible_tree += 1;
            //                             continue 'l;
            //                         }
            //                     }
            //                 }
            //             }
            //         }

            //         if !visible {
            //             for down in (i + 1)..=len {
            //                 if let Some(line) = line_vec.get(down) {
            //                     if let Some(down_tree) = line.get(j) {
            //                         if down_tree >= target_tree {
            //                             visible = false;
            //                             break;
            //                         } else if down == (len - 1) {
            //                             // println!("down ({},{})", i + 1, j + 1);
            //                             visible = true;
            //                             visible_tree += 1;
            //                             continue 'l;
            //                         }
            //                     }
            //                 }
            //             }
            //         }
        }
        // println!("**** {} line clear ****", i + 1);
    }

    println!("highest score: {:?}", highest);
    // println!("visible trees: {}", visible_tree);

    Ok(())
}
