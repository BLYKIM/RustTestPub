// // // // // // // // // // // // // // // // // // // // // use std::fmt;

// // // // // // // // // // // // // // // // // // // // // struct Cat {
// // // // // // // // // // // // // // // // // // // // //     name: String,
// // // // // // // // // // // // // // // // // // // // //     age: u8,
// // // // // // // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // // // // // // impl fmt::Display for Cat {
// // // // // // // // // // // // // // // // // // // // //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
// // // // // // // // // // // // // // // // // // // // //         write!(f, "{} is a cat who is {} years old.", self.name, self.age)
// // // // // // // // // // // // // // // // // // // // //     }
// // // // // // // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // // // // // // fn print_cats(pet: String) {
// // // // // // // // // // // // // // // // // // // // //     println!("{}", pet);
// // // // // // // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // // // // // // fn main() {
// // // // // // // // // // // // // // // // // // // // //     let mr_mantle = Cat {
// // // // // // // // // // // // // // // // // // // // //         name: "Reggie Mantle".to_string(),
// // // // // // // // // // // // // // // // // // // // //         age: 4,
// // // // // // // // // // // // // // // // // // // // //     };

// // // // // // // // // // // // // // // // // // // // //     print_cats(mr_mantle.to_string());
// // // // // // // // // // // // // // // // // // // // //     println!(
// // // // // // // // // // // // // // // // // // // // //         "Mr. Mantle's String is {} letters long.",
// // // // // // // // // // // // // // // // // // // // //         mr_mantle.to_string().chars().count()
// // // // // // // // // // // // // // // // // // // // //     );
// // // // // // // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // // // // // struct Monster {
// // // // // // // // // // // // // // // // // // // //     health: i32,
// // // // // // // // // // // // // // // // // // // // }
// // // // // // // // // // // // // // // // // // // // #[derive(Debug)]
// // // // // // // // // // // // // // // // // // // // struct Wizard {
// // // // // // // // // // // // // // // // // // // //     health: i32,
// // // // // // // // // // // // // // // // // // // // }
// // // // // // // // // // // // // // // // // // // // #[derive(Debug)]
// // // // // // // // // // // // // // // // // // // // struct Ranger {
// // // // // // // // // // // // // // // // // // // //     health: i32,
// // // // // // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // // // // // trait FightClose: std::fmt::Debug {
// // // // // // // // // // // // // // // // // // // //     fn attack_with_sword(&self, opponent: &mut Monster) {
// // // // // // // // // // // // // // // // // // // //         opponent.health -= 10;
// // // // // // // // // // // // // // // // // // // //         println!(
// // // // // // // // // // // // // // // // // // // //             "You attack with your sword. Your opponent now has {} health left. You are now at: {:?}",
// // // // // // // // // // // // // // // // // // // //             opponent.health, self
// // // // // // // // // // // // // // // // // // // //         );
// // // // // // // // // // // // // // // // // // // //     }
// // // // // // // // // // // // // // // // // // // //     fn attack_with_hand(&self, opponent: &mut Monster) {
// // // // // // // // // // // // // // // // // // // //         opponent.health -= 2;
// // // // // // // // // // // // // // // // // // // //         println!(
// // // // // // // // // // // // // // // // // // // //             "You attack with your hand. your opponent now has {} health left. You are now at: {:?}",
// // // // // // // // // // // // // // // // // // // //             opponent.health, self
// // // // // // // // // // // // // // // // // // // //         );
// // // // // // // // // // // // // // // // // // // //     }
// // // // // // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // // // // // impl FightClose for Wizard {}
// // // // // // // // // // // // // // // // // // // // impl FightClose for Ranger {}

