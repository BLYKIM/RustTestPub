// // // // use rand::Rng;
// // // // use std::cmp::Ordering;
// // // // use std::io;

// // // // // fn my_algorithm(number: i32) -> i32 {
// // // // //     number
// // // // // }

// // // // fn main() {
// // // //     let secret_number = rand::thread_rng().gen_range(1..101);

// // // //     //let test_num = 31;

// // // //     loop {
// // // //         println!("Please input your guess.");
// // // //         let mut my_input = String::new();
// // // //         io::stdin()
// // // //             .read_line(&mut my_input)
// // // //             .expect("Failed to read line");

// // // //         //println!("{:?}", my_input);
// // // //         let my_input: u32 = match my_input.trim().parse() {
// // // //             Ok(num) => num,
// // // //             Err(_) => {
// // // //                 println!("Type a number!");
// // // //                 continue;
// // // //             }
// // // //         };

// // // //         println!("You entered: {}", my_input);

// // // //         match my_input.cmp(&secret_number) {
// // // //             Ordering::Less => println!("Too small!"),
// // // //             Ordering::Greater => println!("Too big!"),
// // // //             Ordering::Equal => {
// // // //                 println!("You win!");
// // // //                 break;
// // // //             }
// // // //         }
// // // //     }
// // // //     //println!("{}", my_algorithm(test_num));

// // // //     println!("The secret number is {}", secret_number);
// // // // }

// // // fn first_word(s: &str) -> &str {
// // //     let bytes = s.as_bytes();

// // //     for (i, &item) in bytes.iter().enumerate() {
// // //         if item == b' ' {
// // //             return &s[..i];
// // //         }
// // //     }

// // //     &s[..]
// // // }

// // // fn main() {
// // //     let mut s = String::from("Hello world");

// // //     let word = first_word(&s);

// // //     println!("{}", word);
// // // }

// // #[derive(Debug)]
// // struct User {
// //     active: bool,
// //     username: String,
// //     email: String,
// //     sign_in_count: u64,
// // }

// // fn build_user(email: String, username: String) -> User {
// //     User {
// //         active: true,
// //         username,
// //         email,
// //         sign_in_count: 1,
// //     }
// // }

// // fn main() {
// //     let mut user1 = User {
// //         email: String::from("someone@example.com"),
// //         username: String::from("someusername123"),
// //         active: true,
// //         sign_in_count: 1,
// //     };

// //     user1.email = "anotheremail@example.com".to_string();

// //     println!("{:?}", user1);

// //     let user2 = User {
// //         email: String::from("another@example.com"),
// //         username: String::from("anotherusername567"),
// //         ..user1
// //     };

// //     let user3 = build_user(
// //         String::from("thirdemail@example.com"),
// //         String::from("username012"),
// //     );

// //     println!("{:?}", user2);
// //     println!("{:?}", user3);
// // }

// #[derive(Debug)]
// struct Rectangle {
//     length: u32,
//     width: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.length * self.width
//     }

//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.length > other.length && self.width > other.width
//     }

//     fn square(size: u32) -> Rectangle {
//         Rectangle {
//             length: size,
//             width: size,
//         }
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         length: 50,
//         width: 30,
//     };
//     let rect2 = Rectangle {
//         length: 40,
//         width: 10,
//     };
//     let rect3 = Rectangle {
//         length: 45,
//         width: 60,
//     };

//     let rect4 = Rectangle::square(5);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area()
//     );

//     println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
//     println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
//     println!("Square 4 is {:?}", rect4);
// }

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

fn route(ip_type: IpAddr) {}

fn main() {
    let four = IpAddr::V4;
    let six = IpAddr::V6;

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}
