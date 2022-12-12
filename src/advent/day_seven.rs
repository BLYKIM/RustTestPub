use super::inputs::read_file;
use anyhow::Result;

///
/// # Errors
///
/// # Panics
///
pub fn day_seven() -> Result<()> {
    let lines = read_file("inputs/day_seven_input.txt")?
        .flatten()
        .peekable();
    let mut pwd: Vec<(String, usize)> = Vec::new();
    let mut popped: Vec<(String, usize)> = Vec::new();

    for line in lines {
        let line_vec: Vec<&str> = line.split(' ').collect();

        if line_vec.get(0).unwrap().starts_with('$') {
            // println!("command");
            if line_vec.get(1).unwrap().starts_with("cd") {
                if line_vec.get(2).unwrap().starts_with("..") {
                    // last dir pop
                    let t = pwd.pop().unwrap();
                    popped.push(t);
                } else {
                    // cd something
                    let now = line_vec[2].to_string();
                    pwd.push((now.clone(), 0));
                }
            }
        } else if line_vec.get(0).unwrap().starts_with("dir") {
            // dir list
            // let name = line_vec[1].to_string();
            // pwd.insert(name.clone(), 0);
        } else {
            // println!("file");
            let file_size = line_vec.get(0).unwrap().parse::<usize>().unwrap();
            for i in 0..pwd.len() {
                if let Some((_, v)) = pwd.get_mut(i) {
                    *v += file_size;
                }
            }
        }
    }
    println!("re: {:?}", pwd);
    println!("pop: {:?}", popped);

    let mut new = Vec::new();
    for (name, val) in popped {
        if val < 100_000 && val != 0 {
            new.push((name, val))
        }
    }

    println!("result: {:#?}", new);

    let mut sum = 0usize;
    for (_, val) in new {
        sum += val;
    }
    println!("{}", sum);

    Ok(())
}