// // // // // // // // // // // // // // // // // // // // trait FightFromDistance: std::fmt::Debug {
// // // // // // // // // // // // // // // // // // // //     fn attack_with_bow(&self, opponent: &mut Monster, distance: u32) {
// // // // // // // // // // // // // // // // // // // //         if distance < 10 {
// // // // // // // // // // // // // // // // // // // //             opponent.health -= 10;
// // // // // // // // // // // // // // // // // // // //             println!(
// // // // // // // // // // // // // // // // // // // //                 "You attack with your bow. your opponent now has {} health left. You are now at: {:?}",
// // // // // // // // // // // // // // // // // // // //                 opponent.health, self
// // // // // // // // // // // // // // // // // // // //             );
// // // // // // // // // // // // // // // // // // // //         }
// // // // // // // // // // // // // // // // // // // //     }
// // // // // // // // // // // // // // // // // // // //     fn attack_with_rock(&self, opponent: &mut Monster, distance: u32) {
// // // // // // // // // // // // // // // // // // // //         if distance < 3 {
// // // // // // // // // // // // // // // // // // // //             opponent.health -= 4;
// // // // // // // // // // // // // // // // // // // //             println!(
// // // // // // // // // // // // // // // // // // // //                 "You attack with your rock. your opponent now has {} health left. You are now at: {:?}",
// // // // // // // // // // // // // // // // // // // //                 opponent.health, self
// // // // // // // // // // // // // // // // // // // //             );
// // // // // // // // // // // // // // // // // // // //         }
// // // // // // // // // // // // // // // // // // // //     }
// // // // // // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // // // // // impl FightFromDistance for Ranger {}

// // // // // // // // // // // // // // // // // // // // fn main() {
// // // // // // // // // // // // // // // // // // // //     let radagast = Wizard { health: 60 };
// // // // // // // // // // // // // // // // // // // //     let aragorn = Ranger { health: 80 };

// // // // // // // // // // // // // // // // // // // //     let mut uruk_hai = Monster { health: 40 };

// // // // // // // // // // // // // // // // // // // //     radagast.attack_with_sword(&mut uruk_hai);
// // // // // // // // // // // // // // // // // // // //     aragorn.attack_with_bow(&mut uruk_hai, 8);
// // // // // // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // // // // use std::fmt::Debug; // So we don't have to write std::fmt::Debug every time now

// // // // // // // // // // // // // // // // // // // struct Monster {
// // // // // // // // // // // // // // // // // // //     health: i32,
// // // // // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // // // // #[derive(Debug)]
// // // // // // // // // // // // // // // // // // // struct Wizard {
// // // // // // // // // // // // // // // // // // //     health: i32,
// // // // // // // // // // // // // // // // // // // }
// // // // // // // // // // // // // // // // // // // #[derive(Debug)]
// // // // // // // // // // // // // // // // // // // struct Ranger {
// // // // // // // // // // // // // // // // // // //     health: i32,
// // // // // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // // // // trait Magic {} // No methods for any of these traits. They are just trait bounds
// // // // // // // // // // // // // // // // // // // trait FightClose {}
// // // // // // // // // // // // // // // // // // // trait FightFromDistance {}

// // // // // // // // // // // // // // // // // // // impl FightClose for Ranger {} // Each type gets FightClose,
// // // // // // // // // // // // // // // // // // // impl FightClose for Wizard {}
// // // // // // // // // // // // // // // // // // // impl FightFromDistance for Ranger {} // but only Ranger gets FightFromDistance
// // // // // // // // // // // // // // // // // // // impl Magic for Wizard {} // and only Wizard gets Magic

// // // // // // // // // // // // // // // // // // // fn attack_with_bow<T: FightFromDistance + Debug>(
// // // // // // // // // // // // // // // // // // //     character: &T,
// // // // // // // // // // // // // // // // // // //     opponent: &mut Monster,
// // // // // // // // // // // // // // // // // // //     distance: u32,
// // // // // // // // // // // // // // // // // // // ) {
// // // // // // // // // // // // // // // // // // //     if distance < 10 {
// // // // // // // // // // // // // // // // // // //         opponent.health -= 10;
// // // // // // // // // // // // // // // // // // //         println!(
// // // // // // // // // // // // // // // // // // //             "You attack with your bow. Your opponent now has {} health left.  You are now at: {:?}",
// // // // // // // // // // // // // // // // // // //             opponent.health, character
// // // // // // // // // // // // // // // // // // //         );
// // // // // // // // // // // // // // // // // // //     }
// // // // // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // // // // fn attack_with_sword<T: FightClose + Debug>(character: &T, opponent: &mut Monster) {
// // // // // // // // // // // // // // // // // // //     opponent.health -= 10;
// // // // // // // // // // // // // // // // // // //     println!(
// // // // // // // // // // // // // // // // // // //         "You attack with your sword. Your opponent now has {} health left. You are now at: {:?}",
// // // // // // // // // // // // // // // // // // //         opponent.health, character
// // // // // // // // // // // // // // // // // // //     );
// // // // // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // // // // fn fireball<T: Magic + Debug>(character: &T, opponent: &mut Monster, distance: u32) {
// // // // // // // // // // // // // // // // // // //     if distance < 15 {
// // // // // // // // // // // // // // // // // // //         opponent.health -= 20;
// // // // // // // // // // // // // // // // // // //         println!("You raise your hands and cast a fireball! Your opponent now has {} health left. You are now at: {:?}",
// // // // // // // // // // // // // // // // // // //     opponent.health, character);
// // // // // // // // // // // // // // // // // // //     }
// // // // // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // // // // fn main() {
// // // // // // // // // // // // // // // // // // //     let radagast = Wizard { health: 60 };
// // // // // // // // // // // // // // // // // // //     let aragorn = Ranger { health: 80 };

