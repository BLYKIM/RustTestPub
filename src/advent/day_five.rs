use super::inputs::read_file;
use anyhow::Result;

///
/// # Errors
///
/// # Panics
///
/// [T]             [P]     [J]
/// [F]     [S]     [T]     [R]     [B]
/// [V]     [M] [H] [S]     [F]     [R]
/// [Z]     [P] [Q] [B]     [S] [W] [P]
/// [C]     [Q] [R] [D] [Z] [N] [H] [Q]
/// [W] [B] [T] [F] [L] [T] [M] [F] [T]
/// [S] [R] [Z] [V] [G] [R] [Q] [N] [Z]
/// [Q] [Q] [B] [D] [J] [W] [H] [R] [J]
///  0   1   2   3   4   5   6   7   8

pub fn day_five() -> Result<()> {
    let stacks = &mut vec![
        vec!['Q', 'S', 'W', 'C', 'Z', 'V', 'F', 'T'],
        vec!['Q', 'R', 'B'],
        vec!['B', 'Z', 'T', 'Q', 'P', 'M', 'S'],
        vec!['D', 'V', 'F', 'R', 'Q', 'H'],
        vec!['J', 'G', 'L', 'D', 'B', 'S', 'T', 'P'],
        vec!['W', 'R', 'T', 'Z'],
        vec!['H', 'Q', 'M', 'N', 'S', 'F', 'R', 'J'],
        vec!['R', 'N', 'F', 'H', 'W'],
        vec!['J', 'Z', 'T', 'Q', 'P', 'R', 'B'],
    ];
    let lines = read_file("inputs/day_five_input_procedure.txt")?
        .flatten()
        .peekable();

    // move 2 from 1 to 8
    // move 7 from 3 to 2
    for line in lines {
        let line_vec: Vec<&str> = line.split(' ').collect();
        let cnt = line_vec[1].parse::<usize>().unwrap();
        let from = line_vec[3].parse::<usize>().unwrap() - 1;
        let to = line_vec[5].parse::<usize>().unwrap() - 1;
        let mut tmp_vec = Vec::new();

        for _ in 0..cnt {
            let from_crat = stacks.get_mut(from).unwrap();
            let crat = from_crat.pop().unwrap();
            tmp_vec.push(crat);
        }
        for _ in 0..cnt {
            let c = tmp_vec.pop().unwrap();
            stacks.get_mut(to).unwrap().push(c);
        }
    }
    for i in stacks {
        println!("{:?}", i.last());
    }
    Ok(())
}
