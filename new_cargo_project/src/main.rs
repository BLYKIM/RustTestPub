use rand::Rng;
use std::cmp::Ordering;
use std::io;

// fn my_algorithm(number: i32) -> i32 {
//     number
// }

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);

    //let test_num = 31;

    loop {
        println!("Please input your guess.");
        let mut my_input = String::new();
        io::stdin()
            .read_line(&mut my_input)
            .expect("Failed to read line");

        //println!("{:?}", my_input);
        let my_input: u32 = match my_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Type a number!");
                continue;
            }
        };

        println!("You entered: {}", my_input);

        match my_input.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    //println!("{}", my_algorithm(test_num));

    println!("The secret number is {}", secret_number);
}