// // // // // // // // // // // // // // // // // // //     let mut uruk_hai = Monster { health: 40 };

// // // // // // // // // // // // // // // // // // //     attack_with_sword(&radagast, &mut uruk_hai);
// // // // // // // // // // // // // // // // // // //     attack_with_bow(&aragorn, &mut uruk_hai, 8);
// // // // // // // // // // // // // // // // // // //     fireball(&radagast, &mut uruk_hai, 8);
// // // // // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // // // use std::fmt::Display;

// // // // // // // // // // // // // // // // // // fn print_vec<T: Display>(input: &Vec<T>) {
// // // // // // // // // // // // // // // // // //     for item in input {
// // // // // // // // // // // // // // // // // //         print!("{} ", item);
// // // // // // // // // // // // // // // // // //     }
// // // // // // // // // // // // // // // // // //     println!();
// // // // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // // // fn main() {
// // // // // // // // // // // // // // // // // //     let array_vec = Vec::from([8, 9, 10]);
// // // // // // // // // // // // // // // // // //     print_vec(&array_vec);

// // // // // // // // // // // // // // // // // //     let str_vec = Vec::from("What kind of vec will I be?");
// // // // // // // // // // // // // // // // // //     print_vec(&str_vec);

// // // // // // // // // // // // // // // // // //     let string_vec = Vec::from("What kind of vec will a String be?".to_string());
// // // // // // // // // // // // // // // // // //     print_vec(&string_vec);
// // // // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // // #[derive(Debug)]
// // // // // // // // // // // // // // // // // struct City {
// // // // // // // // // // // // // // // // //     name: String,
// // // // // // // // // // // // // // // // //     population: u32,
// // // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // // #[derive(Debug)]
// // // // // // // // // // // // // // // // // struct Country {
// // // // // // // // // // // // // // // // //     cities: Vec<City>,
// // // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // // impl City {
// // // // // // // // // // // // // // // // //     fn new(name: &str, population: u32) -> Self {
// // // // // // // // // // // // // // // // //         Self {
// // // // // // // // // // // // // // // // //             name: name.to_string(),
// // // // // // // // // // // // // // // // //             population,
// // // // // // // // // // // // // // // // //         }
// // // // // // // // // // // // // // // // //     }
// // // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // // impl From<Vec<City>> for Country {
// // // // // // // // // // // // // // // // //     fn from(cities: Vec<City>) -> Self {
// // // // // // // // // // // // // // // // //         Self { cities }
// // // // // // // // // // // // // // // // //     }
// // // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // // impl Country {
// // // // // // // // // // // // // // // // //     fn print_cities(&self) {
// // // // // // // // // // // // // // // // //         for city in &self.cities {
// // // // // // // // // // // // // // // // //             println!("{:?} has a population of {:?}.", city.name, city.population);
// // // // // // // // // // // // // // // // //         }
// // // // // // // // // // // // // // // // //     }
// // // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // // fn main() {
// // // // // // // // // // // // // // // // //     let helsinki = City::new("Helsinki", 631_695);
// // // // // // // // // // // // // // // // //     let turku = City::new("Turku", 186_756);

// // // // // // // // // // // // // // // // //     let finland_cities = vec![helsinki, turku]; // Vec<City>
// // // // // // // // // // // // // // // // //     let finland = Country::from(finland_cities); // Country::from

