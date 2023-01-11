use std::collections::HashMap;

use super::inputs::read_file;
use anyhow::Result;

///
/// # Errors
///
/// # Panics
///
#[allow(clippy::too_many_lines)]
pub fn day_twelve() -> Result<()> {
    let mut lines = read_file("inputs/day_twelve_input.txt")?
        .flatten()
        .peekable();
    let mut route: Vec<(usize, usize)> = Vec::new();
    let mut now = (0, 0);
    let mut end = (0, 0);

    let mut map: HashMap<(usize, usize), (u32, bool)> = HashMap::new(); // key: (x,y), value: (elev, visited)
    let width = lines.peek().unwrap().len();
    let mut height = 0usize;
    let (mut x, mut y) = (1usize, 1usize);

    // parse map
    for line in lines {
        height += 1;

        for elev in line.chars() {
            if elev == 'S' {
                now = (x, y);
                map.insert(now, ('a' as u32, true));
                route.push(now);
            } else if elev == 'E' {
                end = (x, y);
                map.insert(end, ('z' as u32, false));
            } else {
                map.insert((x, y), (elev as u32, false));
            }
            x += 1;
        }
        x = 1;
        y += 1;
    }

    // ***************************************** Map Generated *****************
    println!("-- Map: {width}x{height}");
    println!(
        "-- Start: {now:?}, {:?}\n-- End: {end:?}, {:?}",
        map.get(&now),
        map.get(&end)
    );
    // println!("{:?}", route);

    // let mut cnt = 0;
    loop {
        let next = map.get(&now).unwrap().0 + 1;
        match now {
            // point
            (1, 1) => {
                // right, down
                match search_route(
                    next,
                    None,
                    Some((now.0, now.1 + 1)),
                    None,
                    Some((now.0 + 1, now.1)),
                    &map,
                ) {
                    Direction::Down => {
                        now = (now.0, now.1 + 1);
                        route.push(now);
                    }
                    Direction::Right => {
                        now = (now.0 + 1, now.1);
                        route.push(now);
                    }
                    Direction::Before => {
                        route.pop();
                        now = *route.last().unwrap();
                    }
                    _ => {}
                }
            }
            v if v == (1, height) => {
                // right, up
                match search_route(
                    next,
                    Some((now.0, now.1 - 1)),
                    None,
                    None,
                    Some((now.0 + 1, now.1)),
                    &map,
                ) {
                    Direction::Up => {
                        now = (now.0, now.1 - 1);
                        route.push(now);
                    }
                    Direction::Right => {
                        now = (now.0 + 1, now.1);
                        route.push(now);
                    }
                    Direction::Before => {
                        route.pop();
                        now = *route.last().unwrap();
                    }
                    _ => {}
                }
            }
            v if v == (width, 1) => {
                // left, down
                match search_route(
                    next,
                    None,
                    Some((now.0, now.1 + 1)),
                    Some((now.0 - 1, now.1)),
                    None,
                    &map,
                ) {
                    Direction::Down => {
                        now = (now.0, now.1 + 1);
                        route.push(now);
                    }
                    Direction::Left => {
                        now = (now.0 - 1, now.1);
                        route.push(now);
                    }
                    Direction::Before => {
                        route.pop();
                        now = *route.last().unwrap();
                    }
                    _ => {}
                }
            }
            v if v == (width, height) => {
                // left, up
                match search_route(
                    next,
                    Some((now.0, now.1 - 1)),
                    None,
                    Some((now.0 - 1, now.1)),
                    None,
                    &map,
                ) {
                    Direction::Up => {
                        now = (now.0, now.1 - 1);
                        route.push(now);
                    }
                    Direction::Left => {
                        now = (now.0 - 1, now.1);
                        route.push(now);
                    }
                    Direction::Before => {
                        route.pop();
                        now = *route.last().unwrap();
                    }
                    _ => {}
                }
            }
            // edge
            (1, _) => {
                // no left
                match search_route(
                    next,
                    Some((now.0, now.1 - 1)),
                    Some((now.0, now.1 + 1)),
                    None,
                    Some((now.0 + 1, now.1)),
                    &map,
                ) {
                    Direction::Up => {
                        now = (now.0, now.1 - 1);
                        route.push(now);
                    }
                    Direction::Down => {
                        now = (now.0, now.1 + 1);
                        route.push(now);
                    }
                    Direction::Right => {
                        now = (now.0 + 1, now.1);
                        route.push(now);
                    }
                    Direction::Before => {
                        route.pop();
                        now = *route.last().unwrap();
                    }
                    Direction::Left => {}
                }
            }
            (v, _) if v == width => {
                // no right
                match search_route(
                    next,
                    Some((now.0, now.1 - 1)),
                    Some((now.0, now.1 + 1)),
                    Some((now.0 - 1, now.1)),
                    None,
                    &map,
                ) {
                    Direction::Up => {
                        now = (now.0, now.1 - 1);
                        route.push(now);
                    }
                    Direction::Down => {
                        now = (now.0, now.1 + 1);
                        route.push(now);
                    }
                    Direction::Left => {
                        now = (now.0 - 1, now.1);
                        route.push(now);
                    }
                    Direction::Before => {
                        route.pop();
                        now = *route.last().unwrap();
                    }
                    Direction::Right => {}
                }
            }
            (_, 1) => {
                // no up
                match search_route(
                    next,
                    None,
                    Some((now.0, now.1 + 1)),
                    Some((now.0 - 1, now.1)),
                    Some((now.0 + 1, now.1)),
                    &map,
                ) {
                    Direction::Down => {
                        now = (now.0, now.1 + 1);
                        route.push(now);
                    }
                    Direction::Left => {
                        now = (now.0 - 1, now.1);
                        route.push(now);
                    }
                    Direction::Right => {
                        now = (now.0 + 1, now.1);
                        route.push(now);
                    }
                    Direction::Before => {
                        route.pop();
                        now = *route.last().unwrap();
                    }
                    Direction::Up => {}
                }
            }
            (_, v) if v == height => {
                // no down
                match search_route(
                    next,
                    Some((now.0, now.1 - 1)),
                    None,
                    Some((now.0 - 1, now.1)),
                    Some((now.0 + 1, now.1)),
                    &map,
                ) {
                    Direction::Up => {
                        now = (now.0, now.1 - 1);
                        route.push(now);
                    }
                    Direction::Left => {
                        now = (now.0 - 1, now.1);
                        route.push(now);
                    }
                    Direction::Right => {
                        now = (now.0 + 1, now.1);
                        route.push(now);
                    }
                    Direction::Before => {
                        route.pop();
                        now = *route.last().unwrap();
                    }
                    Direction::Down => {}
                }
            }
            // inside
            _ => {
                match search_route(
                    next,
                    Some((now.0, now.1 - 1)),
                    Some((now.0, now.1 + 1)),
                    Some((now.0 - 1, now.1)),
                    Some((now.0 + 1, now.1)),
                    &map,
                ) {
                    Direction::Up => {
                        now = (now.0, now.1 - 1);
                        route.push(now);
                    }
                    Direction::Down => {
                        now = (now.0, now.1 + 1);
                        route.push(now);
                    }
                    Direction::Left => {
                        now = (now.0 - 1, now.1);
                        route.push(now);
                    }
                    Direction::Right => {
                        now = (now.0 + 1, now.1);
                        route.push(now);
                    }
                    Direction::Before => {
                        route.pop();
                        now = *route.last().unwrap();
                    }
                }
            }
        }
        if let Some((_, b)) = map.get_mut(&now) {
            *b = true;
        }

        if now == end {
            break;
        }
        // cnt += 1;
        // println!("{:?}", route);
    }

    println!("your steps: {}", route.len() - 1);

    Ok(())
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
    Before,
}

fn search_route(
    mut next: u32,
    up: Option<(usize, usize)>,
    down: Option<(usize, usize)>,
    left: Option<(usize, usize)>,
    right: Option<(usize, usize)>,
    map: &HashMap<(usize, usize), (u32, bool)>,
) -> Direction {
    for _ in 0..3 {
        if let Some(go_upside) = up {
            if map.get(&go_upside).unwrap() == &(next, false) {
                return Direction::Up;
            }
        }
        if let Some(go_right) = right {
            if map.get(&go_right).unwrap() == &(next, false) {
                return Direction::Right;
            }
        }
        if let Some(go_down) = down {
            if map.get(&go_down).unwrap() == &(next, false) {
                return Direction::Down;
            }
        }
        if let Some(go_left) = left {
            if map.get(&go_left).unwrap() == &(next, false) {
                return Direction::Left;
            }
        }
        next -= 1;
        // println!("next: {}", next); // 중복일때 가중치 필요
    }
    Direction::Before
}
