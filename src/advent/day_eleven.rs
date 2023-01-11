use super::inputs::read_file;
use anyhow::Result;

const ROUND: u16 = 10000;

#[derive(Debug, Clone)]
struct Monkey {
    oper: (Operator, String),
    divisible: usize,
    throw: (usize, usize),
    inspect_num: u32,
}

impl Monkey {
    fn worries(&self, item: usize) -> usize {
        let op_num = match self.oper.1.as_str() {
            "old" => item,
            _ => self.oper.1.parse::<usize>().unwrap(),
        };
        match self.oper.0 {
            Operator::Multiply => item * op_num,
            Operator::Plus => item + op_num,
            Operator::Err => 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Multiply,
    Plus,
    Err,
}

///
/// # Errors
///
/// # Panics
///
#[allow(clippy::too_many_lines)]
pub fn day_eleven() -> Result<()> {
    let lines = read_file("inputs/day_eleven_input.txt")?
        .flatten()
        .peekable();

    let mut monkeys: Vec<Monkey> = Vec::new(); // important
    let mut items_vec: Vec<usize> = Vec::new();
    let mut monkeys_item: Vec<Vec<usize>> = Vec::new();
    let mut operation = (Operator::Multiply, String::new());
    let mut div = 0usize;
    let mut if_true = 0usize;
    let mut if_false = 0usize;

    for line in lines {
        let line = line.trim();
        let cmd: Vec<&str> = line.split(':').map(str::trim).collect();

        if cmd.first().unwrap().starts_with("Starting") {
            items_vec = cmd
                .last()
                .unwrap()
                .split(", ")
                .map(|c| c.parse::<usize>().unwrap())
                .collect();
        }
        if cmd.first().unwrap().starts_with("Operation") {
            let op_line_vec = cmd.last().unwrap().split(' ').collect::<Vec<&str>>();
            operation = match op_line_vec.get(3) {
                Some(&"+") => (Operator::Plus, (*op_line_vec.last().unwrap()).to_string()),
                Some(&"*") => (
                    Operator::Multiply,
                    (*op_line_vec.last().unwrap()).to_string(),
                ),
                _ => (Operator::Err, String::new()),
            }
        }
        if cmd.first().unwrap().starts_with("Test") {
            div = cmd
                .last()
                .unwrap()
                .split(' ')
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();
        }
        if cmd.first().unwrap().starts_with("If true") {
            if_true = cmd
                .last()
                .unwrap()
                .split(' ')
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();
        }
        if cmd.first().unwrap().starts_with("If false") {
            if_false = cmd
                .last()
                .unwrap()
                .split(' ')
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();
        }
        if cmd.first().unwrap().is_empty() {
            let monkey = Monkey {
                oper: operation.clone(),
                divisible: div,
                throw: (if_true, if_false),
                inspect_num: 0,
            };
            println!("{monkey:?}");
            monkeys.push(monkey);
            monkeys_item.push(items_vec.clone());
        }
    }

    let monkey = Monkey {
        // last monkey
        oper: operation,
        divisible: div,
        throw: (if_true, if_false),
        inspect_num: 0,
    };
    println!("{monkey:?}");
    monkeys.push(monkey);
    monkeys_item.push(items_vec);

    // ***************************************************************************************

    println!("***** All Monkey Added *****");
    let monkeys_len = monkeys.len();
    let modulo = monkeys.iter().map(|x| x.divisible).product::<usize>();

    for _ in 1..=ROUND {
        for (i, mk) in monkeys.iter().enumerate().take(monkeys_len) {
            let items = monkeys_item[i].clone();
            for item in items {
                let level = mk.worries(item) % modulo;
                if level % mk.divisible == 0 {
                    let thr = mk.throw.0;
                    monkeys_item[thr].push(level);
                } else {
                    let thr = mk.throw.1;
                    monkeys_item[thr].push(level);
                }

                mk.clone().inspect_num += 1;
            }

            monkeys_item[i].clear();
        }

        // println!("After round {}", round);
        // for i in 0..monkeys_len {
        //     println!("{:?}", monkeys_item[i]);
        // }
        // println!();
    }

    for (i, mk) in monkeys.iter().enumerate().take(monkeys_len) {
        println!(" Monkey {i} inspected items {} times", mk.inspect_num);
    }

    Ok(())
}