// // // // // // // // // // // // // // // // //     finland.print_cities();
// // // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // use std::convert::From;

// // // // // // // // // // // // // // // // struct EvenOddVec(Vec<Vec<i32>>);

// // // // // // // // // // // // // // // // impl From<Vec<i32>> for EvenOddVec {
// // // // // // // // // // // // // // // //     fn from(input: Vec<i32>) -> Self {
// // // // // // // // // // // // // // // //         let mut even_odd_vec: Vec<Vec<i32>> = vec![vec![], vec![]]; // vec with two empty vecs

// // // // // // // // // // // // // // // //         for item in input {
// // // // // // // // // // // // // // // //             if item % 2 == 0 {
// // // // // // // // // // // // // // // //                 even_odd_vec[0].push(item);
// // // // // // // // // // // // // // // //             } else {
// // // // // // // // // // // // // // // //                 even_odd_vec[1].push(item);
// // // // // // // // // // // // // // // //             }
// // // // // // // // // // // // // // // //         }
// // // // // // // // // // // // // // // //         Self(even_odd_vec) //return as Self(EvenOddVec)
// // // // // // // // // // // // // // // //     }
// // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // fn main() {
// // // // // // // // // // // // // // // //     let bunch_of_numbers = vec![8, 7, -1, 222, 9787, -47, 77, 0, 55, 7, 8];
// // // // // // // // // // // // // // // //     let new_vec = EvenOddVec::from(bunch_of_numbers);

// // // // // // // // // // // // // // // //     println!(
// // // // // // // // // // // // // // // //         "Even numbers: {:?}\nOddnumbers: {:?}",
// // // // // // // // // // // // // // // //         new_vec.0[0], new_vec.0[1]
// // // // // // // // // // // // // // // //     );
// // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // use std::fmt::{Debug, Display};

// // // // // // // // // // // // // // // fn print_it<T>(input: T)
// // // // // // // // // // // // // // // where
// // // // // // // // // // // // // // //     T: AsRef<str> + Display + Debug,
// // // // // // // // // // // // // // // {
// // // // // // // // // // // // // // //     println!("{}", input);
// // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // fn main() {
// // // // // // // // // // // // // // //     print_it("Please print me");
// // // // // // // // // // // // // // //     print_it("Also, print me".to_string());
// // // // // // // // // // // // // // //     //print_it(8); // asref 때문에 안됨
// // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // fn main() {
// // // // // // // // // // // // // //     // let new_vec = (1..=10).collect::<Vec<i32>>();
// // // // // // // // // // // // // //     // let new_vec: Vec<i32> = (1..=10).collect();
// // // // // // // // // // // // // //     let my_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
// // // // // // // // // // // // // //     let new_vec = my_vec.into_iter().skip(3).take(4).collect::<Vec<i32>>();
// // // // // // // // // // // // // //     println!("{:?}", new_vec);
// // // // // // // // // // // // // // }

// // // // // // // // // // // // // #[derive(Debug, Clone)]
// // // // // // // // // // // // // struct Library {
// // // // // // // // // // // // //     library_type: LibraryType,
// // // // // // // // // // // // //     books: Vec<String>,
// // // // // // // // // // // // // }

// // // // // // // // // // // // // #[derive(Debug, Clone)]
// // // // // // // // // // // // // enum LibraryType {
// // // // // // // // // // // // //     City,
// // // // // // // // // // // // //     Country,
// // // // // // // // // // // // // }

// // // // // // // // // // // // // impl Library {
// // // // // // // // // // // // //     fn add_book(&mut self, book: &str) {
// // // // // // // // // // // // //         self.books.push(book.to_string());
// // // // // // // // // // // // //     }

// // // // // // // // // // // // //     fn new() -> Self {
// // // // // // // // // // // // //         Self {
// // // // // // // // // // // // //             library_type: LibraryType::City,
// // // // // // // // // // // // //             books: Vec::new(),
// // // // // // // // // // // // //         }
// // // // // // // // // // // // //     }
// // // // // // // // // // // // // }

// // // // // // // // // // // // // impl Iterator for Library {
// // // // // // // // // // // // //     type Item = String;

// // // // // // // // // // // // //     fn next(&mut self) -> Option<String> {
// // // // // // // // // // // // //         match self.books.pop() {
// // // // // // // // // // // // //             Some(book) => Some(book + " is found!"),
// // // // // // // // // // // // //             None => None,
// // // // // // // // // // // // //         }
// // // // // // // // // // // // //     }
// // // // // // // // // // // // // }

// // // // // // // // // // // // // fn main() {
// // // // // // // // // // // // //     let mut my_library = Library::new();
// // // // // // // // // // // // //     my_library.add_book("The doom of the Darksword");
// // // // // // // // // // // // //     my_library.add_book("Demian");
// // // // // // // // // // // // //     my_library.add_book("구운몽");
// // // // // // // // // // // // //     my_library.add_book("dlstod");

// // // // // // // // // // // // //     for item in my_library.clone() {
// // // // // // // // // // // // //         println!("{}", item);
// // // // // // // // // // // // //     }
// // // // // // // // // // // // // }

// // // // // // // // // // // // //lambdas

// // // // // // // // // // // // fn main() {
// // // // // // // // // // // //     let num_vec = vec![2, 3, 4];

// // // // // // // // // // // //     num_vec
// // // // // // // // // // // //         .iter()
// // // // // // // // // // // //         .enumerate()
// // // // // // // // // // // //         .for_each(|(index, number)| println!("Index number {} has number {}.", index, number));
// // // // // // // // // // // // }

// // // // // // // // // // // use std::collections::HashMap;

// // // // // // // // // // // fn main() {
// // // // // // // // // // //     let some_numbers = vec![0, 1, 2, 3, 4, 5]; //Vec<i32>
// // // // // // // // // // //     let some_words = vec!["zero", "one", "two", "three", "four", "five"]; // Vec<&str>
// // // // // // // // // // //     let number_word_hashmap: HashMap<_, _> = some_numbers
// // // // // // // // // // //         .into_iter()
// // // // // // // // // // //         .zip(some_words.into_iter())
// // // // // // // // // // //         .collect();

// // // // // // // // // // //     println!(
// // // // // // // // // // //         "For key {} we get {}.",
// // // // // // // // // // //         2,
// // // // // // // // // // //         number_word_hashmap.get(&2).unwrap()
// // // // // // // // // // //     );
// // // // // // // // // // // }

// // // // // // // // // // fn main() {
// // // // // // // // // //     let numbers_together = "140399923481800622623218009598281";

// // // // // // // // // //     for (index, number) in numbers_together.char_indices() {
// // // // // // // // // //         match (index % 3, number) {
// // // // // // // // // //             (0..=1, number) => print!("{}", number),
// // // // // // // // // //             _ => print!("{}\t", number),
// // // // // // // // // //         }
// // // // // // // // // //     }
// // // // // // // // // //     println!();
// // // // // // // // // // }

// // // // // // // // // fn main() {
// // // // // // // // //     let my_vec = vec![8, 9, 10];

// // // // // // // // //     println!(
// // // // // // // // //         "{:?}",
// // // // // // // // //         my_vec
// // // // // // // // //             .iter()
// // // // // // // // //             .for_each(|_| println!("We didn't use the variables at all"))
// // // // // // // // //     );
// // // // // // // // // }

// // // // // // // // fn main() {
// // // // // // // //     let user_input = vec![
// // // // // // // //         "8.9",
// // // // // // // //         "Nine point nine five",
// // // // // // // //         "8.0",
// // // // // // // //         "7.6",
// // // // // // // //         "eleventy_twelve",
// // // // // // // //         "300",
// // // // // // // //     ];

// // // // // // // //     let actual_numbers = user_input
// // // // // // // //         .into_iter()
// // // // // // // //         .filter_map(|input| input.parse::<f32>().ok())
// // // // // // // //         .collect::<Vec<f32>>();

// // // // // // // //     println!("{:?}", actual_numbers);
// // // // // // // // }

// // // // // // // fn main() {
// // // // // // //     let num_vec = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

// // // // // // //     println!("{:?}", num_vec.iter().find(|&number| number % 3 == 0)); // find takes a reference, so we give it &number
// // // // // // //     println!("{:?}", num_vec.iter().find(|&number| number * 2 == 30));

// // // // // // //     println!("{:?}", num_vec.iter().position(|&number| number % 3 == 0));
// // // // // // //     println!("{:?}", num_vec.iter().position(|&number| number * 2 == 30));
// // // // // // // }

// // // // // // #[derive(Debug)]
// // // // // // struct Names {
// // // // // //     one_word: Vec<String>,
// // // // // //     two_words: Vec<String>,
// // // // // //     three_words: Vec<String>,
// // // // // // }

// // // // // // fn main() {
// // // // // //     let vec_of_names = vec![
// // // // // //         "Caesar",
// // // // // //         "Frodo Baggins",
// // // // // //         "Bilbo Baggins",
// // // // // //         "Jean-Luc Picard",
// // // // // //         "Bly Kim",
// // // // // //         "Data",
// // // // // //         "Rand AI 'Thor",
// // // // // //         "Paul Atreides",
// // // // // //         "Barack Hussein Obama",
// // // // // //         "Bill Jefferson Clinton",
// // // // // //     ];

// // // // // //     let mut iter_of_names = vec_of_names.iter().peekable();

// // // // // //     let mut all_names = Names {
// // // // // //         one_word: vec![],
// // // // // //         two_words: vec![],
// // // // // //         three_words: vec![],
// // // // // //     };

// // // // // //     while iter_of_names.peek().is_some() {
// // // // // //         let next_item = iter_of_names.next().unwrap();
// // // // // //         match next_item.match_indices(' ').collect::<Vec<_>>().len() {
// // // // // //             0 => all_names.one_word.push(next_item.to_string()),
// // // // // //             1 => all_names.two_words.push(next_item.to_string()),
// // // // // //             _ => all_names.three_words.push(next_item.to_string()),
// // // // // //         }
// // // // // //     }

// // // // // //     println!("{:?}", all_names);
// // // // // // }

// // // // // fn main() {
// // // // //     let new_vec = vec![8, 9, 10];

// // // // //     let double_vec = new_vec
// // // // //         .iter()
// // // // //         .inspect(|first_item| println!("The item is: {}", first_item))
// // // // //         .map(|x| x * 2)
// // // // //         .inspect(|next_item| println!("Then it is: {}", next_item))
// // // // //         .collect::<Vec<i32>>();
// // // // // }

// // // // // //Lifetimes can be difficult in Rust, but here are some tips to avoid getting too stressed about them:
// // // // // // You can stay with owned types, use clones etc. if you want to avoid them for the time being.
// // // // // // Much of the time, when the compiler wants a lifetime you will just end up writing <'a> here and there and then it will work. It's just a way of saying "don't worry, I won't give you anything that doesn't live long enough".
// // // // // // You can explore lifetimes just a bit at a time. Write some code with owned values, then make one a reference. The compiler will start to complain, but also give some suggestions. And if it gets too complicated, you can undo it and try again next time.

// // // // // multiple threads

// // // // fn main() {
// // // //     let mut my_string = String::from("Can I go inside the thread?");

// // // //     let handle = std::thread::spawn(move || {
// // // //         println!("{}", my_string);
// // // //     });

// // // //     handle.join().unwrap();
// // // // }

// // // #[derive(Debug)]
// // // struct City {
// // //     name: String,
// // //     years: Vec<u32>,
// // //     populations: Vec<u32>,
// // // }

// // // impl City {
// // //     fn new(name: &str, years: Vec<u32>, populations: Vec<u32>) -> Self {
// // //         Self {
// // //             name: name.to_string(),
// // //             years,
// // //             populations,
// // //         }
// // //     }

// // //     fn city_data<F>(&mut self, mut f: F)
// // //     //f is generic F (closure)
// // //     where
// // //         F: FnMut(&mut Vec<u32>, &mut Vec<u32>), //closure takes mut vec u32 (year populations)
// // //     {
// // //         f(&mut self.years, &mut self.populations) // this is actual function,
// // //     }
// // // }

// // // fn main() {
// // //     let years = vec![
// // //         1372, 1834, 1851, 1881, 1897, 1925, 1959, 1989, 2000, 2005, 2010, 2020,
// // //     ];
// // //     let populations = vec![
// // //         3_250, 15_300, 24_000, 45_900, 58_800, 119_800, 283_071, 478_974, 400_378, 401_694,
// // //         406_703, 437_619,
// // //     ];

// // //     let mut tallinn = City::new("Tallinn", years, populations);

// // //     tallinn.city_data(|city_years, city_populations| { //5년 zip print
// // //         let new_vec = city_years
// // //             .into_iter()
// // //             .zip(city_populations.into_iter())
// // //             .take(5)
// // //             .collect::<Vec<(_, _)>>();
// // //         println!("{:?}", new_vec);
// // //     });

// // //     tallinn.city_data(|x, y| { //2030,500_000 추가
// // //         x.push(2030);
// // //         y.push(500_000);
// // //     });

// // //     tallinn.city_data(|x, y| { //1834 data 삭제
// // //         let position_option = x.iter().position(|x| *x == 1834);
// // //         if let Some(position) = position_option {
// // //             println!(
// // //                 "Going to delete {} at position {:?} now.",
// // //                 x[position], position
// // //             );
// // //             x.remove(position);
// // //             y.remove(position);
// // //         }
// // //     });

// // //     println!(
// // //         "Years left are {:?}\nPopulations left are {:?}",
// // //         tallinn.years, tallinn.populations
// // //     );
// // // }

// // //return closure

// // fn returns_a_closure(input: &str) -> impl FnMut(i32) -> i32 {
// //     match input {
// //         "double" => |mut number| {
// //             number *= 2;
// //             println!("Doubling number. Now it is {}", number);
// //             number
// //         },
// //         "triple" => |mut number| {
// //             number *= 3;
// //             println!("Tripling number. Now it its {}", number);
// //             number
// //         },
// //         _ => |number| {
// //             println!("Sorry, it's the same number: {}", number);
// //             number
// //         },
// //     }
// // }

// // fn main() {
// //     let my_number = 10;

// //     let mut doubles = returns_a_closure("double");
// //     let mut triples = returns_a_closure("triple");
// //     let mut quadruples = returns_a_closure("quadruple");

// //     doubles(my_number);
// //     triples(my_number);
// //     quadruples(my_number);
// // }

// enum TimeOfDay {
//     Dawn,
//     Day,
//     Sunset,
//     Night,
// }

// fn change_fear(input: TimeOfDay) -> impl FnMut(f64) -> f64 {
//     use TimeOfDay::*;

//     match input {
//         Dawn => |x| {
//             println!(
//                 "The morning sun has vanquished the horrible night. You no longer feel afraid."
//             );
//             println!("Your fear is now {}", x * 0.5);
//             x * 0.5
//         },
//         Day => |x| {
//             println!("What a nice day. Maybe put your feet up and rest a bit.");
//             println!("Your fear is now {}", x * 0.2);
//             x * 0.2
//         },
//         Sunset => |x| {
//             println!("The sun is almost down! This is no good.");
//             println!("Your fear is now {}", x * 1.4);
//             x * 1.4
//         },
//         Night => |x| {
//             println!("What a horrible night to have a curse.");
//             println!("Your fear is now {}", x * 5.0);
//             x * 5.0
//         },
//     }
// }

// fn main() {
//     use TimeOfDay::*;
//     let mut character_fear = 10.0;

//     let mut daytime = change_fear(Day);
//     let mut sunset = change_fear(Sunset);
//     let mut night = change_fear(Night);
//     let mut morning = change_fear(Dawn);

//     character_fear = daytime(character_fear);
//     character_fear = sunset(character_fear);
//     character_fear = night(character_fear);
//     character_fear = morning(character_fear);
// }

use std::sync::{Arc, Mutex};
use std::thread::spawn;

fn make_arc(number: i32) -> Arc<Mutex<i32>> {
    Arc::new(Mutex::new(number))
}

fn new_clone(input: &Arc<Mutex<i32>>) -> Arc<Mutex<i32>> {
    Arc::clone(&input)
}

fn main() {
    let mut handle_vec = vec![];
    let my_number = make_arc(0);

    for _ in 0..2 {
        let my_number_clone = new_clone(&my_number);
        let handle = spawn(move || {
            for _ in 0..10 {
                let mut value_inside = my_number_clone.lock().unwrap();
                *value_inside += 1;
            }
        });
        handle_vec.push(handle);
    }

    handle_vec
        .into_iter()
        .for_each(|handle| handle.join().unwrap());

    println!("{:?}", my_number);
}
//alalalalalal
